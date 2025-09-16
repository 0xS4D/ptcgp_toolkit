use crate::proto::one_of::ProtoOneOf;
use crate::proto::writer::DEFAULT_INDENT_SIZE;
use heck::ToSnakeCase;
use std::fmt::{self, Write};

impl ProtoOneOf {
    pub fn fmt_pretty(
        &self,
        f: &mut String,
        indent: usize,
        current_namespace: &str,
    ) -> fmt::Result {
        writeln!(
            f,
            "{:width$}oneof {} {{",
            "",
            self.name.to_snake_case(),
            width = indent
        )?;
        let mut sorted_fields: Vec<_> = self.fields.iter().collect();
        sorted_fields.sort_by_key(|f| f.tag);
        for field in sorted_fields {
            let with_namespace =
                !field.namespace.is_empty() && field.namespace != current_namespace;
            field.fmt_pretty(f, indent + DEFAULT_INDENT_SIZE, with_namespace)?;
        }
        writeln!(f, "{:width$}}}", "", width = indent)
    }

    pub fn to_pretty_string(&self, indent: usize, current_namespace: &str) -> String {
        let mut s = String::with_capacity(64);
        self.fmt_pretty(&mut s, indent, current_namespace)
            .expect("Formatting error");
        s
    }
}
