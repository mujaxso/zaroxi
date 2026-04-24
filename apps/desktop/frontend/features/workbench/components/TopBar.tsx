import { Icon } from '@/components/ui/Icon';
import { cn } from '@/lib/utils';
import { useWorkbenchStore } from '../store/workbenchStore';
import { useEffect, useState } from 'react';
import { isTauri, getWindowInstance, windowControlActions } from '@/lib/platform/windowControls';
import { useLayoutMode } from '@/hooks/useLayoutMode';
import { useTabsStore } from '@/features/tabs/store';
import { useWorkspaceStore } from '@/features/workspace/stores/useWorkspaceStore';
import { useWorkspaceName } from '@/hooks/useWorkspaceName';
import { MenuBar } from './MenuBar';

interface TopBarProps {
  className?: string;
}

export function TopBar({ className }: TopBarProps) {
  const layoutMode = useLayoutMode();
  const { togglePanel } = useWorkbenchStore();
  const { tabs } = useTabsStore();
  const [isMaximized, setIsMaximized] = useState(false);
  const [isTauriEnv, setIsTauriEnv] = useState(false);

  const workspacePathFromHook = useWorkspaceName();
  const { rootPath } = useWorkspaceStore();
  const { tabs } = useTabsStore();

  const resolvedDisplayName = (() => {
    if (workspacePathFromHook) {
      const parts = workspacePathFromHook.split('/').filter(Boolean);
      return parts[parts.length - 1] || '';
    }
    if (rootPath) {
      const parts = rootPath.split('/').filter(Boolean);
      return parts[parts.length - 1] || '';
    }
    if (tabs.length > 0) {
      const firstId = tabs[0].id; // typically the absolute file path
      const parts = firstId.split('/').filter(Boolean);
      // For a path like /home/user/project/index.js -> parts = ["home","user","project","index.js"]
      // we want "project".
      if (parts.length >= 2) {
        return parts[parts.length - 2] || parts[parts.length - 1];
      }
      return parts[parts.length - 1] || '';
    }
    return '';
  })();

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
      {/* Left zone: brand + menu bar */}
      <div 
        className="flex items-center gap-1" 
        {...(isTauriEnv ? { 'data-tauri-drag-region': 'true' } : {})}
      >
        <div className="flex items-center gap-1.5" {...(isTauriEnv ? { 'data-tauri-drag-region': 'true' } : {})}>
          <Icon name="code" size={14} className="text-accent" />
          {layoutMode !== 'narrow' && (
            <span className="font-semibold text-sm text-primary leading-tight">Zaroxi Studio</span>
          )}
        </div>
        <MenuBar />
      </div>

      {/* Center zone: workspace context */}
      <div 
        className="flex-1 flex justify-center items-center" 
        {...(isTauriEnv ? { 'data-tauri-drag-region': 'true' } : {})}
      >
        {resolvedDisplayName ? (
          <span className="text-xs text-muted-foreground truncate max-w-md">
            {resolvedDisplayName}
          </span>
        ) : null}
      </div>

      {/* Right zone: window controls / global actions */}
      <div className="flex items-center gap-1">
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
              className="w-8 h-8 flex items-center justify-center rounded hover:bg-destructive/10 hover:text-destructive transition-colors"
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
              <Icon name="settings" size={13} />
            </button>
          </>
        )}
      </div>
    </div>
  );
}
