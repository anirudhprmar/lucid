import { getCurrentWindow } from '@tauri-apps/api/window';
import Notch from './notch/notch';
import Settings from './settings/settings';

function App() {
  const label = getCurrentWindow().label;

  return label === 'notch' ? <Notch /> : <Settings />;
}

export default App;
