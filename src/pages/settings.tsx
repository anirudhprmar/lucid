import CheckForUpdates from '../components/updates';

export default function Settings() {
  return (
    <div className='flex flex-1 flex-col gap-6 p-6'>
      <div>
        <h1 className='text-2xl font-semibold text-white'>Settings</h1>
        <p className='mt-1 text-sm text-zinc-400'>
          Manage your Lucid desktop app.
        </p>
      </div>
      <CheckForUpdates />
    </div>
  );
}
