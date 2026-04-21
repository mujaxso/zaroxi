import { ReactNode } from 'react';
import { ErrorBoundary } from './ErrorBoundary';
import { TauriProvider } from './TauriProvider';
import { KeyboardShortcutsProvider } from '@/lib/keyboard/KeyboardShortcutsProvider';

interface AppProviderProps {
  children: ReactNode;
}

export function AppProvider({ children }: AppProviderProps) {
  return (
    <ErrorBoundary>
      <TauriProvider>
        <KeyboardShortcutsProvider>
          <div className="font-sans antialiased">
            {children}
          </div>
        </KeyboardShortcutsProvider>
      </TauriProvider>
    </ErrorBoundary>
  );
}
