import { Outlet } from 'react-router';
import Nav from '@/views/Layout/Nav';
export default function MainLayout() {
  return (
    <div className="flex h-dvh w-dvw flex-col bg-zinc-900 text-white">
      <div className="flex w-full justify-between border-b border-zinc-700 bg-zinc-900 py-2 px-4">
        <h1>PTCGP Toolkit</h1>
        <Nav />
      </div>
      <div className="m-2 text-xs flex-1 overflow-auto rounded-xl border border-zinc-700 bg-zinc-800/50 p-2">
        <Outlet />
      </div>
    </div>
  );
}
