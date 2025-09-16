use crate::proto::circular::{ProtoMessageGroups, messages_to_message_groups};
use crate::proto::message::ProtoMessage;
use crate::proto::proto_enum::ProtoEnum;
use crate::proto::service::ProtoService;
use crate::unity::generated::il2cpp_2022333f1::root::TypeIndex;
use nohash_hasher::IntSet;

#[derive(Clone, Default, PartialEq)]
pub struct ProtoPackage {
    is_sealed: bool,
    pub package_name: String,
    pub header_comments: Vec<String>,
    pub enums: Vec<ProtoEnum>,
    messages: Vec<ProtoMessage>,
    pub msg_groups: Option<ProtoMessageGroups>,
    pub services: Vec<ProtoService>,
    pub used_types: IntSet<TypeIndex>,
    pub contained_types: IntSet<TypeIndex>,
}

impl ProtoPackage {
    pub fn new<S: Into<String>>(package_name: S, header_comments: Vec<String>) -> Self {
        Self {
            is_sealed: false,
            package_name: package_name.into(),
            header_comments,
            enums: Vec::new(),
            messages: Vec::new(),
            msg_groups: None,
            services: Vec::new(),
            used_types: IntSet::default(),
            contained_types: IntSet::default(),
        }
    }

    pub fn add_enum(&mut self, en: ProtoEnum) {
        if self.is_sealed {
            panic!("Cannot add enum to sealed package");
        }
        self.enums.push(en);
    }

    pub fn add_message(&mut self, msg: ProtoMessage) {
        if self.is_sealed {
            panic!("Cannot add message to sealed package");
        }
        self.messages.push(msg);
    }

    pub fn add_service(&mut self, service: ProtoService) {
        if self.is_sealed {
            panic!("Cannot add service to sealed package");
        }
        self.services.push(service);
    }

    pub fn is_empty(&self) -> bool {
        self.enums.is_empty()
            && self.messages.is_empty()
            && (self.msg_groups.as_ref().is_none_or(|g| g.is_empty()))
            && self.services.is_empty()
    }

    pub fn seal(&mut self) {
        self.is_sealed = true;
        let messages = std::mem::take(&mut self.messages);
        self.msg_groups = Some(messages_to_message_groups(messages));
        self.store_types();
    }

    fn store_types(&mut self) {
        for en in &self.enums {
            self.contained_types.insert(en.type_index);
        }
        for msg in self.msg_groups.as_ref().unwrap().iter() {
            self.contained_types.extend(msg.get_contained_types());
            self.used_types.extend(msg.get_used_types());
        }
        for svc in &self.services {
            self.contained_types.insert(svc.type_index);
            for method in &svc.methods {
                if let Some(type_index) = method.input_type_index {
                    self.used_types.insert(type_index);
                }
                if let Some(type_index) = method.output_type_index {
                    self.used_types.insert(type_index);
                }
            }
        }
    }

    pub fn messages(&self) -> &[ProtoMessage] {
        if self.is_sealed {
            panic!("Cannot access messages of sealed package");
        }
        &self.messages
    }

    pub fn messages_mut(&mut self) -> &mut [ProtoMessage] {
        if self.is_sealed {
            panic!("Cannot access messages of sealed package");
        }
        &mut self.messages
    }
}
