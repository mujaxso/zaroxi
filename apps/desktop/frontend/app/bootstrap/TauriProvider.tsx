import { ReactNode, useEffect, useState } from 'react';

interface TauriProviderProps {
  children: ReactNode;
}

export function TauriProvider({ children }: TauriProviderProps) {
  const [isInitialized, setIsInitialized] = useState(false);

  useEffect(() => {
    // Check if we're running in Tauri - use multiple detection methods
    const isTauri = 
      typeof window !== 'undefined' && 
      (window.__TAURI__ !== undefined || 
       (window as any).__TAURI_INTERNALS__ !== undefined ||
       navigator.userAgent.includes('Tauri'));
    
    if (isTauri) {
      // Initialize Tauri-specific features
      import('@tauri-apps/api').then(() => {
        // API loaded silently
      }).catch(() => {
        // Silently fail
      });
    }
    
    // Simulate initialization delay
    const timer = setTimeout(() => {
      setIsInitialized(true);
    }, 100);
    
    return () => {
      clearTimeout(timer);
    };
  }, []);

  if (!isInitialized) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-background">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary mx-auto"></div>
          <p className="mt-4 text-muted-foreground">Initializing...</p>
        </div>
      </div>
    );
  }

  return <>{children}</>;
}
