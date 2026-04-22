import { ReactNode, useEffect, useState } from 'react';
import { useThemeStore, initializeTheme } from './theme-store';

interface ThemeProviderProps {
  children: ReactNode;
}

export function ThemeProvider({ children }: ThemeProviderProps) {
  const { isLoading } = useThemeStore();
  const [themeReady, setThemeReady] = useState(false);
  
  useEffect(() => {
    // Theme is already applied immediately when the module loads
    // Wait for the next animation frame to ensure CSS is applied
    const frameId = requestAnimationFrame(() => {
      setThemeReady(true);
    });
    
    // Initialize the full theme logic (backend, listeners, etc.)
    const cleanup = initializeTheme();
    
    return () => {
      cancelAnimationFrame(frameId);
      cleanup();
    };
  }, []);
  
  // Don't render children until theme is ready
  // This prevents flash of unstyled content
  if (!themeReady) {
    return null;
  }
  
  return <div className="bg-app min-h-screen w-full h-full">{children}</div>;
}
