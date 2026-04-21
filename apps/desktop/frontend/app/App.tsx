import { AppProvider } from './bootstrap/AppProvider';
import { AppShell } from '@/layouts/shell/AppShell';

function App() {
  return (
    <AppProvider>
      <AppShell />
    </AppProvider>
  );
}

export default App;
