import { motion } from 'motion/react';

export default function Waveform() {
  const bars = [0, 100, 200, 300, 400];

  return (
    <div className='flex h-4 items-center gap-1'>
      {bars.map((delay, i) => (
        <motion.span
          key={i}
          className='h-3.5 w-1 rounded-full bg-white'
          animate={{
            scaleY: [0.4, 1, 0.4],
            opacity: [0.5, 1, 0.5],
          }}
          transition={{
            duration: 0.9,
            repeat: Infinity,
            delay: delay / 1000,
            ease: 'easeInOut',
          }}
        />
      ))}
    </div>
  );
}
