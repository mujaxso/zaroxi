import { useThemeStore } from '@/lib/theme/theme-store';
import { ZaroxiTheme } from '@/lib/theme/types';

export function ThemeSettings() {
  const { themeMode, setThemeMode, isLoading } = useThemeStore();
  
  const themes: { value: ZaroxiTheme; label: string; icon: string }[] = [
    { 
      value: 'system', 
      label: 'System', 
      icon: '🖥️'
    },
    { 
      value: 'light', 
      label: 'Light', 
      icon: '☀️'
    },
    { 
      value: 'dark', 
      label: 'Dark', 
      icon: '🌙'
    },
  ];
  
  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <div>
          <h3 className="text-lg font-medium text-primary">Theme</h3>
          <p className="text-sm text-muted">
            Choose how Zaroxi looks. System will follow your OS preference.
          </p>
        </div>
      </div>
      
      <div className="grid grid-cols-3 gap-3">
        {themes.map((theme) => (
          <button
            key={theme.value}
            onClick={() => setThemeMode(theme.value)}
            disabled={isLoading}
            className={`flex flex-col items-center justify-center p-4 rounded-lg border transition-all ${
              themeMode === theme.value
                ? 'border-accent bg-accent-soft-bg ring-2 ring-accent/20'
                : 'border-border hover:border-accent/50 hover:bg-hover-bg'
            }`}
          >
            <div className={`p-3 rounded-full mb-3 text-2xl ${
              themeMode === theme.value
                ? 'bg-accent/10 text-accent'
                : 'bg-panel text-muted'
            }`}>
              {theme.icon}
            </div>
            <span className={`text-sm font-medium ${
              themeMode === theme.value ? 'text-accent' : 'text-primary'
            }`}>
              {theme.label}
            </span>
          </button>
        ))}
      </div>
      
      <div className="pt-4 border-t border-divider">
        <div className="flex items-center justify-between">
          <div>
            <h4 className="text-sm font-medium text-primary">Current Theme</h4>
            <p className="text-sm text-muted">
              {themeMode === 'system' && 'Following system theme'}
              {themeMode === 'light' && 'Light theme active'}
              {themeMode === 'dark' && 'Dark theme active'}
            </p>
          </div>
          <div className="px-3 py-1 rounded-full bg-panel border border-border text-sm text-primary">
            {themeMode.charAt(0).toUpperCase() + themeMode.slice(1)}
          </div>
        </div>
      </div>
    </div>
  );
}
