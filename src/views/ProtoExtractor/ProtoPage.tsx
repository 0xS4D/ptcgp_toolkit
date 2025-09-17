import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Switch } from '@/components/ui/switch';

export default function ProtoPage() {
  const [libil2cppPath, setLibil2cppPath] = useState('');
  const [metadataPath, setMetadataPath] = useState('');
  const [protosPath, setProtosPath] = useState('');
  const [blacklistText, setBlacklistText] = useState('Lettuce.');
  const [logs, setLogs] = useState<string[]>([]);
  const [isGenerating, setIsGenerating] = useState(false);
  const [legacyMode, setLegacyMode] = useState(false);

  const addLog = (message: string) => {
    setLogs((prev) => [...prev, `[${new Date().toLocaleTimeString()}] ${message}`]);
  };

  async function handleSelectLibil2cppPath() {
    const path = await open({
      filters: [{ name: 'SO', extensions: ['so'] }],
      multiple: false,
    });
    if (path) {
      setLibil2cppPath(path);
      addLog(`Selected libil2cpp.so: ${path}`);
    }
  }

  async function handleSelectMetadataPath() {
    const path = await open({
      filters: [{ name: 'DAT', extensions: ['dat'] }],
      multiple: false,
    });
    if (path) {
      setMetadataPath(path);
      addLog(`Selected metadata: ${path}`);
    }
  }

  async function handleSelectProtosPath() {
    const path = await open({
      directory: true,
      multiple: false,
    });
    if (path) {
      setProtosPath(path);
      addLog(`Selected output directory: ${path}`);
    }
  }

  async function handleGenerateProtos() {
    if (!libil2cppPath || !metadataPath || !protosPath) {
      addLog('Error: Please select all required files and paths');
      return;
    }

    setIsGenerating(true);
    const blacklist = blacklistText
      .trim()
      .split(',')
      .map((s) => s.trim())
      .filter((s) => s);

    addLog('Starting proto generation...');
    if (!legacyMode) {
      addLog(`Using blacklist: [${blacklist.join(', ')}]`);
    }

    try {
      addLog('Loading IL2CPP metadata and analyzing types...');
      await invoke('generate_protos', {
        libil2cppPath: libil2cppPath,
        metadataPath: metadataPath,
        outputDir: protosPath,
        blacklist: blacklist,
        legacy: legacyMode,
      });
      addLog('✓ Done - Proto files generated successfully');
    } catch (error) {
      addLog(`✗ Error generating protos: ${error}`);
    } finally {
      setIsGenerating(false);
    }
  }

  return (
    <div className="flex h-full flex-col gap-4 p-4">
      <Card>
        <CardHeader>
          <div className="flex justify-between">
            <div>
              <CardTitle>Generate Proto Files</CardTitle>
              <p className="text-muted-foreground text-xs">
                Export protobuf schema files from IL2CPP metadata
              </p>
            </div>
            <div className="flex flex-col items-end gap-2">
              <Label htmlFor="legacy">Legacy Mode</Label>
              <div className="flex items-center gap-2">
                {legacyMode ? 'Legacy' : 'New'}
                <Switch id="legacy" checked={legacyMode} onCheckedChange={setLegacyMode} />
              </div>
            </div>
          </div>
        </CardHeader>
        <CardContent className="flex flex-col gap-3">
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
            <Label className="mb-1 text-xs">Global Metadata (Decrypted)</Label>
            <Input
              value={metadataPath}
              placeholder="Click to select decrypted .dat file"
              onClick={handleSelectMetadataPath}
              readOnly
              className="cursor-pointer text-xs"
            />
          </div>

          <div>
            <Label className="mb-1 text-xs">Output Directory</Label>
            <Input
              value={protosPath}
              placeholder="Click to select output directory"
              onClick={handleSelectProtosPath}
              readOnly
              className="cursor-pointer text-xs"
            />
          </div>

          <div>
            <Label className="mb-1 text-xs">Blacklist (comma-separated prefixes to exclude)</Label>
            <Input
              value={blacklistText}
              onChange={(e) => setBlacklistText(e.target.value)}
              placeholder="e.g. Lettuce., Google.Rpc"
              className="text-xs"
              disabled={legacyMode}
            />
            <p className="text-muted-foreground mt-1 text-xs">
              Namespaces starting with these prefixes will be excluded from generation
            </p>
          </div>

          <Button
            onClick={handleGenerateProtos}
            disabled={isGenerating || !libil2cppPath || !metadataPath || !protosPath}
            size="sm"
            className="text-xs"
          >
            {isGenerating ? '⏳ Generating...' : '⚡ Generate Protos'}
          </Button>
        </CardContent>
      </Card>

      {/* Logs */}
      {logs.length > 0 && (
        <Card className="flex min-h-0 flex-1 flex-col">
          <div className="flex items-center justify-between border-b px-3 py-2">
            <span className="text-sm font-medium">Process Log</span>
            <Button variant="outline" size="sm" onClick={() => setLogs([])} className="text-xs">
              Clear
            </Button>
          </div>
          <CardContent className="min-h-0 flex-1 p-0">
            <div className="flex h-full flex-col">
              <div className="min-h-0 flex-1 overflow-y-auto rounded bg-zinc-900 p-2">
                {logs.map((log, i) => (
                  <div key={i} className="py-0.5 font-mono text-xs text-gray-50">
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
