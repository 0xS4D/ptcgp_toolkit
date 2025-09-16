import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

export default function GMetaPage() {
  const [apksPath, setApksPath] = useState("");
  const [extractPath, setExtractPath] = useState("");
  const [libil2cppPath, setLibil2cppPath] = useState("");
  const [encryptedMetadataPath, setEncryptedMetadataPath] = useState("");
  const [decryptedMetadataPath, setDecryptedMetadataPath] = useState("");
  const [logs, setLogs] = useState<string[]>([]);
  const [isExtracting, setIsExtracting] = useState(false);
  const [isDecrypting, setIsDecrypting] = useState(false);

  const addLog = (message: string) => {
    setLogs((prev) => [
      ...prev,
      `[${new Date().toLocaleTimeString()}] ${message}`,
    ]);
  };

  async function handleSelectApksPath() {
    const path = await open({
      filters: [{ name: "APKS", extensions: ["apks"] }],
      multiple: false,
    });
    if (path) {
      setApksPath(path);
      addLog(`Selected APKS file: ${path}`);
    }
  }

  async function handleSelectExtractPath() {
    const path = await open({
      directory: true,
      multiple: false,
    });
    if (path) {
      setExtractPath(path);
      addLog(`Selected extract directory: ${path}`);
    }
  }

  async function handleSelectLibil2cppPath() {
    const path = await open({
      filters: [{ name: "SO", extensions: ["so"] }],
      multiple: false,
    });
    if (path) {
      setLibil2cppPath(path);
      addLog(`Selected libil2cpp.so: ${path}`);
    }
  }

  async function handleSelectEncryptedMetadataPath() {
    const path = await open({
      filters: [{ name: "DAT", extensions: ["dat"] }],
      multiple: false,
    });
    if (path) {
      setEncryptedMetadataPath(path);
      addLog(`Selected encrypted metadata: ${path}`);
    }
  }

  async function handleSelectDecryptedMetadataPath() {
    const path = await open({
      directory: true,
      multiple: false,
    });
    if (path) {
      setDecryptedMetadataPath(path + "/global-metadata.dat");
      addLog(`Selected output directory: ${path}`);
    }
  }

  async function handleExtractFromApks() {
    if (!apksPath || !extractPath) {
      addLog("Error: Please select APKS file and extract path");
      return;
    }

    setIsExtracting(true);
    addLog("Starting extraction from APKS...");

    try {
      addLog("Extracting global-metadata.dat from base.apk...");
      await invoke("extract_from_apks", {
        apksPath: apksPath,
        innerZipName: "base.apk",
        innerFilePath: "assets/bin/Data/Managed/Metadata/global-metadata.dat",
        outputPath: extractPath + "/global-metadata.dat",
      });
      addLog("✓ Extracted global-metadata.dat");

      addLog("Extracting libil2cpp.so from split_config.arm64_v8a.apk...");
      await invoke("extract_from_apks", {
        apksPath: apksPath,
        innerZipName: "split_config.arm64_v8a.apk",
        innerFilePath: "lib/arm64-v8a/libil2cpp.so",
        outputPath: extractPath + "/libil2cpp.so",
      });
      addLog("✓ Extracted libil2cpp.so");
      addLog("✓ Done - All files extracted successfully");
    } catch (error) {
      addLog(`✗ Error during extraction: ${error}`);
    } finally {
      setIsExtracting(false);
    }
  }

  async function handleDecryptMetadata() {
    if (!libil2cppPath || !encryptedMetadataPath || !decryptedMetadataPath) {
      addLog("Error: Please select all required files and paths");
      return;
    }

    setIsDecrypting(true);
    addLog("Starting metadata decryption...");

    try {
      addLog("Analyzing ELF file and extracting keys...");
      await invoke("decrypt_metadata", {
        libil2cppPath: libil2cppPath,
        encryptedMetadataPath: encryptedMetadataPath,
        outputPath: decryptedMetadataPath,
      });
      addLog("✓ Done - Metadata decrypted successfully");
    } catch (error) {
      addLog(`✗ Error during decryption: ${error}`);
    } finally {
      setIsDecrypting(false);
    }
  }

  return (
    <div className="flex h-full flex-col gap-4 p-4">
      <div className="grid grid-cols-2 gap-6 ">
        {/* Extract from APKS */}
        <Card>
          <CardHeader>
            <CardTitle>Extract from APKS</CardTitle>
            <p className="text-xs text-muted-foreground">
              Extract libil2cpp.so & global-metadata.dat (encrypted)
            </p>
          </CardHeader>
          <CardContent className="flex flex-col gap-2">
            <div>
              <Label className="mb-1 text-xs">APKS File</Label>
              <Input
                value={apksPath}
                placeholder="Click to select .apks file"
                onClick={handleSelectApksPath}
                readOnly
                className="cursor-pointer text-xs"
              />
            </div>

            <div>
              <Label className="mb-1 text-xs">Extract Directory</Label>
              <Input
                value={extractPath}
                placeholder="Click to select output directory"
                onClick={handleSelectExtractPath}
                readOnly
                className="cursor-pointer text-xs"
              />
            </div>

            <Button
              onClick={handleExtractFromApks}
              disabled={isExtracting || !apksPath || !extractPath}
              size="sm"
              className="text-xs"
            >
              {isExtracting ? "⏳ Extracting..." : "📤 Extract Files"}
            </Button>
          </CardContent>
        </Card>

        {/* Decrypt Metadata */}
        <Card>
          <CardHeader>
            <CardTitle>Decrypt Metadata</CardTitle>
            <p className="text-xs text-muted-foreground">
              Decrypt global-metadata.dat using libil2cpp.so
            </p>
          </CardHeader>
          <CardContent className="flex flex-col gap-2">
            <div>
              <Label className="mb-1 text-xs">libil2cpp.so File</Label>
              <Input
                value={libil2cppPath}
                placeholder="Click to select libil2cpp.so"
                onClick={handleSelectLibil2cppPath}
                readOnly
                className="cursor-pointer text-xs"
              />
            </div>

            <div>
              <Label className="mb-1 text-xs">Encrypted Metadata</Label>
              <Input
                value={encryptedMetadataPath}
                placeholder="Click to select encrypted .dat file"
                onClick={handleSelectEncryptedMetadataPath}
                readOnly
                className="cursor-pointer text-xs"
              />
            </div>

            <div>
              <Label className="mb-1 text-xs">Output Directory</Label>
              <Input
                value={decryptedMetadataPath}
                placeholder="Click to select output directory"
                onClick={handleSelectDecryptedMetadataPath}
                readOnly
                className="cursor-pointer text-xs"
              />
            </div>

            <Button
              onClick={handleDecryptMetadata}
              disabled={
                isDecrypting ||
                !libil2cppPath ||
                !encryptedMetadataPath ||
                !decryptedMetadataPath
              }
              size="sm"
              className="text-xs"
            >
              {isDecrypting ? "⏳ Decrypting..." : "🔓 Decrypt Metadata"}
            </Button>
          </CardContent>
        </Card>
      </div>

      {/* Logs */}
      {logs.length > 0 && (
        <Card className="flex min-h-0 flex-1 flex-col">
          <div className="flex items-center justify-between border-b px-3 py-2">
            <span className="text-sm font-medium">Process Log</span>
            <Button
              variant="outline"
              size="sm"
              onClick={() => setLogs([])}
              className="text-xs"
            >
              Clear
            </Button>
          </div>
          <CardContent className="flex-1 min-h-0 p-0">
            <div className="flex h-full flex-col">
              <div className="flex-1 min-h-0 overflow-y-auto rounded bg-zinc-900 p-2">
                {logs.map((log, i) => (
                  <div
                    key={i}
                    className="py-0.5 font-mono text-xs text-gray-50"
                  >
                    {log}
                  </div>
                ))}
              </div>
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
}