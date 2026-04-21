import { getCurrent } from '@tauri-apps/api/window';

export async function isTauri(): Promise<boolean> {
  return typeof window !== 'undefined' && '__TAURI__' in window;
}

export async function setupWindowControls() {
  if (await isTauri()) {
    // Enable custom window controls
    const appWindow = getCurrent();
    
    // Make the top bar draggable
    const dragRegions = document.querySelectorAll('[data-tauri-drag-region="true"]');
    dragRegions.forEach(region => {
      region.addEventListener('mousedown', (e) => {
        if (e.target instanceof HTMLElement && e.target.closest('button')) {
          return;
        }
        appWindow.startDragging();
      });
    });
  }
}

export const windowControlActions = {
  minimize: async () => {
    if (await isTauri()) {
      const window = getCurrent();
      await window.minimize();
    }
  },
  maximize: async () => {
    if (await isTauri()) {
      const window = getCurrent();
      const isMaximized = await window.isMaximized();
      if (isMaximized) {
        await window.unmaximize();
      } else {
        await window.maximize();
      }
    }
  },
  close: async () => {
    if (await isTauri()) {
      const window = getCurrent();
      await window.close();
    }
  },
};
