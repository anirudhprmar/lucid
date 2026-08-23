import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart';
import { useState, useEffect } from 'react';
import { Switch } from './ui/switch';

export default function Autostart() {
  const [checked, setChecked] = useState(false);

  useEffect(() => {
    isEnabled().then((enabled) => {
      setChecked(enabled);
    });
  }, []);

  const toggle = () => {
    if (checked) {
      disable();
    } else {
      enable();
    }
    setChecked(!checked);
  };

  return (
    <div className='bg-primary-foreground flex items-center justify-between rounded-xl border border-gray-50/10 p-4'>
      <div>
        <h3 className='text-foreground text-sm font-medium'>
          Start on Startup
        </h3>
        <p className='text-xs text-gray-500'>
          Enable this to start the application automatically when you log in.
        </p>
      </div>
      <Switch checked={checked} onCheckedChange={toggle} />
    </div>
  );
}
