import { Outlet } from 'react-router';
import Titlebar from './title-bar';
import Sidebar from './sidebar';

export default function Layout() {
  return (
    <div className='flex h-screen w-screen overflow-hidden bg-[#0b0b0d]'>
      <Sidebar />
      <div className='flex min-w-0 flex-1 flex-col'>
        <Titlebar />
        <main className='min-h-0 flex-1 overflow-auto'>
          <Outlet />
        </main>
      </div>
    </div>
  );
}
