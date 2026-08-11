import { Routes, Route, Navigate } from 'react-router';
import { getCurrentWindow } from '@tauri-apps/api/window';
import Layout from './components/layout';
import Notch from './notch/notch';
import Dashboard from './pages/dashboard';
import ModelManagement from './pages/model-management';
import Settings from './pages/settings';
import Shortcuts from './pages/shortcuts';

function App() {
  const label = getCurrentWindow().label;

  if (label === 'notch') {
    return <Notch />;
  }

  return (
    <Routes>
      <Route element={<Layout />}>
        <Route path='/' element={<Dashboard />} />
        <Route path='/models' element={<ModelManagement />} />
        <Route path='/shortcuts' element={<Shortcuts />} />
        <Route path='/settings' element={<Settings />} />
      </Route>
      <Route path='*' element={<Navigate to='/' replace />} />
    </Routes>
  );
}

export default App;
