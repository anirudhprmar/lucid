export default function Waveform() {
  const bars = [0, 100, 200, 300, 400]; // stagger delays in ms
  return (
    <div className='flex h-4 items-center gap-1'>
      {bars.map((delay, i) => (
        <span
          key={i}
          className='w-0.5 animate-pulse rounded-full bg-white'
          style={{ animationDelay: `${delay}ms`, height: '60%' }}
        />
      ))}
    </div>
  );
}
