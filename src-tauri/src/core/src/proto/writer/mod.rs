mod field;
mod map;
mod message;
mod message_group;
mod one_of;
mod proto_enum;
mod service;

use hashbrown::HashSet;
use itertools::Itertools;
use std::fmt::{self, Write};
use std::fs::File;
use std::io;
use std::path::PathBuf;

pub(crate) const DEFAULT_INDENT_SIZE: usize = 2;

pub struct ProtoGenFile {
    pub filename: String,
    pub source_code: String,
}

impl ProtoGenFile {
    pub(crate) fn new(
        filename: String,
        package_name: &str,
        header_comments: &[String],
        imports: Option<HashSet<String>>,
        content: &str,
    ) -> Result<ProtoGenFile, fmt::Error> {
        let mut source_code = String::with_capacity(1024);
        write_header(
            &mut source_code,
            format_package_name(package_name),
            header_comments,
            imports,
        )?;
        writeln!(source_code, "{}", content)?;
        Ok(ProtoGenFile {
            filename,
            source_code,
        })
    }
}

pub(crate) fn write_header(
    f: &mut String,
    package_name: String,
    header_comments: &[String],
    imports: Option<HashSet<String>>,
) -> fmt::Result {
    if !header_comments.is_empty() {
        for comment in header_comments {
            writeln!(f, "// {comment}")?;
        }
        f.push('\n');
    }

    writeln!(f, "syntax = \"proto3\";")?;
    f.push('\n');

    writeln!(f, "package {package_name};")?;
    f.push('\n');

    if let Some(imports) = imports {
        if !imports.is_empty() {
            for import in imports.iter().sorted() {
                writeln!(f, "import \"{import}\";")?;
            }
            f.push('\n');
        }
    }

    Ok(())
}

pub(crate) fn format_package_name(package_name: &str) -> String {
    package_name.to_string()
}

pub(crate) fn format_package_filename(package_name: &str) -> String {
    let mut parts: Vec<_> = package_name.split('.').collect();
    if !parts.is_empty() {
        parts.pop();
    }
    format!("{}.proto", parts.join("."))
}

pub(crate) fn write_entry_file(
    file_path: PathBuf,
    namespace: &str,
    public_imports: Vec<String>,
) -> io::Result<()> {
    use std::io::Write;

    let mut file = File::create(file_path)?;

    writeln!(file, "syntax = \"proto3\";\n")?;
    writeln!(file, "package {};\n", namespace)?;

    for import in public_imports {
        writeln!(file, "import public \"{}\";", import)?;
    }

    Ok(())
}
