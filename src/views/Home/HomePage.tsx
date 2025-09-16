import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';

export default function HomePage() {
  return (
    <div className="flex h-full flex-col items-center justify-center p-6">
      <Card className="w-full max-w-md text-center">
        <CardHeader>
          <CardTitle className="text-2xl font-bold">Welcome to PTCGP Toolkit</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <p className="text-muted-foreground text-sm">
            This is a small toolkit for working with <code>.apks</code>,<code>libil2cpp.so</code>,
            and <code>global-metadata.dat</code>.
          </p>

          <p className="text-muted-foreground text-sm italic">
            Thanks to{' '}
            <a href="https://github.com/UnknownCollections" className="font-semibold">
              UnknownCollections
            </a>{' '}
            for the awesome work on ptcgp_tool and make everything possible ✨
          </p>

          <div className="flex justify-center gap-3 pt-2">
            <Button asChild variant="default" size="sm" className="text-xs font-medium">
              <a href="https://github.com/0xs4d" target="_blank">
                Visit GitHub
              </a>
            </Button>
            <Button asChild variant="secondary" size="sm" className="text-xs font-medium">
              <a href="/apks">Get Started</a>
            </Button>
          </div>
          <p className="text-muted-foreground text-xs">
            Made with ❤️ by <span className="font-mono">@0xs4d</span>
          </p>
        </CardContent>
      </Card>
    </div>
  );
}
