use std::fs::{self, File};
use std::io::{Cursor, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use zip::ZipArchive;

pub fn extract_from_apks(
    apks_path: &str,
    inner_zip_name: &str,
    inner_file_path: &str,
    output_path: &str,
) {
    let apks_file = File::open(apks_path).expect("Could not open the .apks file");
    let mut apks_archive = ZipArchive::new(apks_file).expect("Error reading the .apks file");

    let mut inner_bytes = Vec::new();
    apks_archive
        .by_name(inner_zip_name)
        .expect("Inner file not found")
        .read_to_end(&mut inner_bytes)
        .expect("Error reading the inner file");

    let cursor = Cursor::new(inner_bytes);
    let mut inner_archive = ZipArchive::new(cursor).expect("Error reading the inner file");

    let mut inner_file = inner_archive
        .by_name(inner_file_path)
        .expect("Inner file not found inside the zip");

    let mut output = File::create(output_path).expect("Could not create the output file");

    std::io::copy(&mut inner_file, &mut output).expect("Error copying the file");

    println!("Extracted: {}", output_path);
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

pub fn extract_from_device(adb_path: &str, package: &str, apks_name: &str, working_dir: &str) {
    let binding = Path::new(working_dir).join("splitted");
    let output_dir = binding.as_path();

    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    let output = Command::new(adb_path)
        .args(["shell", "pm", "path", package])
        .output()
        .expect("Failed to execute adb.exe");

    let stdout = str::from_utf8(&output.stdout).unwrap();

    for line in stdout.lines() {
        if let Some(path) = line.strip_prefix("package:") {
            if let Some(filename) = Path::new(path).file_name() {
                let dest: PathBuf = output_dir.join(filename);

                let status = Command::new(adb_path)
                    .args(["pull", path, dest.to_str().unwrap()])
                    .status()
                    .expect("adb pull failed");

                if !status.success() {
                    eprintln!("Error extracting {path}");
                }
            }
        }
    }

    let apks_path = Path::new(working_dir).join(apks_name);

    if let Err(e) = create_apks_archive(&output_dir, &apks_path) {
        eprintln!("Error creating {}: {e}", apks_name);
    }
}
