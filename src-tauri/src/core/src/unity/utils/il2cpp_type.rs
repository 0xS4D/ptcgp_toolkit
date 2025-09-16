use anyhow::{Result, anyhow};
use phf::phf_map;

use crate::unity::{
    generated::il2cpp_2022333f1::root::{Il2CppType, Il2CppTypeDefinition},
    utils::{
        blob_value::{BlobValue, BlobValueData},
        complex_type::{ComplexType, ComplexTypeNamespace},
        il2cpp::Il2Cpp,
        read_only::ReadOnly,
    },
};

static TYPE_MAP: phf::Map<i32, &'static str> = phf_map! {
    1i32 => "void",  2i32 => "bool",   3i32 => "char",    4i32 => "sbyte",
    5i32 => "byte",  6i32 => "short",  7i32 => "ushort",  8i32 => "int",
    9i32 => "uint",  10i32 => "long",  11i32 => "ulong",  12i32 => "float",
    13i32 => "double", 14i32 => "string", 22i32 => "TypedReference",
    24i32 => "IntPtr", 25i32 => "UIntPtr", 28i32 => "object",
};

impl<'a> ReadOnly<&'a Il2CppType> {
    pub fn get_type_def(
        &'a self,
        il2cpp: &'a Il2Cpp<'a>,
    ) -> Result<Option<&'a Il2CppTypeDefinition>> {
        use crate::unity::generated::il2cpp_2022333f1::root::*;

        Ok(match self.type_() {
            IL2CPP_TYPE_CLASS | IL2CPP_TYPE_VALUETYPE => unsafe {
                Some(&il2cpp.metadata.type_definitions[self.data.__klassIndex as usize])
            },
            IL2CPP_TYPE_GENERICINST => unsafe {
                let gclass = il2cpp
                    .load_data_instance::<Il2CppGenericClass>(self.data.generic_class as u64)?;
                let inst = il2cpp.load_data_instance::<Il2CppType>(gclass.type_ as u64)?;
                Some(&il2cpp.metadata.type_definitions[inst.data.__klassIndex as usize])
            },
            _ => None,
        })
    }

    pub fn get_declaring_type(
        &'a self,
        il2cpp: &'a Il2Cpp<'a>,
    ) -> Result<Option<&'a ReadOnly<&'a Il2CppType>>> {
        Ok(self.get_type_def(il2cpp)?.and_then(|td| {
            td.declaringTypeIndex
                .try_into()
                .ok()
                .map(|idx: usize| &il2cpp.types[idx])
        }))
    }

    pub fn get_declaring_chain(
        &'a self,
        il2cpp: &'a Il2Cpp<'a>,
    ) -> Result<Vec<&'a ReadOnly<&'a Il2CppType>>> {
        use std::iter::successors;

        let iter = successors(Some(Ok(self)), |ty_res| {
            ty_res
                .as_ref()
                .ok()
                .and_then(|ty| ty.get_declaring_type(il2cpp).transpose())
        });

        iter.collect()
    }

    pub fn get_complex_type(&'a self, il2cpp: &'a Il2Cpp<'a>) -> Result<ComplexType> {
        use crate::unity::generated::il2cpp_2022333f1::root::*;

        let ty = self.type_();
        Ok(match ty {
            IL2CPP_TYPE_ARRAY | IL2CPP_TYPE_SZARRAY | IL2CPP_TYPE_PTR => {
                let inner_ptr = unsafe {
                    if ty == IL2CPP_TYPE_ARRAY {
                        (*self.data.array).etype as u64
                    } else {
                        self.data.type_ as u64
                    }
                };
                let inner_ty = il2cpp
                    .type_by_ptr(inner_ptr)
                    .ok_or_else(|| anyhow!("Unknown inner type for {:?}", ty))?;
                let inner = inner_ty.get_complex_type(il2cpp)?;
                match ty {
                    IL2CPP_TYPE_PTR => ComplexType::Pointer(Box::new(inner)),
                    _ => ComplexType::Array(Box::new(inner)),
                }
            }

            IL2CPP_TYPE_VAR | IL2CPP_TYPE_MVAR => unsafe {
                let gen_param =
                    il2cpp.metadata.generic_parameters[self.data.__genericParameterIndex as usize];
                let name = il2cpp.metadata.get_string_by_index(gen_param.nameIndex);
                ComplexType::Simple {
                    module: None,
                    namespace: None,
                    name,
                    type_index: None,
                }
            },

            IL2CPP_TYPE_CLASS | IL2CPP_TYPE_VALUETYPE => unsafe {
                let type_def = &il2cpp.metadata.type_definitions[self.data.__klassIndex as usize];
                let simple = self.build_simple_from_typedef(il2cpp, type_def)?;
                self.wrap_generic_container(il2cpp, type_def, simple)?
            },

            IL2CPP_TYPE_GENERICINST => unsafe {
                let gclass = il2cpp
                    .load_data_instance::<Il2CppGenericClass>(self.data.generic_class as u64)?;
                let inst = il2cpp.load_data_instance::<Il2CppType>(gclass.type_ as u64)?;
                let type_def = &il2cpp.metadata.type_definitions[inst.data.__klassIndex as usize];
                let base = self.build_simple_from_typedef(il2cpp, type_def)?;

                let ginst = il2cpp
                    .load_data_instance::<Il2CppGenericInst>(gclass.context.class_inst as u64)?;
                let args_ptrs = il2cpp
                    .elf
                    .read_pointer_array(ginst.type_argv as u64, ginst.type_argc as usize);

                let args: Result<Vec<_>> = args_ptrs
                    .into_iter()
                    .map(|ptr| {
                        il2cpp
                            .type_by_ptr(ptr)
                            .ok_or_else(|| anyhow!("Unknown generic arg"))?
                            .get_complex_type(il2cpp)
                    })
                    .collect();

                ComplexType::Generic {
                    base: Box::new(base),
                    args: args?.into(),
                }
            },

            _ => TYPE_MAP
                .get(&(ty as i32))
                .map(|s| ComplexType::Simple {
                    module: None,
                    namespace: None,
                    name: s.to_string(),
                    type_index: None,
                })
                .unwrap_or_else(|| ComplexType::Simple {
                    module: None,
                    namespace: None,
                    name: format!("unknown_{}", ty),
                    type_index: None,
                }),
        })
    }

    unsafe fn build_simple_from_typedef(
        &'a self,
        il2cpp: &'a Il2Cpp<'a>,
        td: &Il2CppTypeDefinition,
    ) -> Result<ComplexType> {
        let raw_name = il2cpp.metadata.get_string_by_index(td.nameIndex);
        let ns = il2cpp.metadata.get_string_by_index(td.namespaceIndex);

        let module = if !ns.is_empty() || td.declaringTypeIndex == -1 {
            Some(ns)
        } else {
            self.get_declaring_chain(il2cpp)?.last().and_then(|outer| {
                match outer.get_complex_type(il2cpp).ok()? {
                    ComplexType::Simple {
                        module: Some(m), ..
                    } => Some(m),
                    _ => None,
                }
            })
        };

        let base_name = raw_name.split('`').next().unwrap_or(&raw_name).to_string();

        let namespace = if td.declaringTypeIndex != -1 {
            let declaring_ty = &il2cpp.types[td.declaringTypeIndex as usize];
            Some(ComplexTypeNamespace::Complex(Box::new(
                declaring_ty.get_complex_type(il2cpp)?,
            )))
        } else if let Some(pos) = base_name.rfind('.') {
            Some(ComplexTypeNamespace::Simple(base_name[..pos].to_string()))
        } else {
            None
        };

        let name = if let Some(pos) = base_name.rfind('.') {
            base_name[pos + 1..].to_string()
        } else {
            base_name
        };

        Ok(ComplexType::Simple {
            module,
            namespace,
            name,
            type_index: Some(td.byvalTypeIndex),
        })
    }

    unsafe fn wrap_generic_container(
        &'a self,
        il2cpp: &'a Il2Cpp<'a>,
        type_def: &Il2CppTypeDefinition,
        simple: ComplexType,
    ) -> Result<ComplexType> {
        if type_def.genericContainerIndex >= 0 {
            let generic_container =
                &il2cpp.metadata.generic_containers[type_def.genericContainerIndex as usize];
            let generic_args: Vec<ComplexType> = (0..generic_container.type_argc)
                .map(|i| {
                    let idx = generic_container.genericParameterStart + i;
                    let param = il2cpp.metadata.generic_parameters[idx as usize];
                    let name = il2cpp.metadata.get_string_by_index(param.nameIndex);
                    ComplexType::Simple {
                        module: None,
                        namespace: None,
                        name,
                        type_index: None,
                    }
                })
                .collect();
            Ok(ComplexType::Generic {
                base: Box::new(simple),
                args: generic_args.into(),
            })
        } else {
            Ok(simple)
        }
    }

    pub fn get_value(
        &self,
        il2cpp: &'a Il2Cpp<'a>,
        data: &[u8],
        offset: usize,
    ) -> Result<BlobValue> {
        use crate::unity::generated::il2cpp_2022333f1::root::*;

        let ty = self.type_();

        fn make_value(il2cpp_type_enum: i32, value: BlobValueData) -> Result<BlobValue> {
            Ok(BlobValue {
                il2cpp_type_enum,
                value,
                enum_type: None,
            })
        }

        macro_rules! read_and_wrap {
            ($il2cpp:expr, $data:expr, $offset:expr, $ty:expr, $method:ident, $variant:ident) => {{
                let v = $il2cpp.metadata.$method($data, $offset);
                make_value($ty, BlobValueData::$variant(v))
            }};
        }

        match ty {
            IL2CPP_TYPE_BOOLEAN => {
                let b = il2cpp.metadata.read_u8(data, offset) != 0;
                make_value(ty, BlobValueData::Boolean(b))
            }
            IL2CPP_TYPE_CHAR => {
                let num = il2cpp.metadata.read_u16(data, offset);
                let c = char::from_u32(num as u32)
                    .ok_or_else(|| anyhow!("Invalid char value: {}", num))?;
                make_value(ty, BlobValueData::Char(c))
            }

            IL2CPP_TYPE_U1 => read_and_wrap!(il2cpp, data, offset, ty, read_u8, U1),
            IL2CPP_TYPE_I1 => read_and_wrap!(il2cpp, data, offset, ty, read_i8, I1),
            IL2CPP_TYPE_U2 => read_and_wrap!(il2cpp, data, offset, ty, read_u16, U2),
            IL2CPP_TYPE_I2 => read_and_wrap!(il2cpp, data, offset, ty, read_i16, I2),
            IL2CPP_TYPE_U4 => read_and_wrap!(il2cpp, data, offset, ty, read_compressed_u32, U4),
            IL2CPP_TYPE_I4 => read_and_wrap!(il2cpp, data, offset, ty, read_compressed_i32, I4),
            IL2CPP_TYPE_U8 => read_and_wrap!(il2cpp, data, offset, ty, read_u64, U8),
            IL2CPP_TYPE_I8 => read_and_wrap!(il2cpp, data, offset, ty, read_i64, I8),
            IL2CPP_TYPE_R4 => read_and_wrap!(il2cpp, data, offset, ty, read_f32, R4),
            IL2CPP_TYPE_R8 => read_and_wrap!(il2cpp, data, offset, ty, read_f64, R8),

            IL2CPP_TYPE_STRING => {
                let length = il2cpp.metadata.read_compressed_i32(data, offset);
                let s = if length == -1 {
                    String::new()
                } else {
                    let end = offset
                        .checked_add(length as usize)
                        .ok_or_else(|| anyhow!("Invalid string length: {}", length))?;
                    let slice = data.get(offset..end).ok_or_else(|| {
                        anyhow!("Out of bounds string slice: len {} at {}", length, offset)
                    })?;
                    String::from_utf8(slice.to_vec())?
                };
                make_value(ty, BlobValueData::String(s))
            }

            IL2CPP_TYPE_SZARRAY => {
                let array_len = il2cpp.metadata.read_compressed_i32(data, offset);
                if array_len == -1 {
                    return make_value(ty, BlobValueData::Array(Vec::new()));
                }

                let mut elements = Vec::with_capacity(array_len as usize);

                let (base_elem_type, mut elem_enum_type) =
                    il2cpp.read_encoded_type_enum(data, offset);
                let has_varying_types = il2cpp.metadata.read_u8(data, offset) == 1;

                for _ in 0..array_len {
                    let elem_type = if has_varying_types {
                        let (et, et_enum) = il2cpp.read_encoded_type_enum(data, offset);
                        elem_enum_type = et_enum;
                        et
                    } else {
                        base_elem_type
                    };

                    let mut value = self.get_value(il2cpp, data, offset)?;
                    value.il2cpp_type_enum = elem_type;
                    value.enum_type = elem_enum_type.map(|t| **(*t));
                    elements.push(value);
                }

                make_value(ty, BlobValueData::Array(elements))
            }

            IL2CPP_TYPE_IL2CPP_TYPE_INDEX => {
                let type_index = il2cpp.metadata.read_compressed_i32(data, offset);
                let type_obj = if type_index == -1 {
                    None
                } else {
                    il2cpp.types.get(type_index as usize).map(|t| *(**t))
                };
                make_value(ty, BlobValueData::TypeIndex(type_obj))
            }

            _ => Err(anyhow!("Unsupported type in get_value: {:?}", ty)),
        }
    }
}
