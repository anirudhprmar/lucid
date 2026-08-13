import CheckForUpdates from '../components/updates';

export default function Settings() {
  return (
    <div className='flex flex-1 flex-col gap-6 p-6'>
      <CheckForUpdates />
    </div>
  );
}
