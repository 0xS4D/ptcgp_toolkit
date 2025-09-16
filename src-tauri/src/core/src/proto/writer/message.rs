use crate::proto::message::ProtoMessage;
use crate::proto::writer::DEFAULT_INDENT_SIZE;
use std::fmt::{self, Write};

impl ProtoMessage {
    pub fn fmt_pretty(
        &self,
        f: &mut String,
        indent: usize,
        current_namespace: &str,
    ) -> fmt::Result {
        writeln!(f, "{:width$}message {} {{", "", self.name, width = indent)?;

        for en in &self.nested_enums {
            en.fmt_pretty(f, indent + DEFAULT_INDENT_SIZE)?;
        }
        for msg in &self.nested_messages {
            msg.fmt_pretty(f, indent + DEFAULT_INDENT_SIZE, current_namespace)?;
        }
        let mut sorted_fields: Vec<_> = self.fields.iter().collect();
        sorted_fields.sort_by_key(|f| f.tag);
        for field in sorted_fields {
            let with_namespace =
                !field.namespace.is_empty() && field.namespace != current_namespace;
            field.fmt_pretty(f, indent + DEFAULT_INDENT_SIZE, with_namespace)?;
        }
        for oneof in &self.oneofs {
            oneof.fmt_pretty(f, indent + DEFAULT_INDENT_SIZE, current_namespace)?;
        }
        let mut sorted_map_fields: Vec<_> = self.map_fields.iter().collect();
        sorted_map_fields.sort_by_key(|m| m.tag);
        for map_field in sorted_map_fields {
            map_field.fmt_pretty(f, indent + DEFAULT_INDENT_SIZE)?;
        }
        writeln!(f, "{:width$}}}", "", width = indent)
    }

    pub fn to_pretty_string(&self, indent: usize, current_namespace: &str) -> String {
        let mut s = String::with_capacity(256);
        self.fmt_pretty(&mut s, indent, current_namespace)
            .expect("Formatting error");
        s
    }
}
