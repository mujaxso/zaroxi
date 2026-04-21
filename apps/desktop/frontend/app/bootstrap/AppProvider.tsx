import { ReactNode, useEffect, useState } from 'react';
import { ErrorBoundary } from './ErrorBoundary';
import { TauriProvider } from './TauriProvider';
import { KeyboardShortcutsProvider } from '@/lib/keyboard/KeyboardShortcutsProvider';
import { FontLoader } from './FontLoader';
import { ThemeProvider } from '@/lib/theme/ThemeProvider';
import { Toolbar } from '@/components/layout/Toolbar';
import { SettingsPage } from '@/features/settings/pages/SettingsPage';
import '@/styles/tokens.css';

// Check if we're running in Tauri
const isTauri = typeof window !== 'undefined' && '__TAURI__' in window;

interface AppProviderProps {
  children: ReactNode;
}

export function AppProvider({ children }: AppProviderProps) {
  const [showSettings, setShowSettings] = useState(false);

  useEffect(() => {
    // Listen for open-settings event from menu (Tauri only)
    let unlistenSettings: Promise<() => void> | undefined;
    let unlistenWorkspace: Promise<() => void> | undefined;
    
    if (isTauri) {
      import('@tauri-apps/api/event').then(({ listen }) => {
        unlistenSettings = listen('open-settings', () => {
          setShowSettings(true);
        });
        
        unlistenWorkspace = listen('menu:open-workspace', () => {
          // Trigger the open workspace dialog
          import('@tauri-apps/api/core').then(({ invoke }) => {
            invoke('open_file_dialog').catch(console.error);
          });
        });
      });
    }

    // Also listen for custom event from toolbar (works in both environments)
    const handleOpenSettings = () => setShowSettings(true);
    window.addEventListener('open-settings', handleOpenSettings as EventListener);

    return () => {
      if (unlistenSettings) {
        unlistenSettings.then(f => f()).catch(console.error);
      }
      if (unlistenWorkspace) {
        unlistenWorkspace.then(f => f()).catch(console.error);
      }
      window.removeEventListener('open-settings', handleOpenSettings as EventListener);
    };
  }, []);

  return (
    <ErrorBoundary>
      <TauriProvider>
        <KeyboardShortcutsProvider>
          <ThemeProvider>
            <FontLoader />
            <div className="font-sans antialiased bg-app text-primary h-screen flex flex-col">
              <Toolbar />
              <div className="flex-1 overflow-hidden">
                {showSettings ? (
                  <SettingsPage />
                ) : (
                  children
                )}
              </div>
              {/* Simple back button when in settings */}
              {showSettings && (
                <div className="absolute top-16 left-4">
                  <button
                    onClick={() => setShowSettings(false)}
                    className="px-4 py-2 bg-panel border border-border rounded-lg hover:bg-hover-bg text-primary"
                  >
                    ← Back
                  </button>
                </div>
              )}
            </div>
          </ThemeProvider>
        </KeyboardShortcutsProvider>
      </TauriProvider>
    </ErrorBoundary>
  );
}
