import CheckForUpdates from './updates';

export default function AppSettings() {
  return (
    <main className='min-h-screen bg-[#0b0b0d] px-5 py-10 text-white sm:px-8'>
      <div className='mx-auto w-full max-w-2xl'>
        <div className='mb-8'>
          <p className='text-sm font-medium text-neutral-300'>Lucid</p>
          <h1 className='mt-1 text-3xl font-semibold tracking-tight'>
            Settings
          </h1>
          <p className='mt-2 text-sm text-zinc-400'>
            Manage your Lucid desktop app.
          </p>
        </div>
        <CheckForUpdates />
      </div>
    </main>
  );
}
