import { ThemeSettings } from '../components/ThemeSettings';

export function SettingsPage() {
  return (
    <div className="h-full bg-app text-primary">
      <div className="max-w-4xl mx-auto p-6">
        <div className="flex items-center gap-3 mb-8">
          <div className="p-2 rounded-lg bg-accent-soft-bg text-accent">
            <span className="text-2xl">⚙️</span>
          </div>
          <div>
            <h1 className="text-2xl font-bold text-primary">Settings</h1>
            <p className="text-muted">Customize your Zaroxi experience</p>
          </div>
        </div>
        
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          <div className="lg:col-span-2 space-y-8">
            <div className="bg-panel rounded-xl border border-border p-6">
              <div className="flex items-center gap-3 mb-6">
                <span className="text-2xl">🎨</span>
                <h2 className="text-xl font-semibold text-primary">Appearance</h2>
              </div>
              <ThemeSettings />
            </div>
            
            <div className="bg-panel rounded-xl border border-border p-6">
              <h2 className="text-xl font-semibold text-primary mb-6">Editor</h2>
              <p className="text-muted">Editor settings coming soon...</p>
            </div>
          </div>
          
          <div className="space-y-6">
            <div className="bg-panel rounded-xl border border-border p-6">
              <h3 className="font-medium text-primary mb-4">Quick Actions</h3>
              <div className="space-y-3">
                <button className="w-full text-left px-4 py-3 rounded-lg border border-border hover:bg-hover-bg transition-colors">
                  <div className="font-medium text-primary">Reset to Defaults</div>
                  <div className="text-sm text-muted">Restore all settings to default values</div>
                </button>
                <button className="w-full text-left px-4 py-3 rounded-lg border border-border hover:bg-hover-bg transition-colors">
                  <div className="font-medium text-primary">Export Settings</div>
                  <div className="text-sm text-muted">Save your configuration to a file</div>
                </button>
              </div>
            </div>
            
            <div className="bg-panel rounded-xl border border-border p-6">
              <h3 className="font-medium text-primary mb-4">About Theme</h3>
              <p className="text-sm text-muted mb-4">
                Zaroxi's theme system is designed for long coding sessions with optimal readability and reduced eye strain.
              </p>
              <div className="text-xs text-faint">
                <p>• Semantic color tokens</p>
                <p>• System theme detection</p>
                <p>• IDE-optimized contrast</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
