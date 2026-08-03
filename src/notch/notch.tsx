import { useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event';
import Waveform from './waveform';
import Spinner from './spinner';
import { motion } from 'motion/react';

type NotchState = 'idle' | 'listening' | 'transcribing' | 'not-ready';

export default function Notch() {
  const [state, setState] = useState<NotchState>('idle');

  useEffect(() => {
    console.log('Notch mounted and listening for notch-state events');
    const unlisten = listen<NotchState>('notch-state', (e) => {
      console.log('notch event received', e.payload);
      setState(e.payload);
    });
    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  const dockVariants = {
    visible: {
      opacity: 1,
      scale: 1,
      transition: {
        when: 'afterChildren',
      },
    },
    hidden: {
      opacity: 0,
      scale: 0,
      transition: {
        when: 'beforeChildren',
      },
    },
  };

  return (
    <div className='flex h-screen items-center justify-center'>
      <motion.div
        className={`flex h-9 w-10 items-center justify-center rounded-lg bg-black/85`}
        initial={state === 'idle' ? 'hidden' : 'visible'}
        animate={state === 'idle' ? 'hidden' : 'visible'}
        variants={dockVariants}
      >
        {state === 'listening' && <Waveform />}
        {state === 'transcribing' && <Spinner />}
        {state === 'not-ready' && (
          <div className='text-sm text-white'>Model not ready yet</div>
        )}
      </motion.div>
    </div>
  );
}
