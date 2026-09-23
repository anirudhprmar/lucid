import { AnimatePresence, motion } from 'motion/react';
import { useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event';
import Waveform from './waveform';
import Spinner from './spinner';
import FloatingPill from './floating-pill';

type NotchState = 'idle' | 'listening' | 'transcribing' | 'not-ready';

export default function Notch() {
  const [state, setState] = useState<NotchState>('idle');

  useEffect(() => {
    document.documentElement.style.background = 'transparent';
    document.body.style.background = 'transparent';
    document.body.style.overflow = 'hidden';
  }, []);

  useEffect(() => {
    const unlisten = listen<NotchState>('notch-state', (e) => {
      setState(e.payload);
    });
    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  const isActive = state !== 'idle';

  return (
    <div className='flex h-full w-full items-center justify-center p-1'>
      {/* keep store tracker mounted */}
      <FloatingPill />
      <AnimatePresence>
        {isActive && (
          <motion.div
            key='dictation-pill'
            layout
            className='flex items-center justify-center rounded-full bg-black shadow-lg shadow-black/30'
            style={{ height: 36 }}
            initial={{ y: 44, opacity: 0, scale: 0.85, width: 40 }}
            animate={{
              y: 0,
              opacity: 1,
              scale: 1,
              width:
                state === 'listening'
                  ? 118
                  : state === 'transcribing'
                    ? 72
                    : 60,
            }}
            exit={{ y: 36, opacity: 0, scale: 0.85, filter: 'blur(4px)' }}
            transition={{
              y: { type: 'spring', stiffness: 340, damping: 26, mass: 0.8 },
              opacity: { duration: 0.25 },
              scale: { type: 'spring', stiffness: 340, damping: 26 },
              width: { type: 'spring', stiffness: 320, damping: 28 },
              filter: { duration: 0.2 },
            }}
          >
            <AnimatePresence mode='wait'>
              {state === 'listening' && (
                <motion.div
                  key='listening'
                  initial={{ opacity: 0, scale: 0.7, y: 6 }}
                  animate={{ opacity: 1, scale: 1, y: 0 }}
                  exit={{ opacity: 0, scale: 0.7, y: -6 }}
                  transition={{ duration: 0.2, ease: 'easeOut' }}
                  className='flex items-center justify-center'
                >
                  <Waveform />
                </motion.div>
              )}
              {state === 'transcribing' && (
                <motion.div
                  key='transcribing'
                  initial={{ opacity: 0, scale: 0.6, rotate: -10 }}
                  animate={{ opacity: 1, scale: 1, rotate: 0 }}
                  exit={{ opacity: 0, scale: 0.6, rotate: 10 }}
                  transition={{ duration: 0.24, ease: 'easeOut' }}
                  className='flex items-center justify-center'
                >
                  <Spinner />
                </motion.div>
              )}
              {state === 'not-ready' && (
                <motion.div
                  key='not-ready'
                  initial={{ opacity: 0, scale: 0.5, y: 8 }}
                  animate={{ opacity: 1, scale: 1, y: 0 }}
                  exit={{ opacity: 0, scale: 0.5, y: -8 }}
                  transition={{ type: 'spring', stiffness: 500, damping: 22 }}
                  className='flex h-6 w-6 items-center justify-center rounded-full bg-white text-xs font-bold text-black'
                >
                  !
                </motion.div>
              )}
            </AnimatePresence>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}
