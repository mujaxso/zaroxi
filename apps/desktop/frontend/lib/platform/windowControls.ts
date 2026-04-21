let getCurrentWindow: any = null;

// Dynamically import to avoid errors if module not available
async function loadWindowApi() {
  if (typeof window === 'undefined') return;
  try {
    const module = await import('@tauri-apps/api/window');
    getCurrentWindow = module.getCurrentWindow || module.default?.getCurrentWindow;
  } catch (error) {
    console.warn('Failed to load Tauri window API:', error);
  }
}

export async function getWindowInstance() {
  if (await isTauri()) {
    await loadWindowApi();
    if (getCurrentWindow) {
      return getCurrentWindow();
    }
  }
  return null;
}

export async function isTauri(): Promise<boolean> {
  return typeof window !== 'undefined' && '__TAURI__' in window;
}

export async function setupWindowControls() {
  if (await isTauri()) {
    try {
      await loadWindowApi();
      if (!getCurrentWindow) {
        console.warn('getCurrentWindow not available');
        return () => {};
      }
      // Enable custom window controls
      const appWindow = getCurrentWindow();
      
      // Make the top bar draggable
      const handleDrag = (e: MouseEvent) => {
        // Check if the click is on a button or element with data-no-drag attribute
        const target = e.target as HTMLElement;
        if (target.closest('[data-no-drag="true"]')) {
          return;
        }
        // Start window dragging
        appWindow.startDragging();
      };

      // Setup drag regions
      const setupDrag = () => {
        const dragRegions = document.querySelectorAll('[data-tauri-drag-region]');
        dragRegions.forEach(region => {
          // Remove existing listeners to avoid duplicates
          region.removeEventListener('mousedown', handleDrag);
          region.addEventListener('mousedown', handleDrag);
        });
      };

      // Initial setup
      setupDrag();
      
      // Also set up a mutation observer to handle dynamic changes
      const observer = new MutationObserver(() => {
        setupDrag();
      });
      
      observer.observe(document.body, { childList: true, subtree: true });
      
      // Return cleanup function
      return () => {
        observer.disconnect();
        const dragRegions = document.querySelectorAll('[data-tauri-drag-region]');
        dragRegions.forEach(region => {
          region.removeEventListener('mousedown', handleDrag);
        });
      };
    } catch (error) {
      console.error('Error setting up window controls:', error);
    }
  }
  return () => {};
}

export const windowControlActions = {
  minimize: async () => {
    if (await isTauri()) {
      try {
        await loadWindowApi();
        if (!getCurrentWindow) return;
        const window = getCurrentWindow();
        await window.minimize();
      } catch (error) {
        console.error('Error minimizing window:', error);
      }
    }
  },
  maximize: async () => {
    if (await isTauri()) {
      try {
        await loadWindowApi();
        if (!getCurrentWindow) return;
        const window = getCurrentWindow();
        const isMaximized = await window.isMaximized();
        if (isMaximized) {
          await window.unmaximize();
        } else {
          await window.maximize();
        }
      } catch (error) {
        console.error('Error toggling maximize:', error);
      }
    }
  },
  close: async () => {
    if (await isTauri()) {
      try {
        await loadWindowApi();
        if (!getCurrentWindow) return;
        const window = getCurrentWindow();
        await window.close();
      } catch (error) {
        console.error('Error closing window:', error);
      }
    }
  },
};
