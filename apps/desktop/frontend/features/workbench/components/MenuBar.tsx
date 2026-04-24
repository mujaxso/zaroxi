import { useState, useEffect } from 'react';
import { cn } from '@/lib/utils';
import { useWorkbenchStore } from '../store/workbenchStore';
import { invoke } from '@tauri-apps/api/core';
import { isTauri } from '@/lib/platform/windowControls';

interface MenuItem {
  label: string;
  action: () => void;
}

export function MenuBar() {
  const [openMenu, setOpenMenu] = useState<string | null>(null);
  const [isTauriEnv, setIsTauriEnv] = useState(false);

  useEffect(() => {
    isTauri().then(setIsTauriEnv);
  }, []);

  const handleOpenWorkspace = async () => {
    if (!isTauriEnv) return;
    try {
      const path = await invoke<string | null>('open_file_dialog', { directory: true });
      if (path) {
        await invoke('open_workspace', { path });
      }
    } catch (e) {
      console.error('Failed to open workspace:', e);
    }
  };

  const menus: { label: string; items: MenuItem[] }[] = [
    {
      label: 'File',
      items: [
        { label: 'Open Workspace', action: handleOpenWorkspace },
        { label: 'New File', action: () => {} },
        { label: 'Save', action: () => {} },
      ],
    },
    {
      label: 'Edit',
      items: [
        { label: 'Undo', action: () => {} },
        { label: 'Redo', action: () => {} },
      ],
    },
    {
      label: 'View',
      items: [
        { label: 'Toggle Sidebar', action: () => {} },
      ],
    },
    {
      label: 'Tools',
      items: [
        { label: 'Settings', action: () => {} },
      ],
    },
  ];

  const toggleMenu = (label: string) => {
    if (openMenu === label) {
      setOpenMenu(null);
    } else {
      setOpenMenu(label);
    }
  };

  const closeAll = () => setOpenMenu(null);

  return (
    <div className="flex items-center h-10 bg-title-bar text-title-bar-foreground select-none" onMouseLeave={closeAll}>
      {menus.map((menu) => (
        <div key={menu.label} className="relative">
          <button
            className={cn(
              'px-3 py-1 text-xs font-medium hover:bg-hover-bg rounded-sm transition-colors',
              openMenu === menu.label && 'bg-hover-bg'
            )}
            onClick={() => toggleMenu(menu.label)}
          >
            {menu.label}
          </button>
          {openMenu === menu.label && (
            <div className="absolute top-full left-0 mt-0 bg-panel shadow-lg border border-border rounded-md py-1 z-50 min-w-[180px]">
              {menu.items.map((item) => (
                <button
                  key={item.label}
                  className="w-full px-3 py-1.5 text-left text-sm hover:bg-hover-bg transition-colors"
                  onClick={() => {
                    item.action();
                    closeAll();
                  }}
                >
                  {item.label}
                </button>
              ))}
            </div>
          )}
        </div>
      ))}
    </div>
  );
}
