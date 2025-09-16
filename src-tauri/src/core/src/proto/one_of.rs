use crate::proto::field::{ProtoCardinality, ProtoField};

#[derive(Clone, PartialEq)]
pub struct ProtoOneOf {
    pub name: String,
    pub fields: Vec<ProtoField>,
}

impl ProtoOneOf {
    pub fn create(name: String) -> Self {
        Self {
            name,
            fields: Vec::new(),
        }
    }

    pub fn add_field(&mut self, mut field: ProtoField) {
        field.cardinality = ProtoCardinality::Single;
        self.fields.push(field);
    }
}
