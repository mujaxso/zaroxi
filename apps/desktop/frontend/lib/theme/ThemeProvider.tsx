import { ReactNode, useEffect, useState } from 'react';
import { useThemeStore, initializeTheme } from './theme-store';

interface ThemeProviderProps {
  children: ReactNode;
}

export function ThemeProvider({ children }: ThemeProviderProps) {
  const { isLoading } = useThemeStore();
  const [initialThemeApplied, setInitialThemeApplied] = useState(false);
  
  useEffect(() => {
    // Apply theme immediately from localStorage before any rendering
    try {
      const saved = localStorage.getItem('zaroxi-theme-storage');
      let isDark = false;
      
      if (saved) {
        const parsed = JSON.parse(saved);
        if (parsed.state && parsed.state.themeMode) {
          const themeMode = parsed.state.themeMode;
          const isSystem = themeMode === 'system';
          const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
          isDark = themeMode === 'dark' || (isSystem && systemPrefersDark);
        } else {
          // If no saved theme, use system preference
          isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        }
      } else {
        // If no saved data, use system preference
        isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
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
    } catch (error) {
      console.error('Failed to apply initial theme:', error);
      // Default to dark theme on error
      document.documentElement.classList.add('dark');
      document.documentElement.setAttribute('data-theme', 'dark');
    }
    setInitialThemeApplied(true);
    
    // Then initialize the full theme logic
    const cleanup = initializeTheme();
    
    return () => {
      cleanup();
    };
  }, []);
  
  // Don't render children until initial theme is applied
  if (!initialThemeApplied) {
    return null;
  }
  
  return <div className="bg-app min-h-screen w-full h-full">{children}</div>;
}
