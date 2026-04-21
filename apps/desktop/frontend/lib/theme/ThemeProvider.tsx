import { ReactNode, useEffect, useState } from 'react';
import { useThemeStore, initializeTheme } from './theme-store';

interface ThemeProviderProps {
  children: ReactNode;
}

export function ThemeProvider({ children }: ThemeProviderProps) {
  const { isLoading } = useThemeStore();
  const [showFallback, setShowFallback] = useState(false);
  
  useEffect(() => {
    const cleanup = initializeTheme();
    // If theme loading takes more than 1 second, show the app anyway
    const timeout = setTimeout(() => {
      setShowFallback(true);
    }, 1000);
    
    return () => {
      cleanup();
      clearTimeout(timeout);
    };
  }, []);
  
  // If still loading after timeout, show children anyway
  if (isLoading && !showFallback) {
    return (
      <div className="fixed inset-0 bg-app flex items-center justify-center">
        <div className="text-muted">Loading theme...</div>
      </div>
    );
  }
  
  return <div className="bg-app min-h-screen w-full h-full">{children}</div>;
}
