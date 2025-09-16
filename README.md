# PTCGP Toolkit

A Tauri-based desktop application for extracting and analyzing APKS files, specifically designed for PTCG data extraction and proto file generation.


> ⚠️ **DISCLAIMER:**  
> This project is for educational purposes. Please respect game terms of service and use responsibly.  
> I'm not attempting to create private servers or exploit the game maliciously.

## What’s the difference with ptcg_tool?
- I split the extraction responsibilities into different functions. If you only need the decrypted global-metadata.dat, you can now get it directly and use it on il2cppinspectorredux or other tools.
- I added a tab to extract the latest APKS from devices. 
- I improved the handling of proto schema generation. Instead of generating many folders. This generates all the files in a single folder.
- Making it easier to edit the namespaces or imports in the rust code. 
- I’ll be adding an option to update the different Unity versions in a future.

## Features

### 🛠️ APK Extraction from Device
- Connect to Android devices via ADB
- Extract APK files directly from connected devices
- Automatic device detection and selection

### 📱 APKS File Processing
- Extract specific files from APKS archives
- Support for nested ZIP extraction (base.apk, split configs)
- Extract `libil2cpp.so` and `global-metadata.dat` files
- Automatic directory creation for output files

### 🔓 Metadata Decryption
- Decrypt encrypted `global-metadata.dat` files
- ELF analysis for key extraction from `libil2cpp.so`
- Advanced cryptographic operations (AES-128 CTR mode)
- Support for ARM64 architecture binaries

### ⚡ Proto File Generation
- Generate protobuf schema files from IL2CPP metadata
- Customizable namespace blacklisting
- Support for complex type analysis and schema generation
- Output organized proto files ready for use

## Tech Stack

- **Frontend**: React + TypeScript + Tailwind CSS
- **Backend**: Rust + Tauri


## Project Structure

```
src-tauri/
├─ src/
│  ├─ commands/            # Tauri command handlers
│  ├─ core/                # Core analysis logic
│  │  ├─ src/
│  │  │  ├─ decrypt/       # Metadata decryption
│  │  │  ├─ extract/       # File extraction utilities
│  │  │  ├─ proto/         # Proto schema generation
│  │  │  └─ unity/         # Unity/IL2CPP analysis
│  │  └─ Cargo.toml
│  ├─ lib.rs
│  └─ main.rs
src/
└─ views/
   ├─ Apks/                # APK extraction from device
   ├─ Gmetadata/           # APKS processing & decryption
   └─ ProtoExtractor/      # Proto file generation
```

## Installation & Development

### Prerequisites
- Rust (latest stable)
- Node.js 18+

## Usage

bun install
bun tauri dev

### 1. APK Extraction from Device
1. Connect your Android device via USB with Developer Options enabled
2. Click "Reload Devices" to detect connected devices
3. Select your device from the dropdown
4. Choose an extraction directory
5. Click "Extract APKS" to start the process

### 2. APKS File Processing
1. **Extract Files**: Select your `.apks` file and output directory for libil2cpp.so and global-metadata.dat
2. **Decrypt Metadata**: Use the extracted `libil2cpp.so` and encrypted `global-metadata.dat`
3. Follow the process logs to monitor extraction and decryption progress

### 3. Proto File Generation
1. Select the `libil2cpp.so` file
2. Select the decrypted `global-metadata.dat` file  
3. Choose output directory for proto files
4. Configure blacklist (default: "Lettuce." - comma-separated prefixes)
5. Click "Generate Protos" to create schema files

## TODO:
- [ ] Clean the project. Now it's a mess.
- [ ] Add support for update unity versions.
- [ ] Remove the "Types" on the protos.
- [ ] Release the headless client?

## 🙏 THANKS!
Special thanks to:
- **[UnknownCollections](https://github.com/UnknownCollections/ptcgp_tool)** for the original ptcgp_tool that inspired this project.

---

— Built with ❤️  — _Saludos!_ 👋
