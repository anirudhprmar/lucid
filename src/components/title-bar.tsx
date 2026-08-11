import { getCurrentWindow } from '@tauri-apps/api/window';
import { useLocation } from 'react-router';

const pageTitles: Record<string, string> = {
  '/': 'Dashboard',
  '/models': 'Models',
  '/shortcuts': 'Shortcuts',
  '/settings': 'Settings',
};

export default function Titlebar() {
  const location = useLocation();
  const currentTitle = pageTitles[location.pathname] || 'Lucid';
  const window = getCurrentWindow();

  return (
    <div
      data-tauri-drag-region={true}
      className='flex h-11 shrink-0 items-center justify-between border-b border-white/8 bg-[#0b0b0d] px-4 select-none'
    >
      <div className='flex items-center gap-2' data-tauri-drag-region>
        <svg
          viewBox='0 0 24 24'
          fill='none'
          stroke='currentColor'
          strokeWidth='2'
          className='size-4 text-white/60'
          aria-hidden='true'
        >
          <path
            strokeLinecap='round'
            strokeLinejoin='round'
            d='M12 3v12m0 0 4-4m-4 4-4-4M5 17v2.5A1.5 1.5 0 0 0 6.5 21h11a1.5 1.5 0 0 0 1.5-1.5V17'
          />
        </svg>
        <span className='text-sm font-medium text-white/50'>Lucid</span>
        <span className='text-white/20'>/</span>
        <span className='text-sm font-semibold text-white'>{currentTitle}</span>
      </div>

      <div className='flex items-center gap-1'>
        <button
          type='button'
          onClick={() => window.minimize()}
          className='flex h-8 w-8 items-center justify-center rounded-md text-white/50 transition hover:bg-white/8 hover:text-white'
        >
          <svg viewBox='0 0 16 16' fill='currentColor' className='size-3.5'>
            <rect y='7' width='16' height='2' rx='1' />
          </svg>
        </button>
        <button
          type='button'
          onClick={() => window.toggleMaximize()}
          className='flex h-8 w-8 items-center justify-center rounded-md text-white/50 transition hover:bg-white/8 hover:text-white'
        >
          <svg
            viewBox='0 0 16 16'
            fill='none'
            stroke='currentColor'
            strokeWidth='1.5'
            className='size-3.5'
          >
            <rect x='2.5' y='2.5' width='11' height='11' rx='2' />
          </svg>
        </button>
        <button
          type='button'
          onClick={() => window.hide()}
          className='flex h-8 w-8 items-center justify-center rounded-md text-white/50 transition hover:bg-red-500/80 hover:text-white'
        >
          <svg
            viewBox='0 0 16 16'
            fill='none'
            stroke='currentColor'
            strokeWidth='1.8'
            className='size-3.5'
          >
            <path strokeLinecap='round' d='M4 4l8 8M12 4l-8 8' />
          </svg>
        </button>
      </div>
    </div>
  );
}
