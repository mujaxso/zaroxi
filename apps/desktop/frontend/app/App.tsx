import { AppProvider } from './bootstrap/AppProvider';
import { AppShell } from './shell/AppShell';

function App() {
  return (
    <AppProvider>
      <AppShell />
    </AppProvider>
  );
}

export default App;
