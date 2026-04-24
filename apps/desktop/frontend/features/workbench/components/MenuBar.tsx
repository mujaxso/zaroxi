import { useState } from 'react';
import { cn } from '@/lib/utils';
import { useWorkbenchStore } from '../store/workbenchStore';

interface MenuItem {
  label: string;
  action: () => void;
}

const menus: { label: string; items: MenuItem[] }[] = [
  {
    label: 'File',
    items: [
      { label: 'Open Folder', action: () => {} },
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

export function MenuBar() {
  const [openMenu, setOpenMenu] = useState<string | null>(null);

  const toggleMenu = (label: string) => {
    if (openMenu === label) {
      setOpenMenu(null);
    } else {
      setOpenMenu(label);
    }
  };

  const closeAll = () => setOpenMenu(null);

  return (
    <div className="flex items-center h-7 bg-title-bar text-title-bar-foreground select-none" onMouseLeave={closeAll}>
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
