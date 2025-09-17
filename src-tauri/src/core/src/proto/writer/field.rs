use crate::proto::field::{ProtoCardinality, ProtoField};
use crate::proto::writer::format_package_name;
use heck::ToSnakeCase;
use std::fmt::{self, Write};

impl ProtoField {
    pub fn fmt_pretty(&self, f: &mut String, indent: usize, with_namespace: bool, legacy: bool) -> fmt::Result {
        write!(f, "{:width$}", "", width = indent)?;
        let field_type_str = if with_namespace {
            format!(
                "{}.{}",
                format_package_name(&self.namespace, legacy),
                self.field_type
            )
        } else {
            self.field_type.clone()
        };
        match self.cardinality {
            ProtoCardinality::Single => writeln!(
                f,
                "{} {} = {};",
                field_type_str,
                self.name.to_snake_case(),
                self.tag
            ),
            _ => writeln!(
                f,
                "{} {} {} = {};",
                self.cardinality,
                field_type_str,
                self.name.to_snake_case(),
                self.tag
            ),
        }
    }

    pub fn to_pretty_string(&self, indent: usize, with_namespace: bool, legacy: bool) -> String {
        let mut s = String::with_capacity(64);
        self.fmt_pretty(&mut s, indent, with_namespace, legacy)
            .expect("Formatting error");
        s
    }
}

impl fmt::Display for ProtoCardinality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProtoCardinality::Optional => write!(f, "optional"),
            ProtoCardinality::Repeated => write!(f, "repeated"),
            ProtoCardinality::Single => Ok(()),
        }
    }
}
