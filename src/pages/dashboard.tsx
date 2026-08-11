export default function Dashboard() {
  return (
    <div className='flex flex-1 flex-col gap-6 p-6'>
      <div>
        <h1 className='text-2xl font-semibold text-white'>Dashboard</h1>
        <p className='mt-1 text-sm text-zinc-400'>
          Overview of your Lucid usage and activity.
        </p>
      </div>

      <div className='grid grid-cols-1 gap-4 sm:grid-cols-3'>
        <StatCard label='Total Transcriptions' value='—' />
        <StatCard label='Hours Transcribed' value='—' />
        <StatCard label='Current Model' value='small.en' />
      </div>

      <div className='rounded-2xl border border-white/10 bg-white/[0.02] p-5'>
        <h2 className='text-sm font-medium text-white/80'>Usage Heatmap</h2>
        <p className='mt-1 text-xs text-zinc-500'>
          Your transcription activity over the past year.
        </p>
        <div className='mt-4 flex h-32 items-center justify-center text-sm text-zinc-600'>
          Start using Lucid to see your activity
        </div>
      </div>
    </div>
  );
}

function StatCard({ label, value }: { label: string; value: string }) {
  return (
    <div className='rounded-xl border border-white/10 bg-white/[0.02] p-4'>
      <p className='text-xs font-medium text-zinc-400'>{label}</p>
      <p className='mt-2 text-2xl font-semibold text-white'>{value}</p>
    </div>
  );
}
