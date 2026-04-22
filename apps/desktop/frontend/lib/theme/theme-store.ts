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
        // Add a timeout to ensure loading doesn't hang indefinitely
        const timeoutId = setTimeout(() => {
          set({ isLoading: false });
        }, 2000);
        
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
            
            clearTimeout(timeoutId);
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
            
            clearTimeout(timeoutId);
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
          clearTimeout(timeoutId);
          // On error, use system preference
          const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
          const isDark = systemPrefersDark;
          set({ 
            themeMode: 'system',
            isDark,
            isSystem: true,
            isLoading: false 
          });
          updateCssVariables(isDark);
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
let currentTheme: 'light' | 'dark' | null = null;

function updateCssVariables(isDark: boolean) {
  const root = document.documentElement;
  
  // Prevent unnecessary updates
  const newTheme = isDark ? 'dark' : 'light';
  if (currentTheme === newTheme) {
    return;
  }
  
  currentTheme = newTheme;
  console.log(`Setting theme to ${newTheme}`);
  
  if (isDark) {
    root.classList.add('dark');
    root.classList.remove('light');
  } else {
    root.classList.add('light');
    root.classList.remove('dark');
  }
  
  // Set data attribute for CSS selectors
  root.setAttribute('data-theme', newTheme);
}

// Apply theme immediately when this module loads (before any React code runs)
function applyThemeImmediately() {
  try {
    const saved = localStorage.getItem('zaroxi-theme-storage');
    let isDark = false;
    let themeMode: 'dark' | 'light' | 'system' = 'system';
    
    if (saved) {
      const parsed = JSON.parse(saved);
      if (parsed.state && parsed.state.themeMode) {
        themeMode = parsed.state.themeMode;
        const isSystem = themeMode === 'system';
        const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        isDark = themeMode === 'dark' || (isSystem && systemPrefersDark);
      } else {
        // If no saved theme, use system preference
        const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        isDark = systemPrefersDark;
        themeMode = 'system';
      }
    } else {
      // If no saved data, use system preference
      const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      isDark = systemPrefersDark;
      themeMode = 'system';
    }
    
    // Update CSS variables immediately
    const root = document.documentElement;
    if (isDark) {
      root.classList.add('dark');
      root.classList.remove('light');
    } else {
      root.classList.add('light');
      root.classList.remove('dark');
    }
    root.setAttribute('data-theme', isDark ? 'dark' : 'light');
    
    // Also update the store state
    useThemeStore.setState({
      themeMode,
      isDark,
      isSystem: themeMode === 'system',
      isLoading: false, // Don't load from backend if we already have the theme
    });
    
    // Store the applied theme to prevent unnecessary backend loading
    (window as any).__zaroxi_theme_applied = true;
  } catch (error) {
    console.error('Failed to apply theme immediately:', error);
    // Default to dark theme on error
    document.documentElement.classList.add('dark');
    document.documentElement.setAttribute('data-theme', 'dark');
    useThemeStore.setState({
      themeMode: 'dark',
      isDark: true,
      isSystem: false,
      isLoading: false,
    });
  }
}

// Call this immediately when the module is imported
applyThemeImmediately();

// Initialize theme on app start
export function initializeTheme() {
  const store = useThemeStore.getState();
  
  // Only load from backend if we haven't already applied the theme
  // This prevents double theme application and flash
  if (!(window as any).__zaroxi_theme_applied) {
    store.loadThemeSettings().catch(error => {
      console.error('Failed to load theme settings:', error);
      useThemeStore.setState({ isLoading: false });
    });
  }
  
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
