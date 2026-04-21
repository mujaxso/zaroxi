import { useState } from 'react';
import { useThemeStore } from '@/lib/theme/theme-store';
import { ZaroxiTheme } from '@/lib/theme/types';

export function Toolbar() {
  const { themeMode, setThemeMode } = useThemeStore();
  const [isMenuOpen, setIsMenuOpen] = useState(false);
  
  const handleThemeChange = (theme: ZaroxiTheme) => {
    setThemeMode(theme);
    setIsMenuOpen(false);
  };
  
  const themeIcon = {
    system: '🖥️',
    light: '☀️',
    dark: '🌙',
  }[themeMode];
  
  return (
    <div className="h-12 bg-title-bar border-b border-border flex items-center justify-between px-4">
      {/* Left side */}
      <div className="flex items-center gap-4">
        <button 
          className="p-2 rounded hover:bg-hover-bg text-primary"
          onClick={() => setIsMenuOpen(!isMenuOpen)}
        >
          <span className="text-lg">☰</span>
        </button>
        
        <div className="text-sm font-medium text-primary">
          Zaroxi IDE
        </div>
      </div>
      
      {/* Right side */}
      <div className="flex items-center gap-2">
        {/* Theme switcher dropdown */}
        <div className="relative">
          <button 
            className="flex items-center gap-2 px-3 py-1.5 rounded-lg border border-border hover:bg-hover-bg text-primary"
            onClick={() => setIsMenuOpen(!isMenuOpen)}
          >
            <span>{themeIcon}</span>
            <span className="text-sm capitalize">{themeMode}</span>
          </button>
          
          {isMenuOpen && (
            <div className="absolute right-0 top-full mt-1 w-48 bg-panel border border-border rounded-lg shadow-lg z-50">
              <div className="py-1">
                <button
                  onClick={() => handleThemeChange('system')}
                  className={`w-full text-left px-4 py-2 flex items-center gap-3 hover:bg-hover-bg ${
                    themeMode === 'system' ? 'bg-selected-bg text-accent' : 'text-primary'
                  }`}
                >
                  <span>🖥️</span>
                  <span>System</span>
                </button>
                <button
                  onClick={() => handleThemeChange('light')}
                  className={`w-full text-left px-4 py-2 flex items-center gap-3 hover:bg-hover-bg ${
                    themeMode === 'light' ? 'bg-selected-bg text-accent' : 'text-primary'
                  }`}
                >
                  <span>☀️</span>
                  <span>Light</span>
                </button>
                <button
                  onClick={() => handleThemeChange('dark')}
                  className={`w-full text-left px-4 py-2 flex items-center gap-3 hover:bg-hover-bg ${
                    themeMode === 'dark' ? 'bg-selected-bg text-accent' : 'text-primary'
                  }`}
                >
                  <span>🌙</span>
                  <span>Dark</span>
                </button>
              </div>
            </div>
          )}
        </div>
        
        <button 
          className="p-2 rounded hover:bg-hover-bg text-primary"
          onClick={() => {
            // Emit event to open settings
            window.dispatchEvent(new CustomEvent('open-settings'));
          }}
        >
          <span className="text-lg">⚙️</span>
        </button>
      </div>
    </div>
  );
}
