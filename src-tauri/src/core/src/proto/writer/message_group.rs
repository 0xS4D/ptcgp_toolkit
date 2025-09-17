use crate::proto::circular::ProtoMessageGroup;
use std::fmt::{self};

impl ProtoMessageGroup {
    pub fn fmt_pretty(
        &self,
        f: &mut String,
        indent: usize,
        current_namespace: &str,
        legacy: bool,
    ) -> fmt::Result {
        for msg in self.iter() {
            msg.fmt_pretty(f, indent, current_namespace, legacy)?;
            f.push('\n');
        }

        Ok(())
    }

    pub fn to_pretty_string(&self, indent: usize, current_namespace: &str, legacy: bool) -> String {
        let mut s = String::with_capacity(256);
        self.fmt_pretty(&mut s, indent, current_namespace, legacy)
            .expect("Formatting error");
        s
    }
}
