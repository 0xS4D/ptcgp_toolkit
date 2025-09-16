import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';

export default function ApksPage() {
  const [devices, setDevices] = useState<string[]>([]);
  const [extractPath, setExtractPath] = useState('');
  const [selectDevice, setSelectDevice] = useState('');
  const [logs, setLogs] = useState<string[]>([]);
  const [isLoading, setIsLoading] = useState(false);

  const addLog = (message: string) => {
    setLogs((prev) => [...prev, `[${new Date().toLocaleTimeString()}] ${message}`]);
  };

  async function handleLoadDevices() {
    addLog('Loading devices...');
    try {
      const devices = (await invoke('load_devices')) as string[];
      if (devices.length > 0) {
        setDevices(devices);
        addLog(`Found ${devices.length} device(s)`);
      } else {
        addLog('No devices found');
      }
    } catch (error) {
      addLog(`Error loading devices: ${error}`);
    }
  }

  async function handleSelectExtractPath() {
    const path = await open({
      directory: true,
      multiple: false,
    });
    if (path) {
      setExtractPath(path);
      addLog(`Selected extract path: ${path}`);
    }
  }

  async function handleExtract() {
    if (!selectDevice || !extractPath) {
      addLog('Error: Please select device and extract path');
      return;
    }

    setIsLoading(true);
    addLog('Starting APK extraction from device...');
    try {
      await invoke('extract_from_device', {
        device: selectDevice,
        extractPath: extractPath,
      });
      addLog('✓ Done - APK extraction completed successfully');
    } catch (error) {
      addLog(`✗ Error during extraction: ${error}`);
    } finally {
      setIsLoading(false);
    }
  }

  return (
    <div className="flex h-full flex-col gap-4 p-4">
      <Card>
        <CardHeader>
          <CardTitle>Extract APKS from Device</CardTitle>
        </CardHeader>
        <CardContent className="flex flex-col gap-3">
          <div>
            <Label className="mb-1 text-xs">Android Device</Label>
            <Select onValueChange={setSelectDevice}>
              <SelectTrigger className="w-full text-xs">
                <SelectValue placeholder="Select device" />
              </SelectTrigger>
              <SelectContent>
                {devices.map((device, i) => (
                  <SelectItem key={i} value={device}>
                    {device}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <Button variant="outline" size="sm" onClick={handleLoadDevices} className="text-xs">
            🔄 Reload Devices
          </Button>

          <div>
            <Label className="mb-1 text-xs">Extract Path</Label>
            <Input
              value={extractPath}
              onClick={handleSelectExtractPath}
              placeholder="Click to select extract path"
              readOnly
              className="cursor-pointer text-xs"
            />
          </div>

          <Button
            onClick={handleExtract}
            size="sm"
            disabled={isLoading || !selectDevice || !extractPath}
            className="text-xs"
          >
            {isLoading ? '⏳ Extracting...' : '📦 Extract APKS'}
          </Button>
        </CardContent>
      </Card>

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
