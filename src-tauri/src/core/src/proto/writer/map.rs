use crate::proto::map::ProtoMapField;
use heck::ToSnakeCase;
use std::fmt::{self, Write};

impl ProtoMapField {
    pub fn fmt_pretty(&self, f: &mut String, indent: usize) -> fmt::Result {
        writeln!(
            f,
            "{:width$}map<{}, {}> {} = {};",
            "",
            self.key_type,
            self.value_type,
            self.name.to_snake_case(),
            self.tag,
            width = indent
        )
    }

    pub fn to_pretty_string(&self, indent: usize) -> String {
        let mut s = String::with_capacity(64);
        self.fmt_pretty(&mut s, indent).expect("Formatting error");
        s
    }
}
