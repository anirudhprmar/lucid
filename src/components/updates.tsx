import { check, type Update } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import { useState } from 'react';

type UpdateStatus =
  | 'idle'
  | 'checking'
  | 'up-to-date'
  | 'available'
  | 'downloading'
  | 'ready'
  | 'error';

const APP_VERSION = '0.1.0';

function getErrorMessage(error: unknown) {
  return error instanceof Error
    ? error.message
    : 'Unable to check for updates.';
}

export default function CheckForUpdates() {
  const [status, setStatus] = useState<UpdateStatus>('idle');
  const [update, setUpdate] = useState<Update | null>(null);
  const [downloadedBytes, setDownloadedBytes] = useState(0);
  const [totalBytes, setTotalBytes] = useState<number | null>(null);
  const [error, setError] = useState<string | null>(null);

  const checkForUpdates = async () => {
    setStatus('checking');
    setError(null);
    setUpdate(null);

    try {
      const availableUpdate = await check();
      setUpdate(availableUpdate);
      setStatus(availableUpdate ? 'available' : 'up-to-date');
    } catch (error) {
      setError(getErrorMessage(error));
      setStatus('error');
    }
  };

  const installUpdate = async () => {
    if (!update) return;

    setStatus('downloading');
    setError(null);
    setDownloadedBytes(0);
    setTotalBytes(null);

    try {
      await update.downloadAndInstall((event) => {
        if (event.event === 'Started') {
          setTotalBytes(event.data.contentLength ?? null);
        }

        if (event.event === 'Progress') {
          setDownloadedBytes((bytes) => bytes + event.data.chunkLength);
        }
      });
      setStatus('ready');
    } catch (error) {
      setError(getErrorMessage(error));
      setStatus('error');
    }
  };

  const progress = totalBytes
    ? Math.min(100, Math.round((downloadedBytes / totalBytes) * 100))
    : null;

  const isBusy = status === 'checking' || status === 'downloading';
  const updateLabel = update ? `Version ${update.version}` : 'Update available';

  return (
    <section
      aria-labelledby='updates-heading'
      className='overflow-hidden rounded-2xl border border-white/10 bg-white/4 shadow-2xl shadow-black/20'
    >
      <div className='flex items-start gap-4 p-5 sm:p-6'>
        <div className='flex size-11 shrink-0 items-center justify-center rounded-xl bg-neutral-500/15 text-neutral-200 ring-1 ring-neutral-300/20 ring-inset'>
          <svg
            viewBox='0 0 24 24'
            fill='none'
            stroke='currentColor'
            strokeWidth='1.8'
            className='size-5'
            aria-hidden='true'
          >
            <path
              strokeLinecap='round'
              strokeLinejoin='round'
              d='M12 3v12m0 0 4-4m-4 4-4-4M5 17v2.5A1.5 1.5 0 0 0 6.5 21h11a1.5 1.5 0 0 0 1.5-1.5V17'
            />
          </svg>
        </div>

        <div className='min-w-0 flex-1'>
          <div className='flex flex-wrap items-center gap-x-3 gap-y-1'>
            <h2
              id='updates-heading'
              className='text-base font-semibold text-white'
            >
              Software updates
            </h2>
            <span className='rounded-full bg-white/8 px-2 py-0.5 text-xs font-medium text-zinc-400'>
              v{APP_VERSION}
            </span>
          </div>
          <p className='mt-1 text-sm leading-6 text-zinc-400'>
            Keep Lucid current with the latest improvements and fixes.
          </p>
        </div>
      </div>

      <div className='border-t border-white/8 px-5 py-4 sm:px-6'>
        {(status === 'idle' || status === 'up-to-date') && (
          <div className='flex flex-wrap items-center justify-between gap-3'>
            <p className='text-sm text-zinc-400'>
              {status === 'up-to-date'
                ? 'You’re running the latest version.'
                : 'Check for a newer version of Lucid.'}
            </p>
            <button
              type='button'
              onClick={checkForUpdates}
              disabled={isBusy}
              className='inline-flex items-center gap-2 rounded-lg bg-white px-3.5 py-2 text-sm font-semibold text-zinc-950 transition hover:bg-zinc-200 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-neutral-400 disabled:cursor-not-allowed disabled:opacity-60'
            >
              Check for updates
            </button>
          </div>
        )}

        {status === 'checking' && (
          <div
            className='flex items-center gap-3 text-sm text-zinc-300'
            role='status'
          >
            <span
              className='size-4 animate-spin rounded-full border-2 border-zinc-600 border-t-white'
              aria-hidden='true'
            />
            Checking for updates…
          </div>
        )}

        {status === 'available' && update && (
          <div className='flex flex-wrap items-center justify-between gap-3'>
            <div>
              <p className='text-sm font-medium text-white'>
                {updateLabel} is ready to install.
              </p>
              {update.body && (
                <p className='mt-1 text-sm text-zinc-400'>{update.body}</p>
              )}
            </div>
            <button
              type='button'
              onClick={installUpdate}
              className='inline-flex items-center gap-2 rounded-lg bg-neutral-500 px-3.5 py-2 text-sm font-semibold text-white transition hover:bg-neutral-400 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-neutral-400'
            >
              Install update
            </button>
          </div>
        )}

        {status === 'downloading' && (
          <div role='status'>
            <div className='flex items-center justify-between gap-4 text-sm'>
              <span className='font-medium text-white'>
                Downloading update…
              </span>
              <span className='text-zinc-400'>
                {progress === null ? 'Preparing' : `${progress}%`}
              </span>
            </div>
            <div className='mt-3 h-1.5 overflow-hidden rounded-full bg-white/10'>
              <div
                className='h-full rounded-full bg-neutral-400 transition-[width] duration-200'
                style={{ width: `${progress ?? 8}%` }}
              />
            </div>
          </div>
        )}

        {status === 'ready' && (
          <div className='flex flex-wrap items-center justify-between gap-3'>
            <p className='text-sm text-emerald-300'>
              Update installed. Restart Lucid to finish.
            </p>
            <button
              type='button'
              onClick={() => void relaunch()}
              className='rounded-lg bg-emerald-400 px-3.5 py-2 text-sm font-semibold text-emerald-950 transition hover:bg-emerald-300 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-emerald-300'
            >
              Restart now
            </button>
          </div>
        )}

        {status === 'error' && (
          <div className='flex flex-wrap items-center justify-between gap-3'>
            <p className='text-sm text-rose-300'>{error}</p>
            <button
              type='button'
              onClick={checkForUpdates}
              className='rounded-lg border border-white/15 px-3.5 py-2 text-sm font-semibold text-white transition hover:bg-white/10 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-neutral-400'
            >
              Try again
            </button>
          </div>
        )}
      </div>
    </section>
  );
}
