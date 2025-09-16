use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use goblin::elf::Elf;
use beta::{extract_from_apks as core_extract_from_apks, extract_metadata_key_xor, extract_metadata_key, decrypt_metadata as decrypt_metadata_data, generate_proto_schema, Il2Cpp};

#[tauri::command]
pub fn load_devices() -> Vec<String> {
    let adb_path = "adb_files/adb.exe";

    let output = Command::new(adb_path)
        .arg("devices")
        .output()
        .expect("Failed to execute adb");

    let stdout = String::from_utf8_lossy(&output.stdout);

    let devices: Vec<String> = stdout
        .lines()
        .skip(1)
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && parts[1] == "device" {
                Some(parts[0].to_string())
            } else {
                None
            }
        })
        .collect();

    devices
}

#[tauri::command]
pub fn extract_from_device(device: String, extract_path: &str) -> Result<(), String> {
    let adb_path = "adb_files/adb.exe";
    let binding = Path::new(extract_path).join("splitted");
    let output_dir = binding.as_path();
    let output = Command::new(adb_path)
        .args(["-s", &device, "shell", "pm", "path", "jp.pokemon.pokemontcgp"])
        .output()
        .expect("Failed to execute adb.exe");

    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    let stdout = str::from_utf8(&output.stdout).unwrap();

    for line in stdout.lines() {
        if let Some(path) = line.strip_prefix("package:") {
            if let Some(filename) = Path::new(path).file_name() {
                let dest: PathBuf = output_dir.join(filename);

                let status = Command::new(adb_path)
                    .args(["-s", &device, "pull", path, dest.to_str().unwrap()])
                    .status()
                    .expect("adb pull failed");

                if !status.success() {
                    eprintln!("Error extracting {path}");
                }
            }
        }
    }

    let apks_path = Path::new(extract_path).join("PokemonTCGP.apks");

    let _ = create_apks_archive(&output_dir, &apks_path);
    Ok(())
}

fn create_apks_archive(dir: &Path, apks_name: &Path) -> zip::result::ZipResult<()> {
    let file = File::create(apks_name)?;
    let mut zip = zip::ZipWriter::new(file);

    let options: zip::write::SimpleFileOptions =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let name = path.file_name().unwrap().to_string_lossy().into_owned();

            let mut f = File::open(&path)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;

            zip.start_file(name, options)?;
            zip.write_all(&buffer)?;
        }
    }

    zip.finish()?;
    Ok(())
}

#[tauri::command]
pub fn extract_from_apks(apks_path: String, inner_zip_name: String, inner_file_path: String, output_path: String) -> Result<(), String> {
    core_extract_from_apks(&apks_path, &inner_zip_name, &inner_file_path, &output_path);
    Ok(())
}

#[tauri::command]
pub fn decrypt_metadata(libil2cpp_path: String, encrypted_metadata_path: String, output_path: String) -> Result<(), String> {
    let elf_data = fs::read(&libil2cpp_path).map_err(|e| format!("Failed to read libil2cpp.so: {}", e))?;
    let enc_data = fs::read(&encrypted_metadata_path).map_err(|e| format!("Failed to read encrypted metadata: {}", e))?;

    if &elf_data[0..4] != b"\x7fELF" {
        return Err("Not an ELF file".to_string());
    }

    let elf = Elf::parse(&elf_data).map_err(|e| format!("Failed to parse ELF: {}", e))?;

    // Extract key_xor and key from the ELF
    let (key_xor_off, key_xor) = extract_metadata_key_xor(&elf, &elf_data).map_err(|e| format!("Failed to extract key_xor: {}", e))?;
    let key = extract_metadata_key(&elf, &elf_data, key_xor_off).map_err(|e| format!("Failed to extract key: {}", e))?;
    let dec = decrypt_metadata_data(&enc_data, &key, key_xor).map_err(|e| format!("Failed to decrypt metadata: {}", e))?;

    if let Some(parent) = Path::new(&output_path).parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Could not create output directory: {}", e))?;
    }

    fs::write(&output_path, &dec).map_err(|e| format!("Failed to write decrypted metadata: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn generate_protos(libil2cpp_path: String, metadata_path: String, output_dir: String, blacklist: Vec<String>) -> Result<(), String> {
    let libil2cpp_so = fs::read(&libil2cpp_path).map_err(|e| format!("Failed to read libil2cpp.so: {}", e))?;
    let metadata_data = fs::read(&metadata_path).map_err(|e| format!("Failed to read metadata: {}", e))?;

    let il2cpp = Il2Cpp::load_from_vec(libil2cpp_so, metadata_data).map_err(|e| format!("Failed to load IL2CPP: {}", e))?;

    let proto_files = generate_proto_schema(il2cpp).map_err(|e| format!("Failed to generate proto schema: {}", e))?;

    fs::create_dir_all(&output_dir).map_err(|e| format!("Failed to create output directory: {}", e))?;

    let mut files_written = 0;
    for unit in proto_files {
        if blacklist.iter().any(|b| unit.namespace.starts_with(b)) {
            continue;
        }

        let proto_file = unit.render();
        let output_path = format!("{}/{}", output_dir, proto_file.filename);
        fs::write(&output_path, proto_file.source_code).map_err(|e| format!("Failed to write proto file: {}", e))?;
        files_written += 1;
    }

    println!("Generated {} proto files", files_written);
    Ok(())
}
