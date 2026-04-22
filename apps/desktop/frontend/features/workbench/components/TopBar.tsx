import { Icon } from '@/components/ui/Icon';
import { cn } from '@/lib/utils';
import { useWorkbenchStore } from '../store/workbenchStore';
import { useEffect, useState } from 'react';
import { isTauri, getWindowInstance, windowControlActions } from '@/lib/platform/windowControls';

interface TopBarProps {
  className?: string;
}

export function TopBar({ className }: TopBarProps) {
  const { togglePanel } = useWorkbenchStore();
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
        'h-9 flex items-center justify-between px-4 border-b border-divider',
        'bg-title-bar text-title-bar-foreground',
        'select-none',
        isTauriEnv ? 'cursor-default' : 'cursor-auto',
        className
      )}
      {...(isTauriEnv ? { 'data-tauri-drag-region': 'true' } : {})}
    >
      {/* Left section: Brand and menu */}
      <div 
        className="flex items-center gap-6" 
        {...(isTauriEnv ? { 'data-tauri-drag-region': 'true' } : {})}
      >
        <div 
          className="flex items-center gap-2" 
          {...(isTauriEnv ? { 'data-tauri-drag-region': 'true' } : {})}
        >
          <Icon name="code" size={16} className="text-accent" />
          <span className="font-semibold text-sm text-primary">Zaroxi Studio</span>
        </div>
        
        <div className="flex items-center gap-0">
          <button
            onClick={() => togglePanel('explorer')}
            className="px-3 py-1.5 text-xs hover:bg-hover-bg transition-colors text-primary hover:text-accent font-semibold"
            data-no-drag="true"
          >
            File
          </button>
          <button
            onClick={() => togglePanel('search')}
            className="px-3 py-1.5 text-xs hover:bg-hover-bg transition-colors text-primary hover:text-accent font-semibold"
            data-no-drag="true"
          >
            Edit
          </button>
          <button
            onClick={() => togglePanel('settings')}
            className="px-3 py-1.5 text-xs hover:bg-hover-bg transition-colors text-primary hover:text-accent font-semibold"
            data-no-drag="true"
          >
            View
          </button>
          <button
            onClick={() => togglePanel('assistant')}
            className="px-3 py-1.5 text-xs hover:bg-hover-bg transition-colors text-primary hover:text-accent font-semibold"
            data-no-drag="true"
          >
            Tools
          </button>
        </div>
      </div>

      {/* Center section: Workspace context */}
      <div 
        className="flex-1 flex justify-center" 
        {...(isTauriEnv ? { 'data-tauri-drag-region': 'true' } : {})}
      >
        <div className="text-xs text-muted truncate max-w-md">
          No workspace open
        </div>
      </div>

      {/* Right section: Window controls or quick actions */}
      <div className="flex items-center gap-2">
        {isTauriEnv ? (
          <>
            <button
              onClick={handleMinimize}
              className="w-8 h-8 flex items-center justify-center rounded hover:bg-hover-bg transition-colors"
              aria-label="Minimize"
              data-no-drag="true"
            >
              <Icon name="window-minimize" size={12} />
            </button>
            <button
              onClick={handleMaximize}
              className="w-8 h-8 flex items-center justify-center rounded hover:bg-hover-bg transition-colors"
              aria-label={isMaximized ? 'Restore' : 'Maximize'}
              data-no-drag="true"
            >
              <Icon name={isMaximized ? 'window-restore' : 'window-maximize'} size={12} />
            </button>
            <button
              onClick={handleClose}
              className="w-8 h-8 flex items-center justify-center rounded hover:bg-destructive/20 hover:text-destructive transition-colors"
              aria-label="Close"
              data-no-drag="true"
            >
              <Icon name="window-close" size={12} />
            </button>
          </>
        ) : (
          <>
            <button
              onClick={() => togglePanel('settings')}
              className="w-8 h-8 flex items-center justify-center rounded hover:bg-hover-bg transition-colors"
              aria-label="Settings"
            >
              <Icon name="settings" size={14} />
            </button>
            <button
              onClick={() => {}}
              className="w-8 h-8 flex items-center justify-center rounded hover:bg-hover-bg transition-colors"
              aria-label="Theme"
            >
              <Icon name="palette" size={14} />
            </button>
          </>
        )}
      </div>
    </div>
  );
}
