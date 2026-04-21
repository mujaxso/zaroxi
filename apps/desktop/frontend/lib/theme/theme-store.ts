import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { ZaroxiTheme } from './types';

// Check if we're running in Tauri
const isTauri = typeof window !== 'undefined' && '__TAURI__' in window;

// Helper to convert Rust enum to TypeScript string
function toRustTheme(mode: ZaroxiTheme): 'Dark' | 'Light' | 'System' {
  switch (mode) {
    case 'dark': return 'Dark';
    case 'light': return 'Light';
    case 'system': return 'System';
  }
}

function fromRustTheme(mode: 'Dark' | 'Light' | 'System'): ZaroxiTheme {
  switch (mode) {
    case 'Dark': return 'dark';
    case 'Light': return 'light';
    case 'System': return 'system';
  }
}

interface ThemeStore {
  themeMode: ZaroxiTheme;
  isDark: boolean;
  isSystem: boolean;
  isLoading: boolean;
  
  // Actions
  setThemeMode: (mode: ZaroxiTheme) => Promise<void>;
  loadThemeSettings: () => Promise<void>;
  applyTheme: (data: { mode: ZaroxiTheme; isDark: boolean }) => void;
}

export const useThemeStore = create<ThemeStore>()(
  persist(
    (set, get) => ({
      themeMode: 'system',
      isDark: true,
      isSystem: true,
      isLoading: false,
      
      setThemeMode: async (mode) => {
        set({ isLoading: true });
        try {
          if (isTauri) {
            // In Tauri environment, use the backend
            const { invoke } = await import('@tauri-apps/api/core');
            const rustTheme = toRustTheme(mode);
            await invoke('set_theme', { theme: rustTheme });
          } else {
            // In browser, just update locally
            // Simulate async operation
            await new Promise(resolve => setTimeout(resolve, 50));
          }
          
          // Determine if dark based on mode
          const isSystem = mode === 'system';
          const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
          const isDark = mode === 'dark' || (isSystem && systemPrefersDark);
          
          set({ 
            themeMode: mode, 
            isDark,
            isSystem,
            isLoading: false 
          });
          
          // Update CSS variables
          updateCssVariables(isDark);
        } catch (error) {
          console.error('Failed to set theme:', error);
          set({ isLoading: false });
        }
      },
      
      loadThemeSettings: async () => {
        set({ isLoading: true });
        try {
          if (isTauri) {
            // In Tauri environment, load from backend
            const { invoke } = await import('@tauri-apps/api/core');
            // The Rust command returns a ZaroxiTheme enum which will be serialized as a string
            const currentTheme: string = await invoke('get_current_theme');
            const rustTheme = currentTheme as 'Dark' | 'Light' | 'System';
            const tsTheme = fromRustTheme(rustTheme);
            
            const isSystem = tsTheme === 'system';
            const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
            const isDark = tsTheme === 'dark' || (isSystem && systemPrefersDark);
            
            set({ 
              themeMode: tsTheme,
              isDark,
              isSystem,
              isLoading: false 
            });
            
            updateCssVariables(isDark);
          } else {
            // In browser, load from localStorage (handled by persist middleware)
            // Also check system preference
            const savedState = get();
            const isSystem = savedState.themeMode === 'system';
            const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
            const isDark = savedState.themeMode === 'dark' || (isSystem && systemPrefersDark);
            
            set({ 
              themeMode: savedState.themeMode,
              isDark,
              isSystem,
              isLoading: false 
            });
            
            updateCssVariables(isDark);
          }
        } catch (error) {
          console.error('Failed to load theme settings:', error);
          set({ isLoading: false });
        }
      },
      
      applyTheme: (data) => {
        const isSystem = data.mode === 'system';
        set({
          themeMode: data.mode,
          isDark: data.isDark,
          isSystem,
        });
        updateCssVariables(data.isDark);
      },
    }),
    {
      name: 'zaroxi-theme-storage',
      partialize: (state) => ({
        themeMode: state.themeMode,
        isDark: state.isDark,
      }),
    }
  )
);

// Listen to theme changes from backend (Tauri only)
async function setupThemeListener() {
  if (!isTauri) return () => {};
  
  try {
    const { listen } = await import('@tauri-apps/api/event');
    return listen<{ mode: string; isDark: boolean }>('theme:changed', (event) => {
      // The mode comes as a string from Rust serialization
      const rustMode = event.payload.mode as 'Dark' | 'Light' | 'System';
      const tsMode = fromRustTheme(rustMode);
      useThemeStore.getState().applyTheme({ 
        mode: tsMode, 
        isDark: event.payload.isDark 
      });
    });
  } catch (error) {
    console.error('Failed to setup theme listener:', error);
    return () => {};
  }
}

// Update CSS custom properties based on theme
function updateCssVariables(isDark: boolean) {
  const root = document.documentElement;
  
  if (isDark) {
    root.classList.add('dark');
    root.classList.remove('light');
  } else {
    root.classList.add('light');
    root.classList.remove('dark');
  }
  
  // Set data attribute for CSS selectors
  root.setAttribute('data-theme', isDark ? 'dark' : 'light');
}

// Initialize theme on app start
export function initializeTheme() {
  const store = useThemeStore.getState();
  
  // Load saved theme
  store.loadThemeSettings();
  
  // Listen to system theme changes
  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
  const handleSystemThemeChange = (e: MediaQueryListEvent) => {
    const { themeMode } = useThemeStore.getState();
    if (themeMode === 'system') {
      updateCssVariables(e.matches);
      useThemeStore.getState().applyTheme({ 
        mode: 'system', 
        isDark: e.matches 
      });
    }
  };
  
  mediaQuery.addEventListener('change', handleSystemThemeChange);
  
  // Setup backend listener (Tauri only)
  let cleanupListener: (() => void) | undefined;
  setupThemeListener().then(unlisten => {
    if (unlisten) {
      cleanupListener = () => {
        try {
          unlisten.then(f => f());
        } catch (error) {
          console.error('Error cleaning up listener:', error);
        }
      };
    }
  });
  
  return () => {
    mediaQuery.removeEventListener('change', handleSystemThemeChange);
    if (cleanupListener) {
      cleanupListener();
    }
  };
}
