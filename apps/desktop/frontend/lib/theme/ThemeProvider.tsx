import { ReactNode, useEffect } from 'react';
import { useThemeStore, initializeTheme } from './theme-store';

interface ThemeProviderProps {
  children: ReactNode;
}

export function ThemeProvider({ children }: ThemeProviderProps) {
  const { isLoading } = useThemeStore();
  
  useEffect(() => {
    // Initialize the full theme logic (backend, listeners, etc.)
    const cleanup = initializeTheme();
    
    return () => {
      cleanup();
    };
  }, []);
  
  // Theme is already applied immediately when the module loads
  // So we don't need to block rendering
  return <div className="bg-app min-h-screen w-full h-full">{children}</div>;
}
