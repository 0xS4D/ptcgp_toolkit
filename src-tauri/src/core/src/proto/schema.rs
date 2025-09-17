use crate::proto::package::ProtoPackage;
use crate::proto::writer::ProtoGenFile;
use crate::proto::{message::ProtoMessage, writer::format_package_filename};
use crate::unity::generated::il2cpp_2022333f1::root::TypeIndex;
use anyhow::{Result, anyhow};
use hashbrown::{HashMap, HashSet};
use nohash_hasher::{IntMap, IntSet};
use std::fmt::Write as FmtWrite;

pub struct ProtoGenUnit {
    pub namespace: String,
    pub header_comments: Vec<String>,
    pub imports: HashSet<String>,
    pub messages: Vec<String>,
    pub enums: Vec<String>,
    pub services: Vec<String>,
}

impl ProtoGenUnit {
    pub fn render(&self) -> ProtoGenFile {
        let mut source_code = String::new();

        writeln!(source_code, "syntax = \"proto3\";").unwrap();
        writeln!(
            source_code,
            "option csharp_namespace = \"{}\";",
            self.namespace
        )
        .unwrap();
        writeln!(source_code, "package {};", self.namespace).unwrap();
        writeln!(source_code).unwrap();

        let mut imports: Vec<_> = self.imports.iter().cloned().collect();

        imports.retain(|imp| {
            let ns_path = imp.replace(".proto", "");
            ns_path != self.namespace
        });

        for import in imports {
            writeln!(source_code, "import \"{}\";", import).unwrap();
        }

        writeln!(source_code).unwrap();

        for en in &self.enums {
            source_code.push_str(en);
            source_code.push('\n');
        }
        for msg in &self.messages {
            source_code.push_str(msg);
            source_code.push('\n');
        }
        for svc in &self.services {
            source_code.push_str(svc);
            source_code.push('\n');
        }

        let filename = self.namespace.clone() + ".proto";

        ProtoGenFile {
            filename,
            source_code,
        }
    }
}

#[derive(Default)]
pub struct ProtoGenSchema {
    pub enums: Vec<ProtoGenFile>,
    pub messages: Vec<ProtoGenFile>,
    pub services: Vec<ProtoGenFile>,
}

impl ProtoGenSchema {
    pub fn count(&self) -> usize {
        self.enums.len() + self.messages.len() + self.services.len()
    }
}

#[derive(Default)]
pub struct ProtoSchema {
    pub packages: HashMap<String, ProtoPackage>,
    type_file_mapping: IntMap<TypeIndex, String>,
}

impl ProtoSchema {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, package: ProtoPackage) {
        self.packages.insert(package.package_name.clone(), package);
    }

    pub fn get(&mut self, package_name: String) -> &mut ProtoPackage {
        self.packages
            .entry(package_name.clone())
            .or_insert_with(|| ProtoPackage::new(package_name, vec![]))
    }

    fn build_message_file_mappings(
        type_file_mapping: &mut IntMap<TypeIndex, String>,
        filename: String,
        msg: &ProtoMessage,
    ) {
        for nested_enum in &msg.nested_enums {
            type_file_mapping.insert(nested_enum.type_index, filename.clone());
        }
        for nested_msg in &msg.nested_messages {
            Self::build_message_file_mappings(type_file_mapping, filename.clone(), nested_msg);
        }
        type_file_mapping.insert(msg.type_index, filename);
    }

    fn build_package_file_mappings(
        type_file_mapping: &mut IntMap<TypeIndex, String>,
        package: &ProtoPackage,
    ) {
        for en in &package.enums {
            type_file_mapping.insert(
                en.type_index,
                format!("{}.{}", package.package_name, en.name),
            );
        }
        if let Some(msg_groups) = &package.msg_groups {
            for msg_group in msg_groups {
                let primary_msg = msg_group.get_primary();
                let filepath = format!("{}.{}", package.package_name, primary_msg.name);
                for msg in msg_group.iter() {
                    Self::build_message_file_mappings(type_file_mapping, filepath.clone(), msg);
                }
            }
        }
        for svc in &package.services {
            type_file_mapping.insert(
                svc.type_index,
                format!("{}.{}", package.package_name, svc.name),
            );
        }
    }

    pub fn seal(&mut self) {
        for package in self.packages.values_mut() {
            package.seal();
        }
        for package in self.packages.values_mut() {
            Self::build_package_file_mappings(&mut self.type_file_mapping, package);
        }
    }

    pub fn build(&self, legacy: bool) -> Result<ProtoGenSchema> {
        let all_used_types = self
            .packages
            .values()
            .flat_map(|package| package.used_types.clone())
            .collect::<IntSet<TypeIndex>>();

        let mut enums = Vec::new();
        let mut messages = Vec::new();
        let mut services = Vec::new();

        let filtered: Vec<_> = self
            .packages
            .values()
            .filter(move |package| {
                !package.package_name.starts_with("Google.")
                    && !package.is_empty()
                    && !package.contained_types.is_disjoint(&all_used_types)
            })
            .collect();

        for package in filtered {
            enums.extend(self.build_enums_for_package(package, legacy)?);
            messages.extend(self.build_messages_for_package(package, legacy)?);
            services.extend(self.build_services_for_package(package, legacy)?);
        }

        Ok(ProtoGenSchema {
            enums,
            messages,
            services,
        })
    }

    pub fn build_units(&self, legacy: bool) -> Result<Vec<ProtoGenUnit>> {
        let all_used_types = self
            .packages
            .values()
            .flat_map(|package| package.used_types.clone())
            .collect::<IntSet<TypeIndex>>();

        let mut units = Vec::new();

        let filtered: Vec<_> = self
            .packages
            .values()
            .filter(move |package| {
                let ns = &package.package_name;
                let keep_always = ns.starts_with("Takasho.Schema.") || ns == "Google.Rpc";
                (keep_always
                    || (!ns.starts_with("Google.")
                        && !package.contained_types.is_disjoint(&all_used_types)))
                    && !package.is_empty()
            })
            .collect();

        for package in filtered {
            let mut unit = ProtoGenUnit {
                namespace: package.package_name.clone(),
                header_comments: package.header_comments.clone(),
                imports: HashSet::new(),
                messages: Vec::new(),
                enums: Vec::new(),
                services: Vec::new(),
            };

            for en in &package.enums {
                let content = en.to_pretty_string(0);
                unit.enums.push(content);
            }

            if let Some(msg_groups) = &package.msg_groups {
                for msg_group in msg_groups {
                    let content = msg_group.to_pretty_string(0, &package.package_name, legacy);
                    unit.messages.push(content);

                    let imports = msg_group
                        .get_used_types()
                        .difference(&msg_group.get_contained_types())
                        .map(|idx| self.get_formatted_filename(*idx, legacy))
                        .collect::<Result<HashSet<_>, _>>()?;
                    unit.imports.extend(imports);
                }
            }

            for svc in &package.services {
                let content = svc.to_pretty_string(0);
                unit.services.push(content);

                let imports = svc
                    .get_used_types()
                    .iter()
                    .map(|idx| self.get_formatted_filename(*idx, legacy))
                    .collect::<Result<HashSet<_>, _>>()?;
                unit.imports.extend(imports);
            }

            units.push(unit);
        }

        Ok(units)
    }

    fn build_enums_for_package(&self, package: &ProtoPackage, legacy: bool) -> Result<Vec<ProtoGenFile>> {
        let mut files = Vec::new();
        for en in &package.enums {
            let filename = self.get_formatted_filename(en.type_index, legacy)?;
            let content = en.to_pretty_string(0);

            let file = ProtoGenFile::new(
                filename,
                &package.package_name,
                &package.header_comments,
                None,
                &content,
                legacy
            )?;
            files.push(file);
        }
        Ok(files)
    }

    fn build_messages_for_package(&self, package: &ProtoPackage, legacy: bool) -> Result<Vec<ProtoGenFile>> {
        let mut files = Vec::new();
        for msg_group in package.msg_groups.as_ref().unwrap() {
            let filename = self.get_formatted_filename(msg_group.get_primary().type_index, legacy)?;

            let imports = msg_group
                .get_used_types()
                .difference(&msg_group.get_contained_types())
                .map(|idx| self.get_formatted_filename(*idx, legacy))
                .filter(|import_filename| {
                    if let Ok(import_filename) = import_filename {
                        import_filename != &filename
                    } else {
                        false
                    }
                })
                .collect::<Result<HashSet<_>, _>>()?;

            let content = msg_group.to_pretty_string(0, &package.package_name, legacy);
            let file = ProtoGenFile::new(
                filename,
                &package.package_name,
                &package.header_comments,
                Some(imports),
                &content,
                legacy
            )?;
            files.push(file);
        }
        Ok(files)
    }

    fn build_services_for_package(&self, package: &ProtoPackage, legacy: bool) -> Result<Vec<ProtoGenFile>> {
        let mut files = Vec::new();
        for svc in &package.services {
            let filename = self.get_formatted_filename(svc.type_index, legacy)?;

            let imports = svc
                .get_used_types()
                .iter()
                .map(|idx| self.get_formatted_filename(*idx, legacy))
                .filter(|import_filename| {
                    if let Ok(import_filename) = import_filename {
                        import_filename != &filename
                    } else {
                        false
                    }
                })
                .collect::<Result<HashSet<_>, _>>()?;

            let content = svc.to_pretty_string(0);
            let file = ProtoGenFile::new(
                filename,
                &package.package_name,
                &package.header_comments,
                Some(imports),
                &content,
                legacy
            )?;
            files.push(file);
        }
        Ok(files)
    }

    fn get_formatted_filename(&self, type_index: TypeIndex, legacy: bool) -> Result<String> {
        let filename = self
            .type_file_mapping
            .get(&type_index)
            .ok_or_else(|| anyhow!("Missing type index in schema mapping for {}", type_index))?;

        let filename = Self::remap_builtin_filenames(filename, legacy);

        Ok(filename)
    }

    fn remap_builtin_filenames(namespace: &str, legacy: bool) -> String {
        if let Some(remaining) = namespace.strip_prefix("Google.Protobuf.WellKnownTypes.") {
            let last_seg = remaining.rsplit('.').next().unwrap();
            let lower = last_seg.to_ascii_lowercase();
            return format!("google/protobuf/{}.proto", lower);
        }
        format_package_filename(namespace, legacy)
    }
}
