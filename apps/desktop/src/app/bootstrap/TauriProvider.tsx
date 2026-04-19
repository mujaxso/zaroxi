import { ReactNode, useEffect, useState } from 'react';

interface TauriProviderProps {
  children: ReactNode;
}

export function TauriProvider({ children }: TauriProviderProps) {
  const [isInitialized, setIsInitialized] = useState(false);

  useEffect(() => {
    // Initialize Tauri-specific setup here
    console.log('Tauri provider mounted');
    
    // Check if we're running in Tauri
    const isTauri = window.__TAURI__ !== undefined;
    
    if (isTauri) {
      console.log('Running in Tauri environment');
    } else {
      console.warn('Running in browser environment - some features may be limited');
    }
    
    setIsInitialized(true);
    
    return () => {
      // Cleanup
    };
  }, []);

  if (!isInitialized) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-background">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary mx-auto"></div>
          <p className="mt-4 text-muted-foreground">Initializing Tauri...</p>
        </div>
      </div>
    );
  }

  return <>{children}</>;
}
