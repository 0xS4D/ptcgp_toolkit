use crate::unity::generated::il2cpp_2022333f1::root::TypeIndex;
use hashbrown::HashMap;

#[derive(Clone, Default, PartialEq)]
pub struct ProtoEnum {
    pub name: String,
    pub variants: HashMap<String, ProtoEnumVariant>,
    pub type_index: TypeIndex,
}

impl ProtoEnum {
    pub fn create(name: &str, type_index: TypeIndex) -> Self {
        Self {
            name: name.to_string(),
            variants: HashMap::new(),
            type_index,
        }
    }

    pub fn add_variant(&mut self, name: &str, number: i32) {
        let enum_name = format!("{}_{}", self.name, name);
        self.variants
            .insert(name.into(), ProtoEnumVariant::new(enum_name, number));
    }
}

#[derive(Clone, PartialEq)]
pub struct ProtoEnumVariant {
    pub name: String,
    pub tag: i32,
}

impl ProtoEnumVariant {
    pub fn new<S: Into<String>>(name: S, tag: i32) -> Self {
        Self {
            name: name.into(),
            tag,
        }
    }
}
