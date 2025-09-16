use crate::unity::generated::il2cpp_2022333f1::root::TypeIndex;

#[derive(Clone, PartialEq)]
pub struct ProtoMapField {
    pub name: String,
    pub key_type: String,
    pub key_type_index: Option<TypeIndex>,
    pub value_type: String,
    pub value_type_index: Option<TypeIndex>,
    pub tag: i32,
}

impl ProtoMapField {
    pub fn new(
        key_type: String,
        key_type_index: Option<TypeIndex>,
        value_type: String,
        value_type_index: Option<TypeIndex>,
        name: String,
        tag: i32,
    ) -> Self {
        Self {
            name,
            key_type,
            key_type_index,
            value_type,
            value_type_index,
            tag,
        }
    }
}
