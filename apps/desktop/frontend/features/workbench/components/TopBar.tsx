import { Icon } from '@/components/ui/Icon';
import { cn } from '@/lib/utils';
import { useWorkbenchStore } from '../store/workbenchStore';
import { useEffect, useState } from 'react';
import { isTauri, getWindowInstance, windowControlActions } from '@/lib/platform/windowControls';
import { useLayoutMode } from '@/hooks/useLayoutMode';
import { useWorkspaceStore } from '@/features/workspace/stores/useWorkspaceStore';

interface TopBarProps {
  className?: string;
}

export function TopBar({ className }: TopBarProps) {
  const layoutMode = useLayoutMode();
  const { togglePanel } = useWorkbenchStore();
  const { rootPath } = useWorkspaceStore();
  const [isMaximized, setIsMaximized] = useState(false);
  const [isTauriEnv, setIsTauriEnv] = useState(false);

  useEffect(() => {
    const checkTauri = async () => {
      const tauriCheck = await isTauri();
      setIsTauriEnv(tauriCheck);
      if (tauriCheck) {
        try {
          const currentWindow = await getWindowInstance();
          if (!currentWindow) return;
          const updateMaximized = async () => {
            setIsMaximized(await currentWindow.isMaximized());
          };
          await updateMaximized();
          
          const unlisten = await currentWindow.onResized(() => {
            updateMaximized();
          });
          
          return () => {
            if (unlisten) {
              unlisten();
            }
          };
        } catch (error) {
          console.error('Error setting up window listeners:', error);
        }
      }
    };
    checkTauri();
  }, []);

  const handleMinimize = async () => {
    if (isTauriEnv) {
      await windowControlActions.minimize();
    }
  };

  const handleMaximize = async () => {
    if (isTauriEnv) {
      await windowControlActions.maximize();
      // Update maximized state after a short delay
      setTimeout(async () => {
        try {
          const currentWindow = await getWindowInstance();
          if (currentWindow) {
            setIsMaximized(await currentWindow.isMaximized());
          }
        } catch (error) {
          console.error('Error updating maximized state:', error);
        }
      }, 100);
    }
  };

  const handleClose = async () => {
    if (isTauriEnv) {
      await windowControlActions.close();
    }
  };

  return (
    <div 
      className={cn(
        'h-10 flex items-center justify-between px-4',
        'bg-title-bar text-title-bar-foreground',
        'select-none',
        isTauriEnv ? 'cursor-default' : 'cursor-auto',
        className
      )}
      style={{ borderBottom: '0.5px solid var(--color-divider-subtle)' }}
      {...(isTauriEnv ? { 'data-tauri-drag-region': 'true' } : {})}
    >
      {/* Left zone: brand + menus */}
      <div 
        className="flex items-center gap-4" 
        {...(isTauriEnv ? { 'data-tauri-drag-region': 'true' } : {})}
      >
        <div 
          className="flex items-center gap-2" 
          {...(isTauriEnv ? { 'data-tauri-drag-region': 'true' } : {})}
        >
          <Icon name="code" size={18} className="text-accent" />
          {layoutMode !== 'narrow' && (
            <span className="font-semibold text-sm text-primary">Zaroxi Studio</span>
          )}
        </div>
        <nav 
          className="flex items-center gap-1" 
          {...(isTauriEnv ? { 'data-tauri-drag-region': 'true' } : {})}
        >
          <button
            onClick={() => togglePanel('explorer')}
            className="px-2 py-1.5 text-xs hover:bg-hover-bg transition-colors text-primary rounded-sm font-medium"
            data-no-drag="true"
          >
            File
          </button>
          <button
            onClick={() => togglePanel('search')}
            className="px-2 py-1.5 text-xs hover:bg-hover-bg transition-colors text-primary rounded-sm font-medium"
            data-no-drag="true"
          >
            Edit
          </button>
          <button
            onClick={() => togglePanel('settings')}
            className="px-2 py-1.5 text-xs hover:bg-hover-bg transition-colors text-primary rounded-sm font-medium"
            data-no-drag="true"
          >
            View
          </button>
          <button
            onClick={() => togglePanel('assistant')}
            className="px-2 py-1.5 text-xs hover:bg-hover-bg transition-colors text-primary rounded-sm font-medium"
            data-no-drag="true"
          >
            Tools
          </button>
        </nav>
      </div>

      {/* Center zone: workspace context */}
      <div 
        className="flex-1 flex justify-center items-center" 
        {...(isTauriEnv ? { 'data-tauri-drag-region': 'true' } : {})}
      >
        <span className="text-sm text-muted-foreground truncate max-w-md">
          {rootPath ? rootPath.split('/').pop() ?? rootPath : 'No project open'}
        </span>
      </div>

      {/* Right zone: window controls / global actions */}
      <div className="flex items-center gap-2">
        {isTauriEnv ? (
          <>
            <button
              onClick={handleMinimize}
              className="w-9 h-9 flex items-center justify-center rounded hover:bg-hover-bg transition-colors"
              aria-label="Minimize"
              data-no-drag="true"
            >
              <Icon name="window-minimize" size={13} />
            </button>
            <button
              onClick={handleMaximize}
              className="w-9 h-9 flex items-center justify-center rounded hover:bg-hover-bg transition-colors"
              aria-label={isMaximized ? 'Restore' : 'Maximize'}
              data-no-drag="true"
            >
              <Icon name={isMaximized ? 'window-restore' : 'window-maximize'} size={13} />
            </button>
            <button
              onClick={handleClose}
              className="w-9 h-9 flex items-center justify-center rounded hover:bg-destructive/10 hover:text-destructive transition-colors"
              aria-label="Close"
              data-no-drag="true"
            >
              <Icon name="window-close" size={13} />
            </button>
          </>
        ) : (
          <>
            <button
              onClick={() => togglePanel('settings')}
              className="w-9 h-9 flex items-center justify-center rounded hover:bg-hover-bg transition-colors"
              aria-label="Settings"
            >
              <Icon name="settings" size={15} />
            </button>
          </>
        )}
      </div>
    </div>
  );
}
