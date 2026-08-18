import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';

export default function ModelManagement() {
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

  return (
    <div className='flex flex-1 flex-col gap-6 p-6'>
      {modelReady === null ? (
        <div className='flex items-center justify-center py-12 text-sm text-zinc-500'>
          Checking model status...
        </div>
      ) : !modelReady ? (
        <div className='rounded-2xl border border-white/10 bg-white/[0.02] p-6'>
          <h2 className='text-sm font-medium text-white/80'>
            Downloading Model
          </h2>
          <p className='mt-1 text-xs text-zinc-500'>
            One-time download of ggml-small.en.bin (~466 MB)
          </p>
          <div className='mt-4 h-2 overflow-hidden rounded-full bg-white/10'>
            <div
              className='h-full rounded-full bg-white/60 transition-[width] duration-200'
              style={{ width: `${progress}%` }}
            />
          </div>
          <p className='mt-2 text-xs text-zinc-400'>{progress}%</p>
        </div>
      ) : (
        <div className='space-y-3'>
          <ModelCard
            name='small.en'
            size='466 MB'
            description='Good accuracy, moderate speed'
            active
          />
          <ModelCard
            name='base.en'
            size='74 MB'
            description='Decent accuracy, faster'
          />
          <ModelCard
            name='tiny.en'
            size='38 MB'
            description='Lower accuracy, fastest'
          />
        </div>
      )}
    </div>
  );
}

function ModelCard({
  name,
  size,
  description,
  active = false,
}: {
  name: string;
  size: string;
  description: string;
  active?: boolean;
}) {
  return (
    <div
      className={`flex items-center justify-between rounded-xl border p-4 ${
        active
          ? 'border-white/20 bg-white/[0.04]'
          : 'border-white/10 bg-white/[0.02]'
      }`}
    >
      <div className='flex items-center gap-3'>
        <div className='flex size-10 items-center justify-center rounded-lg bg-white/5'>
          <svg
            viewBox='0 0 24 24'
            fill='none'
            stroke='currentColor'
            strokeWidth='1.8'
            className='size-5 text-white/60'
          >
            <circle cx='12' cy='12' r='3' />
            <path strokeLinecap='round' d='M12 2v4m0 12v4M2 12h4m12 0h4' />
          </svg>
        </div>
        <div>
          <p className='text-sm font-medium text-white'>{name}</p>
          <p className='text-xs text-zinc-500'>{description}</p>
        </div>
      </div>
      <div className='flex items-center gap-3'>
        <span className='text-xs text-zinc-500'>{size}</span>
        {active ? (
          <span className='rounded-full bg-emerald-500/15 px-2.5 py-0.5 text-xs font-medium text-emerald-400'>
            Active
          </span>
        ) : (
          <button
            type='button'
            className='rounded-lg border border-white/10 px-3 py-1.5 text-xs font-medium text-white/70 transition hover:bg-white/5'
          >
            Switch
          </button>
        )}
      </div>
    </div>
  );
}
