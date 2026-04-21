import { StatusBar } from './StatusBar';
import { CommandPalette } from './CommandPalette';
import { EditorContainer } from '@/features/editor/containers/EditorContainer';
import { ActivityRail } from '@/features/workbench/components/ActivityRail';
import { PanelHost } from '@/features/workbench/components/PanelHost';
import { useWorkbenchStore } from '@/features/workbench/store/workbenchStore';
import { cn } from '@/lib/utils';
import { SettingsPanel } from '@/features/settings/panel/SettingsPanel';
import { Suspense } from 'react';

export function AppShell() {
  const { activeLeftPanel, isLeftPanelVisible, isRightPanelVisible, activeRightPanel } = useWorkbenchStore();
  
  const isSettingsActive = activeLeftPanel === 'settings';
  const isExtensionsActive = activeLeftPanel === 'extensions';
  const isAssistantActive = activeRightPanel === 'assistant' && isRightPanelVisible;

  // When settings or extensions is active, hide the left panel and show full-width panel
  const showLeftPanel = isLeftPanelVisible && activeLeftPanel && !isSettingsActive && !isExtensionsActive;
  const showMainContent = !isSettingsActive && !isExtensionsActive;

  return (
    <div className="flex flex-col h-screen bg-background text-foreground font-sans">
      <CommandPalette />
      
      <div className="flex flex-1 overflow-hidden">
        {/* Activity Rail */}
        <ActivityRail />
        
        {/* Left Panel (Explorer, Search, Git, Debug) - Hidden when settings/extensions is active */}
        {showLeftPanel && (
          <PanelHost side="left" />
        )}
        
        {/* Main Content Area - Hidden when settings/extensions is active */}
        {showMainContent && (
          <div className="flex-1 flex flex-col overflow-hidden">
            <div className="flex-1 overflow-hidden">
              <EditorContainer />
            </div>
          </div>
        )}
        
        {/* Settings Panel (full width when active) */}
        {isSettingsActive && (
          <div className="flex-1 flex flex-col overflow-hidden">
            <div className="h-full overflow-auto">
              <Suspense fallback={
                <div className="p-4">
                  <div className="space-y-2">
                    <div className="h-4 bg-muted rounded animate-pulse w-3/4"></div>
                    <div className="h-4 bg-muted rounded animate-pulse w-1/2"></div>
                    <div className="h-4 bg-muted rounded animate-pulse w-5/6"></div>
                  </div>
                </div>
              }>
                <SettingsPanel />
              </Suspense>
            </div>
          </div>
        )}
        
        {/* Extensions Panel (full width when active) */}
        {isExtensionsActive && (
          <div className="flex-1 flex flex-col overflow-hidden">
            <div className="h-full overflow-auto p-6">
              <div className="max-w-4xl mx-auto">
                <div className="flex items-center gap-3 mb-8">
                  <div className="p-2 rounded-lg bg-accent-soft-bg text-accent">
                    <span className="text-2xl">🧩</span>
                  </div>
                  <div>
                    <h1 className="text-2xl font-bold text-primary">Extensions</h1>
                    <p className="text-muted">Enhance your Zaroxi experience with extensions</p>
                  </div>
                </div>
                
                <div className="bg-panel rounded-xl border border-border p-6">
                  <h2 className="text-xl font-semibold text-primary mb-4">Coming Soon</h2>
                  <p className="text-muted mb-4">
                    The extensions marketplace is under development. You'll be able to browse, install, and manage extensions for:
                  </p>
                  <ul className="space-y-2 text-muted">
                    <li>• Language support (Python, JavaScript, Go, etc.)</li>
                    <li>• Themes and UI customizations</li>
                    <li>• Productivity tools and integrations</li>
                    <li>• AI assistants and code analysis</li>
                  </ul>
                  <div className="mt-6 p-4 bg-muted rounded-lg">
                    <p className="text-sm text-primary">
                      <strong>Note:</strong> Extension support is planned for a future release. Stay tuned!
                    </p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}
        
        {/* Right Panel (Assistant) */}
        {isAssistantActive && (
          <PanelHost side="right" />
        )}
      </div>
      
      {/* Status Bar */}
      <StatusBar />
    </div>
  );
}
