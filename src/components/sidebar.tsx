import { useState } from 'react';
import { NavLink } from 'react-router';
import { motion } from 'motion/react';
import { HugeiconsIcon } from '@hugeicons/react';
import { PanelLeftCloseIcon, PanelLeftIcon } from '@hugeicons/core-free-icons';

const navItems = [
  { to: '/', label: 'Dashboard', icon: DashboardIcon },
  { to: '/models', label: 'Models', icon: ModelIcon },
  { to: '/shortcuts', label: 'Shortcuts', icon: ShortcutIcon },
  { to: '/settings', label: 'Settings', icon: SettingsIcon },
];

export default function Sidebar() {
  const [collapsed, setCollapsed] = useState(false);

  return (
    <motion.aside
      animate={{ width: collapsed ? 56 : 200 }}
      transition={{ type: 'spring', stiffness: 300, damping: 30 }}
      className='flex h-full shrink-0 flex-col border-r border-white/8 bg-[#0b0b0d]'
    >
      <button
        type='button'
        onClick={() => setCollapsed(!collapsed)}
        className='relative z-50 flex h-16 shrink-0 items-center justify-start gap-3 overflow-hidden border-b border-white/8 px-5 text-white/50 transition hover:text-white'
        aria-label={collapsed ? 'Expand sidebar' : 'Collapse sidebar'}
      >
        {!collapsed && (
          <img
            src='/bg.jpg'
            alt='gradient-bg'
            className='absolute inset-0 h-full w-full mask-x-from-95% object-cover opacity-50'
          />
        )}

        <motion.svg
          viewBox='0 0 20 20'
          fill='currentColor'
          className='size-5'
          animate={{ rotate: collapsed ? 180 : 0 }}
          transition={{ type: 'spring', stiffness: 300, damping: 25 }}
        >
          {!collapsed ? (
            <HugeiconsIcon
              icon={PanelLeftCloseIcon}
              size={20}
              color='#ffffff'
              strokeWidth={1.8}
            />
          ) : (
            <HugeiconsIcon
              icon={PanelLeftIcon}
              size={20}
              color='#ffffff'
              strokeWidth={1.8}
            />
          )}
        </motion.svg>

        {!collapsed && (
          <p className='z-50 text-sm font-medium text-white/90'>Lucid</p>
        )}
      </button>

      <nav className='flex flex-1 flex-col gap-1 p-2'>
        {navItems.map((item) => (
          <NavLink
            key={item.to}
            to={item.to}
            end={item.to === '/'}
            className={({ isActive }) =>
              `group relative flex h-9 items-center gap-3 rounded-lg px-2.5 text-sm font-medium transition ${
                isActive
                  ? 'bg-white/10 text-white'
                  : 'text-white/50 hover:bg-white/5 hover:text-white'
              } ${collapsed ? 'justify-center' : ''}`
            }
            title={collapsed ? item.label : undefined}
          >
            {({ isActive }) => (
              <>
                {isActive && (
                  <motion.div
                    layoutId='sidebar-active'
                    className='absolute inset-0 rounded-lg bg-white/10'
                    transition={{ type: 'spring', stiffness: 400, damping: 30 }}
                  />
                )}
                <item.icon className='relative z-10 size-4 shrink-0' />
                {!collapsed && (
                  <motion.span
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    exit={{ opacity: 0 }}
                    className='relative z-10'
                  >
                    {item.label}
                  </motion.span>
                )}
              </>
            )}
          </NavLink>
        ))}
      </nav>
    </motion.aside>
  );
}

function DashboardIcon({ className }: { className?: string }) {
  return (
    <svg
      viewBox='0 0 24 24'
      fill='none'
      stroke='currentColor'
      strokeWidth='1.8'
      className={className}
    >
      <rect x='3' y='3' width='7' height='9' rx='1.5' />
      <rect x='14' y='3' width='7' height='5' rx='1.5' />
      <rect x='14' y='12' width='7' height='9' rx='1.5' />
      <rect x='3' y='16' width='7' height='5' rx='1.5' />
    </svg>
  );
}

function ModelIcon({ className }: { className?: string }) {
  return (
    <svg
      viewBox='0 0 24 24'
      fill='none'
      stroke='currentColor'
      strokeWidth='1.8'
      className={className}
    >
      <circle cx='12' cy='12' r='3' />
      <path strokeLinecap='round' d='M12 2v4m0 12v4M2 12h4m12 0h4' />
      <path
        strokeLinecap='round'
        d='M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83'
      />
    </svg>
  );
}

function ShortcutIcon({ className }: { className?: string }) {
  return (
    <svg
      viewBox='0 0 24 24'
      fill='none'
      stroke='currentColor'
      strokeWidth='1.8'
      className={className}
    >
      <rect x='2' y='6' width='20' height='12' rx='2' />
      <rect x='5' y='9' width='3' height='2' rx='0.5' fill='currentColor' />
      <rect x='10' y='9' width='3' height='2' rx='0.5' fill='currentColor' />
      <rect x='15' y='9' width='4' height='2' rx='0.5' fill='currentColor' />
      <rect x='5' y='13' width='14' height='2' rx='0.5' fill='currentColor' />
    </svg>
  );
}

function SettingsIcon({ className }: { className?: string }) {
  return (
    <svg
      viewBox='0 0 24 24'
      fill='none'
      stroke='currentColor'
      strokeWidth='1.8'
      className={className}
    >
      <circle cx='12' cy='12' r='3' />
      <path d='M19.4 15a1.65 1.65 0 00.33 1.82l.06.06a2 2 0 01-2.83 2.83l-.06-.06a1.65 1.65 0 00-1.82-.33 1.65 1.65 0 00-1 1.51V21a2 2 0 01-4 0v-.09A1.65 1.65 0 009 19.4a1.65 1.65 0 00-1.82.33l-.06.06a2 2 0 01-2.83-2.83l.06-.06A1.65 1.65 0 004.68 15a1.65 1.65 0 00-1.51-1H3a2 2 0 010-4h.09A1.65 1.65 0 004.6 9a1.65 1.65 0 00-.33-1.82l-.06-.06a2 2 0 012.83-2.83l.06.06A1.65 1.65 0 009 4.68a1.65 1.65 0 001-1.51V3a2 2 0 014 0v.09a1.65 1.65 0 001 1.51 1.65 1.65 0 001.82-.33l.06-.06a2 2 0 012.83 2.83l-.06.06A1.65 1.65 0 0019.4 9a1.65 1.65 0 001.51 1H21a2 2 0 010 4h-.09a1.65 1.65 0 00-1.51 1z' />
    </svg>
  );
}
