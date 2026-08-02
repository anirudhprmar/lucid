import { motion } from 'motion/react';

export default function Spinner() {
  return (
    <motion.div className='h-3 w-3 animate-spin rounded-full border-2 border-white/30 border-t-white' />
  );
}
