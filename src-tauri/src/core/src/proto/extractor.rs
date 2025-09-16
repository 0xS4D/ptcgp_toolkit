use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::{Result, anyhow};
use phf::phf_map;

use crate::{
    proto::{
        ProtoType,
        field::{ProtoCardinality, ProtoField},
        map::ProtoMapField,
        message::ProtoMessage,
        one_of::ProtoOneOf,
        package::ProtoPackage,
        proto_enum::ProtoEnum,
        schema::{ProtoGenUnit, ProtoSchema},
        service::{ProtoService, ProtoServiceMethod},
    },
    unity::{
        generated::il2cpp_2022333f1::root::{
            Il2CppImageDefinition, Il2CppTypeDefinition, TypeIndex,
        },
        utils::{complex_type::ComplexType, il2cpp::Il2Cpp},
    },
};

pub fn generate_proto_schema(il2cpp: Il2Cpp) -> Result<Vec<ProtoGenUnit>> {
    let mut schema = ProtoSchema::new();

    let mut nested_types_map: HashMap<TypeIndex, Vec<ProtoType>> = HashMap::new();
    let mut oneof_cases: HashMap<TypeIndex, Vec<ProtoEnum>> = HashMap::new();

    for game_image in &il2cpp.metadata.images {
        process_image(
            game_image,
            &il2cpp,
            &mut schema,
            &mut nested_types_map,
            &mut oneof_cases,
        )?;
    }

    process_nested_types(&il2cpp, &mut nested_types_map)?;
    integrate_nested_types_into_packages(&mut schema, &mut nested_types_map);

    schema.seal();
    schema.build_units()
}

static NET_TO_PROTO: phf::Map<&'static str, &'static str> = phf_map! {
    "int" => "int32",
    "Int32" => "int32",
    "long" => "int64",
    "Int64" => "int64",
    "ulong" => "uint64",
    "UInt64" => "fixed64",
    "uint" => "fixed32",
    "UInt32" => "fixed32",
    "Single" => "float",
    "Boolean" => "bool",
    "Double" => "double",
    "String" => "string",
    "ByteString" => "bytes",
};

fn process_image<'a>(
    game_image: &Il2CppImageDefinition,
    il2cpp: &'a Il2Cpp<'a>,
    schema: &mut ProtoSchema,
    nested_types_map: &mut HashMap<TypeIndex, Vec<ProtoType>>,
    oneof_cases: &mut HashMap<TypeIndex, Vec<ProtoEnum>>,
) -> Result<()> {
    // let image_name = il2cpp.metadata.get_string_by_index(game_image.nameIndex);

    let type_start = game_image.typeStart as usize;
    let type_end = type_start + game_image.typeCount as usize;
    let type_defs = &il2cpp.metadata.type_definitions[type_start..type_end];

    for ty_def in type_defs {
        let namespace = il2cpp.metadata.get_string_by_index(ty_def.namespaceIndex);
        let package = schema.get(namespace);

        if ty_def.has_field(il2cpp, "__ServiceName", "string") {
            process_service(ty_def, il2cpp, package)?;
        } else if ty_def.is_enum_type() {
            process_enum(ty_def, il2cpp, package, nested_types_map, oneof_cases)?;
        } else if ty_def.has_field(il2cpp, "_parser", "MessageParser") {
            process_message(ty_def, il2cpp, package, nested_types_map, oneof_cases)?;
        }
    }
    Ok(())
}

fn process_service<'a>(
    ty_def: &Il2CppTypeDefinition,
    il2cpp: &'a Il2Cpp<'a>,
    package: &mut ProtoPackage,
) -> Result<()> {
    let metadata = &il2cpp.metadata;
    let service_name = metadata.get_string_by_index(ty_def.nameIndex);
    let expected_client_name = format!("{}Client", service_name);

    let service_client_ty_def = metadata
        .type_definitions
        .iter()
        .find(|td| {
            td.declaringTypeIndex == ty_def.byvalTypeIndex
                && metadata.get_string_by_index(td.nameIndex) == expected_client_name
        })
        .ok_or_else(|| anyhow!("Could not find client type for service {}", service_name))?;

    let mut service = ProtoService::new(service_name.clone(), ty_def.byvalTypeIndex);

    for field_idx in ty_def.field_indices() {
        let field = &metadata.fields[field_idx];
        let field_name = metadata.get_string_by_index(field.nameIndex);

        if !field_name.starts_with("__Method_") {
            continue;
        }

        let rpc_name = field_name.trim_start_matches("__Method_");

        let method_field_type = &il2cpp.types[field.typeIndex as usize];
        match method_field_type.get_complex_type(il2cpp)? {
            ComplexType::Generic { ref base, ref args } => {
                if base.to_string() != "Method" {
                    println!("Unexpected base type in service method: {}", base);
                }
                if args.args.len() < 2 {
                    println!("Service method field does not have two type arguments");
                }

                let request_type = &args.args[0];
                let response_type = &args.args[1];

                let (client_streaming, server_streaming) =
                    get_rpc_streaming_info(service_client_ty_def, rpc_name, il2cpp)?;

                let rpc_method = ProtoServiceMethod::new(
                    rpc_name.to_string(),
                    request_type.get_root_namespace().map(String::from),
                    request_type.get_name_str(true)?,
                    request_type.get_type_index(),
                    response_type.get_root_namespace().map(String::from),
                    response_type.get_name_str(true)?,
                    response_type.get_type_index(),
                    client_streaming,
                    server_streaming,
                );

                service.add_method(rpc_method);
            }
            other => {
                println!("Unexpected method field type: {:?}", other);
            }
        }
    }
    package.add_service(service);
    Ok(())
}

fn get_rpc_streaming_info<'a>(
    client_ty_def: &Il2CppTypeDefinition,
    rpc_name: &str,
    il2cpp: &'a Il2Cpp<'a>,
) -> Result<(bool, bool)> {
    let metadata = &il2cpp.metadata;
    let start = client_ty_def.methodStart as usize;
    let end = start + client_ty_def.method_count as usize;
    let methods = &metadata.methods[start..end];

    for method in methods {
        if metadata.get_string_by_index(method.nameIndex) == rpc_name {
            let return_type = &il2cpp.types[method.returnType as usize];
            return match return_type.get_complex_type(il2cpp)? {
                ComplexType::Generic { ref base, .. } => {
                    let base_name = base.to_string();
                    if base_name == "AsyncDuplexStreamingCall" {
                        Ok((true, true))
                    } else if base_name == "AsyncClientStreamingCall" {
                        Ok((true, false))
                    } else if base_name == "AsyncServerStreamingCall" {
                        Ok((false, true))
                    } else {
                        Ok((false, false))
                    }
                }
                _ => Ok((false, false)),
            };
        }
    }
    Ok((false, false))
}

fn process_enum<'a>(
    ty_def: &Il2CppTypeDefinition,
    il2cpp: &'a Il2Cpp<'a>,
    package: &mut ProtoPackage,
    nested_types_map: &mut HashMap<TypeIndex, Vec<ProtoType>>,
    oneof_cases: &mut HashMap<TypeIndex, Vec<ProtoEnum>>,
) -> Result<()> {
    let enum_type = parse_enum_type(il2cpp, ty_def)?;
    if enum_type.name.ends_with("OneofCase") {
        oneof_cases
            .entry(ty_def.declaringTypeIndex)
            .or_default()
            .push(enum_type);
    } else if ty_def.declaringTypeIndex >= 0 {
        nested_types_map
            .entry(ty_def.declaringTypeIndex)
            .or_default()
            .push(ProtoType::Enum(enum_type));
    } else {
        package.add_enum(enum_type);
    }
    Ok(())
}

fn process_message<'a>(
    ty_def: &Il2CppTypeDefinition,
    il2cpp: &'a Il2Cpp<'a>,
    package: &mut ProtoPackage,
    nested_types_map: &mut HashMap<TypeIndex, Vec<ProtoType>>,
    oneof_cases: &mut HashMap<TypeIndex, Vec<ProtoEnum>>,
) -> Result<()> {
    let message_name = il2cpp.metadata.get_string_by_index(ty_def.nameIndex);
    let mut new_message = ProtoMessage::create(message_name, ty_def.byvalTypeIndex);

    let mut oneof_field_map: HashMap<String, Rc<RefCell<ProtoOneOf>>> = HashMap::new();
    let mut oneof_fields = Vec::new();
    if let Some(oneof_enums) = oneof_cases.remove(&ty_def.byvalTypeIndex) {
        for oneof_enum in oneof_enums {
            let oneof_name = oneof_enum
                .name
                .strip_suffix("OneofCase")
                .unwrap_or(&oneof_enum.name)
                .to_ascii_lowercase();
            let proto_field = Rc::new(RefCell::new(ProtoOneOf::create(oneof_name)));
            oneof_fields.push(proto_field.clone());
            for variant_name in oneof_enum.variants.keys() {
                oneof_field_map.insert(variant_name.clone(), proto_field.clone());
            }
        }
    }

    let methods_slice = {
        let start = ty_def.methodStart as usize;
        let end = start + ty_def.method_count as usize;
        &il2cpp.metadata.methods[start..end]
    };

    for field_idx in ty_def.field_indices() {
        let field = &il2cpp.metadata.fields[field_idx];
        let field_name = il2cpp.metadata.get_string_by_index(field.nameIndex);
        let proto_field_name = match field_name.strip_suffix("FieldNumber") {
            Some(n) => n,
            None => continue,
        };
        let proto_field_number = get_field_default_numeric_value(il2cpp, field_idx as i32)?;
        let getter_name = format!("get_{}", proto_field_name);

        if let Some(method) = methods_slice
            .iter()
            .find(|m| il2cpp.metadata.get_string_by_index(m.nameIndex) == getter_name)
        {
            let return_type = &il2cpp.types[method.returnType as usize];
            match return_type.get_complex_type(il2cpp)? {
                ComplexType::Simple {
                    mut module,
                    namespace,
                    name,
                    mut type_index,
                } => {
                    let simple_name = if let Some(ref ns) = namespace {
                        format!("{}.{}", ns, name)
                    } else {
                        name
                    };
                    let simple_type_name = if let Some(proto_name) = NET_TO_PROTO.get(&simple_name)
                    {
                        module = None;
                        type_index = None;
                        proto_name.to_string()
                    } else {
                        simple_name
                    };
                    let field_obj = ProtoField::new(
                        module,
                        proto_field_name.to_string(),
                        simple_type_name,
                        type_index,
                        proto_field_number,
                        None,
                    );
                    if let Some(oneof_field) = oneof_field_map.get(proto_field_name) {
                        oneof_field.borrow_mut().add_field(field_obj);
                    } else {
                        new_message.add_field(field_obj);
                    }
                }
                ComplexType::Generic { base, args } => {
                    let base_name = base.to_string();
                    if base_name == "MapField" {
                        new_message.add_map_field(ProtoMapField::new(
                            args.args[0].to_string(),
                            args.args[0].get_type_index(),
                            args.args[1].to_string(),
                            args.args[1].get_type_index(),
                            proto_field_name.to_string(),
                            proto_field_number,
                        ));
                    } else {
                        let cardinality = match base_name.as_str() {
                            "Nullable" => Some(ProtoCardinality::Optional),
                            "RepeatedField" => Some(ProtoCardinality::Repeated),
                            _ => unimplemented!("Cardinality unsupported: {}<{:?}>", base, args),
                        };
                        let mut module_name = args.get_module_name();
                        let inner_type = args.get_name_str(true)?;
                        let mut field_type_index = args.args[0].get_type_index();
                        let type_name = if let Some(proto_name) = NET_TO_PROTO.get(&inner_type) {
                            module_name = None;
                            field_type_index = None;
                            proto_name.to_string()
                        } else {
                            inner_type
                        };
                        let field_obj = ProtoField::new(
                            module_name,
                            proto_field_name.to_string(),
                            type_name,
                            field_type_index,
                            proto_field_number,
                            cardinality,
                        );
                        if let Some(oneof_field) = oneof_field_map.get(proto_field_name) {
                            oneof_field.borrow_mut().add_field(field_obj);
                        } else {
                            new_message.add_field(field_obj);
                        }
                    }
                }
                ct => unimplemented!("Complex type unsupported: {:?}", ct),
            }
        }
    }

    for oneof_field in oneof_fields {
        new_message.add_oneof(oneof_field.borrow().to_owned());
    }

    if ty_def.declaringTypeIndex >= 0 {
        nested_types_map
            .entry(ty_def.declaringTypeIndex)
            .or_default()
            .push(ProtoType::Message(new_message));
    } else {
        package.add_message(new_message);
    }
    Ok(())
}

fn process_nested_types<'a>(
    il2cpp: &'a Il2Cpp<'a>,
    nested_types_map: &mut HashMap<TypeIndex, Vec<ProtoType>>,
) -> Result<()> {
    let nested_type_indexes: Vec<TypeIndex> = nested_types_map.keys().copied().collect();
    for ty_idx in nested_type_indexes {
        let first_parent_ty = &il2cpp.types[ty_idx as usize];
        let mut ty_chain = first_parent_ty.get_declaring_chain(il2cpp)?;
        if ty_chain.len() == 1 {
            continue;
        }
        let new_target_ty_def = ty_chain.pop().unwrap().get_type_def(il2cpp)?.unwrap();
        let mut new_message: Option<ProtoMessage> = None;
        for ty in ty_chain {
            let ty_def = ty.get_type_def(il2cpp)?.unwrap();
            let ty_name = ty.get_complex_type(il2cpp)?;
            let mut ty_message =
                ProtoMessage::create(ty_name.get_name_str(false)?, ty_def.byvalTypeIndex);
            if let Some(base) = new_message.take() {
                ty_message.nested_messages.push(base);
            } else if let Some(nested_types) = nested_types_map.remove(&ty_idx) {
                for nested_type in nested_types {
                    match nested_type {
                        ProtoType::Enum(enum_type) => ty_message.nested_enums.push(enum_type),
                        ProtoType::Message(message) => ty_message.nested_messages.push(message),
                        _ => unreachable!(),
                    }
                }
            }
            new_message = Some(ty_message);
        }
        let new_message = new_message.unwrap();
        let nested_types = nested_types_map
            .entry(new_target_ty_def.byvalTypeIndex)
            .or_default();
        if let Some(ProtoType::Message(existing_msg)) =
            nested_types.iter_mut().find(|nested_type| {
                if let ProtoType::Message(msg) = nested_type {
                    msg.type_index == new_message.type_index
                } else {
                    false
                }
            })
        {
            existing_msg.merge(new_message);
        } else {
            nested_types.push(ProtoType::Message(new_message));
        }
    }
    Ok(())
}

fn integrate_nested_types_into_packages(
    schema: &mut ProtoSchema,
    nested_types_map: &mut HashMap<TypeIndex, Vec<ProtoType>>,
) {
    for namespace in schema.packages.values_mut() {
        for message in namespace.messages_mut() {
            if let Some(nested_types) = nested_types_map.remove(&message.type_index) {
                for nested_type in nested_types {
                    match nested_type {
                        ProtoType::Enum(e) => message.nested_enums.push(e),
                        ProtoType::Message(m) => message.nested_messages.push(m),
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
}

fn get_field_default_numeric_value<'a>(il2cpp: &'a Il2Cpp<'a>, field_index: i32) -> Result<i32> {
    let fdv = il2cpp
        .metadata
        .field_default_values_map
        .get(&field_index)
        .ok_or_else(|| {
            anyhow!(
                "No field default value found for field index {}",
                field_index
            )
        })?;
    let fdv_ty = &il2cpp.types[fdv.typeIndex as usize];
    let value = fdv_ty.get_value(
        il2cpp,
        &il2cpp.metadata.field_and_parameter_default_value_data,
        fdv.dataIndex as usize,
    )?;
    Ok(value.as_num()? as i32)
}

fn parse_enum_type<'a>(il2cpp: &'a Il2Cpp<'a>, ty_def: &Il2CppTypeDefinition) -> Result<ProtoEnum> {
    let type_name = il2cpp.metadata.get_string_by_index(ty_def.nameIndex);
    let mut new_enum = ProtoEnum::create(&type_name, ty_def.byvalTypeIndex);

    for j in ty_def.field_indices().skip(1) {
        let field = &il2cpp.metadata.fields[j];
        let element_name = il2cpp.metadata.get_string_by_index(field.nameIndex);
        let element_value = get_field_default_numeric_value(il2cpp, j as i32)?;
        new_enum.add_variant(&element_name, element_value);
    }

    Ok(new_enum)
}
