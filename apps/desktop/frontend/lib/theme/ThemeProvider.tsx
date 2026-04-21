import { ReactNode, useEffect } from 'react';
import { useThemeStore, initializeTheme } from './theme-store';

interface ThemeProviderProps {
  children: ReactNode;
}

export function ThemeProvider({ children }: ThemeProviderProps) {
  const { isLoading } = useThemeStore();
  
  useEffect(() => {
    const cleanup = initializeTheme();
    // Mark loading as complete after a short delay to ensure theme is applied
    const timer = setTimeout(() => {
      // We can't directly modify the store here, but the store should update itself
    }, 100);
    return () => {
      cleanup();
      clearTimeout(timer);
    };
  }, []);
  
  // Prevent flash of unstyled content
  if (isLoading) {
    return (
      <div className="fixed inset-0 bg-app flex items-center justify-center">
        <div className="text-muted">Loading theme...</div>
      </div>
    );
  }
  
  return <div className="bg-app min-h-screen w-full h-full">{children}</div>;
}
