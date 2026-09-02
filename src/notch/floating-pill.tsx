import Waveform from './waveform';
import Spinner from './spinner';
import { useEffect, useState, useRef } from 'react';
import { listen } from '@tauri-apps/api/event';
import { LazyStore } from '@tauri-apps/plugin-store';

const d = new Date();
const store = new LazyStore('usage.json');

type storeDate = {
  count: number;
  duration: number;
};

type NotchState = 'idle' | 'listening' | 'transcribing' | 'not-ready';

export default function FloatingPill() {
  const [state, setState] = useState<NotchState>('idle');
  const startTimeRef = useRef<number | null>(null);

  useEffect(() => {
    console.log('Notch mounted and listening for notch-state events');
    const unlisten = listen<NotchState>('notch-state', (e) => {
      setState(e.payload);
    });
    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  useEffect(() => {
    async function updateStore() {
      const key = d.toISOString().split('T')[0];
      let data = await store.get<storeDate>(key);

      if (!data) {
        data = { count: 0, duration: 0 };
        // We save immediately if it's new so we have a base entry
        await store.set(key, data);
        await store.save();
      }

      if (state === 'listening') {
        // Start the timer
        startTimeRef.current = Date.now();
      } else if (startTimeRef.current !== null) {
        // We stopped listening, so calculate duration
        const duration = Date.now() - startTimeRef.current;
        data.duration += duration;
        data.count += 1;

        await store.set(key, data);
        await store.save();

        // Reset timer
        startTimeRef.current = null;
      }
    }

    void updateStore();
  }, [state]);

  return (
    <div className='flex h-full items-center justify-center'>
      {state === 'listening' && <Waveform />}
      {state === 'transcribing' && <Spinner />}
      {state === 'not-ready' && <div className='text-sm text-white'>!</div>}
    </div>
  );
}
