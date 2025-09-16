use crate::proto::field::ProtoField;
use crate::proto::map::ProtoMapField;
use crate::proto::one_of::ProtoOneOf;
use crate::proto::proto_enum::ProtoEnum;
use crate::unity::generated::il2cpp_2022333f1::root::TypeIndex;

#[derive(Clone, PartialEq)]
pub struct ProtoMessage {
    pub name: String,
    pub fields: Vec<ProtoField>,
    pub oneofs: Vec<ProtoOneOf>,
    pub map_fields: Vec<ProtoMapField>,
    pub nested_messages: Vec<ProtoMessage>,
    pub nested_enums: Vec<ProtoEnum>,
    pub type_index: TypeIndex,
}

impl ProtoMessage {
    pub fn create<S: Into<String>>(name: S, type_index: TypeIndex) -> Self {
        Self {
            name: name.into(),
            fields: Vec::new(),
            oneofs: Vec::new(),
            map_fields: Vec::new(),
            nested_messages: Vec::new(),
            nested_enums: Vec::new(),
            type_index,
        }
    }

    pub fn merge(&mut self, other: ProtoMessage) {
        self.fields.extend(other.fields);
        self.oneofs.extend(other.oneofs);
        self.map_fields.extend(other.map_fields);
        self.nested_enums.extend(other.nested_enums);

        for nested in other.nested_messages {
            if let Some(existing) = self
                .nested_messages
                .iter_mut()
                .find(|m| m.type_index == nested.type_index)
            {
                existing.merge(nested);
            } else {
                self.nested_messages.push(nested);
            }
        }
    }

    pub fn add_field(&mut self, field: ProtoField) {
        self.fields.push(field);
    }

    pub fn add_oneof(&mut self, oneof: ProtoOneOf) {
        self.oneofs.push(oneof);
    }

    pub fn add_map_field(&mut self, map: ProtoMapField) {
        self.map_fields.push(map);
    }

    pub fn get_contained_types(&self) -> Vec<TypeIndex> {
        let mut contained_types = Vec::new();
        contained_types.push(self.type_index);
        for en in &self.nested_enums {
            contained_types.push(en.type_index);
        }
        for msg in &self.nested_messages {
            contained_types.extend(msg.get_contained_types());
        }
        contained_types
    }

    pub fn get_used_types(&self) -> Vec<TypeIndex> {
        let mut used_types = Vec::new();
        for field in &self.fields {
            if let Some(field_type_index) = field.field_type_index {
                used_types.push(field_type_index);
            }
        }
        for oneof in &self.oneofs {
            for field in &oneof.fields {
                if let Some(field_type_index) = field.field_type_index {
                    used_types.push(field_type_index);
                }
            }
        }
        for map in &self.map_fields {
            if let Some(key_type_index) = map.key_type_index {
                used_types.push(key_type_index);
            }
            if let Some(value_type_index) = map.value_type_index {
                used_types.push(value_type_index);
            }
        }
        for nested in &self.nested_messages {
            used_types.extend(nested.get_used_types());
        }
        for nested in &self.nested_enums {
            used_types.push(nested.type_index);
        }

        used_types
    }
}
