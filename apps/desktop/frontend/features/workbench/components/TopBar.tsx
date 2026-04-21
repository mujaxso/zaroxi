import { Icon } from '@/components/ui/Icon';
import { cn } from '@/lib/utils';
import { useWorkbenchStore } from '../store/workbenchStore';
import { useEffect, useState } from 'react';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { isTauri } from '@/lib/platform/windowControls';

interface TopBarProps {
  className?: string;
}

export function TopBar({ className }: TopBarProps) {
  const { togglePanel } = useWorkbenchStore();
  const [isMaximized, setIsMaximized] = useState(false);
  const [isTauriEnv, setIsTauriEnv] = useState(false);

  useEffect(() => {
    const checkTauri = async () => {
      setIsTauriEnv(await isTauri());
      if (await isTauri()) {
        const currentWindow = getCurrentWindow();
        const updateMaximized = async () => {
          setIsMaximized(await currentWindow.isMaximized());
        };
        updateMaximized();
        
        const unlisten = await currentWindow.onResized(() => {
          updateMaximized();
        });
        
        return () => {
          unlisten();
        };
      }
    };
    checkTauri();
  }, []);

  const handleMinimize = async () => {
    if (isTauriEnv) {
      const window = getCurrentWindow();
      await window.minimize();
    }
  };

  const handleMaximize = async () => {
    if (isTauriEnv) {
      const window = getCurrentWindow();
      if (isMaximized) {
        await window.unmaximize();
      } else {
        await window.maximize();
      }
      setIsMaximized(!isMaximized);
    }
  };

  const handleClose = async () => {
    if (isTauriEnv) {
      const window = getCurrentWindow();
      await window.close();
    }
  };

  return (
    <div 
      className={cn(
        'h-10 flex items-center justify-between px-4 border-b border-divider',
        'bg-title-bar text-title-bar-foreground',
        'select-none',
        isTauriEnv && 'cursor-default',
        className
      )}
      data-tauri-drag-region={isTauriEnv}
    >
      {/* Left section: Brand and menu */}
      <div className="flex items-center gap-6" data-tauri-drag-region={isTauriEnv}>
        <div className="flex items-center gap-2" data-tauri-drag-region={isTauriEnv}>
          <Icon name="code" size={16} className="text-accent" />
          <span className="font-semibold text-sm tracking-tight">Zaroxi Studio</span>
        </div>
        
        <div className="flex items-center gap-1">
          <button
            onClick={() => togglePanel('explorer')}
            className="px-3 py-1.5 text-xs rounded hover:bg-hover-bg transition-colors"
          >
            File
          </button>
          <button
            onClick={() => togglePanel('search')}
            className="px-3 py-1.5 text-xs rounded hover:bg-hover-bg transition-colors"
          >
            Edit
          </button>
          <button
            onClick={() => togglePanel('settings')}
            className="px-3 py-1.5 text-xs rounded hover:bg-hover-bg transition-colors"
          >
            View
          </button>
          <button
            onClick={() => togglePanel('assistant')}
            className="px-3 py-1.5 text-xs rounded hover:bg-hover-bg transition-colors"
          >
            Tools
          </button>
        </div>
      </div>

      {/* Center section: Workspace context */}
      <div className="flex-1 flex justify-center" data-tauri-drag-region={isTauriEnv}>
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
            >
              <Icon name="window-minimize" size={12} />
            </button>
            <button
              onClick={handleMaximize}
              className="w-8 h-8 flex items-center justify-center rounded hover:bg-hover-bg transition-colors"
              aria-label={isMaximized ? 'Restore' : 'Maximize'}
            >
              <Icon name={isMaximized ? 'window-restore' : 'window-maximize'} size={12} />
            </button>
            <button
              onClick={handleClose}
              className="w-8 h-8 flex items-center justify-center rounded hover:bg-destructive/20 hover:text-destructive transition-colors"
              aria-label="Close"
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
