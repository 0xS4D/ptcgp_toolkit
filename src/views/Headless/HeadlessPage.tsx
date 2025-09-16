import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Separator } from '@/components/ui/separator';

export default function HeadlessPage() {
  return (
    <div className="flex h-full flex-col gap-6 p-6">
      <Card>
        <CardHeader>
          <CardTitle>Headless Client Playground</CardTitle>
          <p className="text-muted-foreground text-xs">Someday I will share this…</p>
        </CardHeader>
        <CardContent className="flex flex-col gap-6">
          {/* Load protos */}
          <div className="flex items-end gap-3">
            <div className="flex-1">
              <Label className="mb-1 text-xs">Proto Path</Label>
              <Input type="text" placeholder="Path Protos…" className="text-xs" disabled />
            </div>
            <Button size="sm" className="text-xs" disabled>
              Load Protos
            </Button>
          </div>

          <Separator />

          {/* Login with Device Data */}
          <div className="space-y-3">
            <p className="text-sm font-medium opacity-70">Login with Device Data</p>
            <div className="grid grid-cols-1 gap-2 md:grid-cols-2">
              <Input placeholder="deviceAccount" className="text-xs" disabled />
              <Input placeholder="id" className="text-xs" disabled />
              <Input placeholder="password" className="text-xs" disabled />
              <Input placeholder="identifier" className="text-xs" disabled />
            </div>
            <div className="flex justify-between">
              <Button className="w-full text-xs md:w-auto" disabled>
                Login with Device Data
              </Button>
              <Button className="w-full text-xs md:w-auto" disabled>
                Login with N!(shh)ntend0 Account
              </Button>
            </div>
          </div>

          <Separator />

          {/* Player actions */}
          <div>
            <p className="mb-2 text-sm font-medium opacity-70">Player Actions</p>
            <div className="flex flex-wrap gap-2">
              <Button size="sm" variant="secondary" className="text-xs" disabled>
                Player Info
              </Button>
              <Button size="sm" variant="secondary" className="text-xs" disabled>
                Album
              </Button>
              <Button size="sm" variant="secondary" className="text-xs" disabled>
                Open Packs
              </Button>
              <Button size="sm" variant="secondary" className="text-xs" disabled>
                Wonder Picks
              </Button>
              <Button size="sm" variant="secondary" className="text-xs" disabled>
                Trade
              </Button>
            </div>
          </div>
        </CardContent>
      </Card>
      <div className="flex h-screen w-full overflow-hidden border p-1 rounded-md">
        {/* Lado izquierdo con la captura */}
        <div className="flex w-1/2 items-center justify-center">
          <img src="/screenshot.png" alt="screenshot" className="h-full w-full object-cover" />
        </div>

        {/* Lado derecho con el JSON */}
        <div className="w-1/2 overflow-hidden bg-zinc-900 p-4 text-zinc-600">
          <pre className="text-sm break-words whitespace-pre-wrap">
            {`[{"cardId":"PK_10_000010_00","cardAmount":1,"languageTags":[{"lang":7,"amount":1}],"expansionIds":["A1"],"firstReceivedAt":"2025-09-16T10:47:22.712Z","lastReceivedAt":"2025-09-16T10:47:22.712Z"},{"cardId":"PK_10_000510_00","cardAmount":1,"languageTags":[{"lang":7,"amount":1}],"expansionIds":["A1"],"firstReceivedAt":"2025-09-16T10:50:23.232Z","lastReceivedAt":"2025-09-16T10:50:23.232Z"},{"cardId":"PK_10_000660_00","cardAmount":1,"languageTags":[{"lang":7,"amount":1}],"expansionIds":["A1"],"firstReceivedAt":"2025-09-16T10:47:22.712Z","lastReceivedAt":"2025-09-16T10:47:22.712Z"},{"cardId":"PK_10_001330_00","cardAmount":1,"languageTags":[{"lang":7,"amount":1}],"expansionIds":["A1"],"firstReceivedAt":"2025-09-1:1,"languageTags":[{"lang":7,"amount":1}],"expansionIds":["A1"],"firstReceivedAt":"2025-09-16T10:47:22.712Z","lastReceivedAt":"2025-09-16T10:47:1,"languageTags":[{"lang":7,"amount":1}],"expansionIds":["A1"],"firstReceivedAt":"2025-09-16T10:47:22.712Z","lastReceivedAt":"2025-09-16T10:47:22.712Z"},{"cardId":"PK_10_001330_00","cardAmount":1,"languageTags":[{"lang":7,"amount":1}],"expansionIds":["A1"],"firstReceivedAt":"2025-09-1:1,"languageTags":[{"lang":7,"amount":1}],"expansionIds":["A1"],"firstReceivedAt":"2025-09-16T10:47:22.712Z","lastReceivedAt":"2025-09-16T10:47:1,"languageTags":[{"lang":7,"amount":1}],"expansionIds":["A1"],"firstReceivedAt":"2025-09-16T10:47:22.712Z","lastReceivedAt":"2025-09-16T10:47:1,"languageTags":[{"lang":7,"amount":1}],"expansionIds":["A1"],"firstReceivedAt":"2025-09-16T10:47:22.712Z","lastReceivedAt":"2025-09-16T10:47:22.712Z"},{"cardId":"PK_10_001330_00","cardAmount":1,"languageTags":[{"lang":7,"amount":1}],"expansionIds":["A1"],"firstReceivedAt":"2025-09-1:1,"languageTags":[{"lang":7,"amount":1}],"expansionIds":["A1"],"firstReceivedAt":"2025-09-16T10:47:22.712Z","lastReceivedAt":"2025-09-16T10:47:1,"languageTags":[{"lang":7,"amount":1}],"expansionIds":["A1"],"firstReceivedAt":"2025-09-16T10:47:22.712Z","lastReceivedAt":"2025-09-16T10:47:1,"languageTags":[{"lang":7,"amount":1}],"expansionIds":["A1"],"firstReceivedAt":"2025-09-16T10:47:22.712Z","lastReceivedAt":"2025-09-16T10:47:22.712Z"},{"cardId":"PK_10_001330_00",9-16T10:47:22.712Z"},{"cardId":"PK_10_001570_00","cardAmount":1,"languageTags":[{"lang":7,"amount":1}],"expansionIds":["A1"],"firstReceivedAt":"2025-09-16T10:50:23.232Z","lastReceivedAt":"2025-09-16T10:50:23.232Z"},{"cardId":"PK_10_001640_00","cardAmount":1,"languageTags":[{"lang":7,"amount":1}],"expansionIds":["A1"],"firstReceivedAt":"2025-09-16T10:50:23.232Z","lastReceivedAt":"2025-09-16T10:50:23.232Z"},{"cardId":"PK_10_001830_00","cardAmount":1,"languageTags":[{"lang":7,"amount":1}],"expansionIds":["A1"],"firstReceivedAt":"2025-09-16T10:47:22.712Z","lastReceivedAt":"2025-09-16T10:47:22.712Z"},{"cardId":"PK_20_000010_00","cardAmount":1,"languageTags":[{"lang":7,"amount":1}],"expansionIds":["A1"],"firstReceivedAt":"2025-09-16T10:52:01.572Z","lastReceivedAt":"2025-09-16T10:52:01.572Z"},{"cardId":"PK_20_001850_00","cardAmount":1,"languageTags":[{"lang":7,"amount":1}],"expansionIds":["A1"],"firstReceivedAt":"2025-09-16T10:50:23.232Z","lastReceivedAt":"2025-09-16T10:50:23.232Z"}]`}
          </pre>
        </div>
      </div>
    </div>
  );
}
