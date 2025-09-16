use crate::unity::generated::il2cpp_2022333f1::root::TypeIndex;

#[derive(Clone, PartialEq)]
pub struct ProtoService {
    pub name: String,
    pub type_index: TypeIndex,
    pub methods: Vec<ProtoServiceMethod>,
}

impl ProtoService {
    pub fn new<S: Into<String>>(name: S, type_index: TypeIndex) -> Self {
        Self {
            name: name.into(),
            type_index,
            methods: Vec::new(),
        }
    }

    pub fn add_method(&mut self, method: ProtoServiceMethod) {
        self.methods.push(method);
    }

    pub fn get_used_types(&self) -> Vec<TypeIndex> {
        let mut used_types = Vec::new();
        for method in &self.methods {
            if let Some(input_type_index) = method.input_type_index {
                used_types.push(input_type_index);
            }
            if let Some(input_type_index) = method.output_type_index {
                used_types.push(input_type_index);
            }
        }

        used_types
    }
}

#[derive(Clone, PartialEq)]
pub struct ProtoServiceMethod {
    pub name: String,
    pub input_namespace: Option<String>,
    pub input_type: String,
    pub input_type_index: Option<TypeIndex>,
    pub output_namespace: Option<String>,
    pub output_type: String,
    pub output_type_index: Option<TypeIndex>,
    pub client_streaming: bool,
    pub server_streaming: bool,
}

impl ProtoServiceMethod {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        input_namespace: Option<String>,
        input_type: String,
        input_type_index: Option<TypeIndex>,
        output_namespace: Option<String>,
        output_type: String,
        output_type_index: Option<TypeIndex>,
        client_streaming: bool,
        server_streaming: bool,
    ) -> Self {
        Self {
            name,
            input_namespace,
            input_type,
            input_type_index,
            output_namespace,
            output_type,
            output_type_index,
            client_streaming,
            server_streaming,
        }
    }
}
