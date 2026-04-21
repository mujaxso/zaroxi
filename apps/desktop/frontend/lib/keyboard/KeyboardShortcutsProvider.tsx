import { ReactNode, useEffect } from 'react';
import { useWorkbenchStore } from '@/features/workbench/store/workbenchStore';

interface KeyboardShortcutsProviderProps {
  children: ReactNode;
}

function KeyboardShortcutsHandler({ children }: KeyboardShortcutsProviderProps) {
  const { toggleLeftPanelVisibility, activateLeftPanel, activateRightPanel } = useWorkbenchStore();

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Global keyboard shortcuts
      switch (e.key) {
        case 'b':
          if ((e.ctrlKey || e.metaKey) && e.shiftKey) {
            e.preventDefault();
            toggleLeftPanelVisibility();
          }
          break;
        case ',':
          if (e.ctrlKey || e.metaKey) {
            e.preventDefault();
            activateLeftPanel('settings');
          }
          break;
        case '1':
          if (e.ctrlKey || e.metaKey) {
            e.preventDefault();
            activateLeftPanel('explorer');
          }
          break;
        case '2':
          if (e.ctrlKey || e.metaKey) {
            e.preventDefault();
            activateLeftPanel('search');
          }
          break;
        case '3':
          if (e.ctrlKey || e.metaKey) {
            e.preventDefault();
            activateLeftPanel('git');
          }
          break;
        case '4':
          if (e.ctrlKey || e.metaKey) {
            e.preventDefault();
            activateLeftPanel('debug');
          }
          break;
        case '5':
          if (e.ctrlKey || e.metaKey) {
            e.preventDefault();
            activateRightPanel('assistant');
          }
          break;
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [toggleLeftPanelVisibility, activateLeftPanel, activateRightPanel]);

  return <>{children}</>;
}

export function KeyboardShortcutsProvider({ children }: KeyboardShortcutsProviderProps) {
  return <KeyboardShortcutsHandler children={children} />;
}
