import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';
import AppSettings from './app-settings';

export default function Settings() {
  const [modelReady, setModelReady] = useState<boolean | null>(null);
  const [progress, setProgress] = useState(0);

  useEffect(() => {
    invoke<boolean>('check_model_exists').then(setModelReady);

    const unlistenProgress = listen<number>('model-download-progress', (e) =>
      setProgress(e.payload)
    );
    const unlistenDone = listen('model-download-complete', () =>
      setModelReady(true)
    );

    return () => {
      unlistenProgress.then((f) => f());
      unlistenDone.then((f) => f());
    };
  }, []);

  if (modelReady === null)
    return (
      <div className='flex min-h-screen items-center justify-center bg-black text-2xl font-semibold text-white'>
        Checking setup...
      </div>
    );
  if (!modelReady) {
    return (
      <div className='flex min-h-screen flex-col items-center justify-center bg-black text-2xl font-semibold text-white'>
        <img src='/icon.png' alt='lucid logo' className='h-40 w-40' />
        <h2>Setting up Lucid</h2>
        <p>Downloading speech model (~466MB), one-time only.</p>
        <progress value={progress} max={100} className='w-1/2' />
        <span>{progress}%</span>
      </div>
    );
  }

  return <AppSettings />;
}
