use crate::proto::proto_enum::{ProtoEnum, ProtoEnumVariant};
use crate::proto::writer::DEFAULT_INDENT_SIZE;
use heck::ToShoutySnakeCase;
use std::fmt::{self, Write};

impl ProtoEnum {
    pub fn fmt_pretty(&self, f: &mut String, indent: usize) -> fmt::Result {
        writeln!(f, "{:width$}enum {} {{", "", self.name, width = indent)?;
        let mut variants: Vec<_> = self.variants.values().collect();
        variants.sort_by_key(|v| v.tag);
        for variant in variants {
            variant.fmt_pretty(f, indent + DEFAULT_INDENT_SIZE)?;
        }
        writeln!(f, "{:width$}}}", "", width = indent)
    }

    pub fn to_pretty_string(&self, indent: usize) -> String {
        let mut s = String::with_capacity(64);
        self.fmt_pretty(&mut s, indent).expect("Formatting error");
        s
    }
}

impl ProtoEnumVariant {
    pub fn fmt_pretty(&self, f: &mut String, indent: usize) -> fmt::Result {
        writeln!(
            f,
            "{:width$}{} = {};",
            "",
            self.name.to_shouty_snake_case(),
            self.tag,
            width = indent
        )
    }

    pub fn to_pretty_string(&self, indent: usize) -> String {
        let mut s = String::with_capacity(32);
        self.fmt_pretty(&mut s, indent).expect("Formatting error");
        s
    }
}
