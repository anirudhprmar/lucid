export default function Shortcuts() {
  return (
    <div className='flex flex-1 flex-col gap-6 p-6'>
      <div className='rounded-2xl border border-white/10 bg-white/[0.02] p-5'>
        <div className='flex items-center justify-between'>
          <div>
            <h2 className='text-sm font-medium text-white/80'>Push-to-Talk</h2>
            <p className='mt-1 text-xs text-zinc-500'>
              Hold this shortcut to record audio for transcription.
            </p>
          </div>
          <div className='flex items-center gap-1.5'>
            <Kbd>Ctrl</Kbd>
            <Kbd>Alt</Kbd>
            <Kbd>Space</Kbd>
          </div>
        </div>
      </div>

      <div className='rounded-2xl border border-white/10 bg-white/[0.02] p-5'>
        <div className='flex items-center justify-between'>
          <div>
            <h2 className='text-sm font-medium text-white/80'>Toggle Notch</h2>
            <p className='mt-1 text-xs text-zinc-500'>
              Show or hide the floating indicator.
            </p>
          </div>
          <div className='flex items-center gap-1.5'>
            <Kbd>Ctrl</Kbd>
            <Kbd>Alt</Kbd>
            <Kbd>N</Kbd>
          </div>
        </div>
      </div>
    </div>
  );
}

function Kbd({ children }: { children: React.ReactNode }) {
  return (
    <kbd className='inline-flex h-7 min-w-7 items-center justify-center rounded-md border border-white/10 bg-white/5 px-1.5 py-1 text-xs font-medium text-white/70 shadow-[0_1px_0_0_rgba(255,255,255,0.05)_inset]'>
      {children}
    </kbd>
  );
}
