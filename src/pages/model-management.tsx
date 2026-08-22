import { invoke } from '@tauri-apps/api/core';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';

interface ModelCard {
  name: string;
  size: string;
  description: string;
  status: 'not-downloaded' | 'downloading' | 'downloaded' | 'deleting';
  isActive: boolean;
  isSwitching?: boolean;
  progress?: number;
  onDownload: (name: string) => void;
  onSwitch?: (name: string) => void;
  onDelete: (name: string) => void;
}

export default function ModelManagement() {
  const [downloaded, setDownloaded] = useState<string[]>([]);
  const [active, setActive] = useState<string | null>(null);
  const [progress, setProgress] = useState<Record<string, number>>({});
  const [deleting, setDeleting] = useState<Set<string>>(new Set());
  const [switching, setSwitching] = useState<Set<string>>(new Set());

  const models = [
    {
      name: 'tiny',
      filename: 'ggml-tiny.en.bin',
      size: '75 MB',
      description: 'Fastest and most efficient, lower accuracy',
    },
    {
      name: 'base.en',
      filename: 'ggml-base.en.bin',
      size: '142 MB',
      description: 'Good balance between accuracy and speed',
    },
    {
      name: 'small-q5_1',
      filename: 'ggml-small-q5_1.bin',
      size: '190 MB',
      description: 'Works great on mid-tier devices',
    },
    {
      name: 'small.en',
      filename: 'ggml-small.en.bin',
      size: '466 MB',
      description: 'Most accurate, higher memory usage',
    },
  ];

  useEffect(() => {
    invoke<string[]>('list_downloaded_models').then((res) =>
      setDownloaded(res || [])
    );
    invoke<string | null>('get_current_model').then((res) =>
      setActive(res || null)
    );

    const unlistens: Promise<UnlistenFn>[] = [];

    models.forEach((model) => {
      const p1 = listen<number>(
        `model-download-progress-${model.name}`,
        (e) => {
          setProgress((prev) => ({ ...prev, [model.name]: e.payload }));
        }
      );

      const p2 = listen(`model-download-complete-${model.name}`, () => {
        setProgress((prev) => {
          const next = { ...prev };
          delete next[model.name];
          return next;
        });
        invoke<string[]>('list_downloaded_models').then((res) =>
          setDownloaded(res || [])
        );
      });

      unlistens.push(p1, p2);
    });

    return () => {
      unlistens.forEach((p) => p.then((f) => f()));
    };
  }, []);

  const handleDownload = async (name: string) => {
    try {
      setProgress((prev) => ({ ...prev, [name]: 0 }));
      await invoke('download_named_model', { name });
    } catch (error) {
      console.error('Failed to download model:', error);
      setProgress((prev) => {
        const next = { ...prev };
        delete next[name];
        return next;
      });
    }
  };

  const handleDelete = async (name: string) => {
    setDeleting((prev) => new Set(prev).add(name));
    try {
      await invoke('delete_model', { name });
      const updated = await invoke<string[]>('list_downloaded_models');
      setDownloaded(updated || []);
    } catch (err) {
      console.error('Failed to delete model:', err);
    } finally {
      setDeleting((prev) => {
        const next = new Set(prev);
        next.delete(name);
        return next;
      });
    }
  };

  const handleSwitch = async (name: string) => {
    setSwitching((prev) => new Set(prev).add(name));
    try {
      await invoke('switch_active_model', { name });
      const current = await invoke<string | null>('get_current_model');
      setActive(current || null);
    } catch (error) {
      console.error('Failed to switch model:', error);
    } finally {
      setSwitching((prev) => {
        const next = new Set(prev);
        next.delete(name);
        return next;
      });
    }
  };

  const getStatus = (modelName: string, filename: string) => {
    if (deleting.has(modelName)) return 'deleting';
    if (downloaded.includes(filename)) return 'downloaded';
    if (progress[modelName] !== undefined) return 'downloading';
    return 'not-downloaded';
  };

  const checkIsActive = (filename: string) => {
    if (!active) return false;
    return active.endsWith(filename) || active === filename;
  };

  return (
    <div className='flex flex-1 flex-col gap-6 p-6'>
      <div className='space-y-3'>
        {models.map((model) => (
          <ModelCard
            key={model.name}
            name={model.name}
            size={model.size}
            description={model.description}
            isActive={checkIsActive(model.filename)}
            isSwitching={switching.has(model.name)}
            status={getStatus(model.name, model.filename)}
            progress={progress[model.name]}
            onDownload={handleDownload}
            onDelete={handleDelete}
            onSwitch={handleSwitch}
          />
        ))}
      </div>
    </div>
  );
}

function ModelCard({
  name,
  size,
  description,
  isActive = false,
  isSwitching = false,
  status,
  progress,
  onDownload,
  onDelete,
  onSwitch,
}: ModelCard) {
  return (
    <div
      className={`flex items-center justify-between rounded-xl border p-4 ${
        isActive ? 'border-white/20 bg-white/4' : 'border-white/10 bg-white/2'
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
          <div className='flex items-center gap-2'>
            <p className='text-sm font-medium text-white'>{name}</p>
            {isActive && (
              <span className='rounded-full bg-emerald-500/15 px-2.5 py-0.5 text-xs font-medium text-emerald-400'>
                Active
              </span>
            )}
          </div>
          <p className='text-xs text-zinc-500'>{description}</p>
        </div>
      </div>

      <div className='flex items-center gap-3'>
        <span className='text-xs text-zinc-500'>{size}</span>

        {status === 'downloading' && (
          <div className='flex min-w-[120px] items-center gap-3'>
            <div className='h-1.5 flex-1 overflow-hidden rounded-full bg-white/10'>
              <div
                className='h-full rounded-full bg-white/60 transition-[width] duration-200'
                style={{ width: `${progress ?? 0}%` }}
              />
            </div>
            <span className='font-mono text-xs text-zinc-400'>
              {progress ?? 0}%
            </span>
          </div>
        )}

        {status === 'deleting' && (
          <span className='animate-pulse text-xs text-rose-400/80'>
            Deleting...
          </span>
        )}

        {status === 'downloaded' && (
          <div className='flex items-center gap-2'>
            {!isActive && onSwitch && (
              <button
                type='button'
                onClick={() => onSwitch(name)}
                disabled={isSwitching}
                className='rounded-lg border border-white/10 px-3 py-1.5 text-xs font-medium text-white/90 transition hover:bg-white/10 disabled:opacity-50'
              >
                {isSwitching ? 'Switching...' : 'Switch'}
              </button>
            )}
            <button
              type='button'
              onClick={() => onDelete(name)}
              title={isActive ? 'Will be removed on next restart' : undefined}
              className={`rounded-lg px-3 py-1.5 text-xs font-medium transition ${
                isActive
                  ? 'border border-rose-500/20 bg-rose-500/10 text-rose-300 hover:bg-rose-500/20'
                  : 'border border-white/10 text-rose-400/80 hover:border-rose-500/30 hover:bg-rose-500/10 hover:text-rose-300'
              }`}
            >
              Delete
            </button>
          </div>
        )}

        {status === 'not-downloaded' && (
          <button
            type='button'
            onClick={() => onDownload(name)}
            className='inline-flex items-center gap-1.5 rounded-lg border border-white/10 px-3 py-1.5 text-xs font-medium text-white/90 transition hover:bg-white/10'
          >
            <svg
              viewBox='0 0 24 24'
              fill='none'
              stroke='currentColor'
              strokeWidth='2'
              className='size-3.5'
            >
              <path
                strokeLinecap='round'
                strokeLinejoin='round'
                d='M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5M16.5 12L12 16.5m0 0L7.5 12m4.5 4.5V3'
              />
            </svg>
            Download
          </button>
        )}
      </div>
    </div>
  );
}
