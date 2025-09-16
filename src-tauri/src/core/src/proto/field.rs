use crate::unity::generated::il2cpp_2022333f1::root::TypeIndex;

#[derive(Clone, PartialEq)]
pub enum ProtoCardinality {
    Single,
    Optional,
    Repeated,
}

#[derive(Clone, PartialEq)]
pub struct ProtoField {
    pub namespace: String,
    pub name: String,
    pub field_type: String,
    pub field_type_index: Option<TypeIndex>,
    pub tag: i32,
    pub cardinality: ProtoCardinality,
}

impl ProtoField {
    pub fn new(
        namespace: Option<String>,
        name: String,
        field_type: String,
        field_type_index: Option<TypeIndex>,
        tag: i32,
        cardinality: Option<ProtoCardinality>,
    ) -> Self {
        let field = Self {
            namespace: namespace.unwrap_or_default(),
            name,
            field_type,
            field_type_index,
            tag,
            cardinality: cardinality.unwrap_or(ProtoCardinality::Single),
        };
        field.remap_builtin_type()
    }

    fn remap_builtin_type(mut self) -> Self {
        if self.namespace == "Google.Protobuf.WellKnownTypes" {
            self.namespace = "google.protobuf".to_string();
        }
        self
    }
}
