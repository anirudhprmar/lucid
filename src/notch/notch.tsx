import FloatingPill from './floating-pill';
import { motion } from 'motion/react';
import { useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event';

type NotchState = 'idle' | 'listening' | 'transcribing' | 'not-ready';

export default function Notch() {
  const [state, setState] = useState<NotchState>('idle');

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
    <div className='flex h-full w-full justify-center'>
      <motion.div
        className='flex items-center justify-center overflow-hidden rounded-b-2xl bg-black'
        initial={{ y: -50, width: 120, height: 40 }}
        animate={{
          y: 0,
          width: isActive ? 200 : 120,
          height: 40,
        }}
        transition={{
          y: { type: 'spring', stiffness: 300, damping: 24 },
          width: { type: 'spring', stiffness: 400, damping: 28 },
        }}
      >
        <FloatingPill />
      </motion.div>
    </div>
  );
}
