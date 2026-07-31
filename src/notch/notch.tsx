import { useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event';
import Waveform from './waveform';
import Spinner from './spinner';

type NotchState = 'idle' | 'listening' | 'transcribing';

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

  return (
    <div
      className={`flex h-full items-center justify-center rounded-full bg-black/85 transition-opacity duration-150 ease-out ${state === 'idle' ? 'opacity-0' : 'opacity-100'} `}
    >
      hi
      {state === 'listening' && <Waveform />}
      {state === 'transcribing' && <Spinner />}
    </div>
  );
}
