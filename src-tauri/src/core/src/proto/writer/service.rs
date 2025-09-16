use crate::proto::service::{ProtoService, ProtoServiceMethod};
use crate::proto::writer::DEFAULT_INDENT_SIZE;
use heck::ToUpperCamelCase;
use std::fmt::{self, Write};

impl ProtoService {
    pub fn fmt_pretty(&self, f: &mut String, indent: usize) -> fmt::Result {
        writeln!(f, "{:width$}service {} {{", "", self.name, width = indent)?;
        for method in &self.methods {
            method.fmt_pretty(f, indent + DEFAULT_INDENT_SIZE)?;
        }
        writeln!(f, "{:width$}}}", "", width = indent)
    }

    pub fn to_pretty_string(&self, indent: usize) -> String {
        let mut s = String::with_capacity(128);
        self.fmt_pretty(&mut s, indent).expect("Formatting error");
        s
    }
}

impl ProtoServiceMethod {
    pub fn fmt_pretty(&self, f: &mut String, indent: usize) -> fmt::Result {
        write!(
            f,
            "{:width$}rpc {} (",
            "",
            self.name.to_upper_camel_case(),
            width = indent
        )?;

        if self.client_streaming {
            write!(f, "stream ")?;
        }
        write!(f, "{}) returns (", self.input_type)?;

        if self.server_streaming {
            write!(f, "stream ")?;
        }
        writeln!(f, "{});", self.output_type)
    }

    pub fn to_pretty_string(&self, indent: usize) -> String {
        let mut s = String::with_capacity(64);
        self.fmt_pretty(&mut s, indent).expect("Formatting error");
        s
    }
}
