import { StatusBar } from './StatusBar';
import { CommandPalette } from './CommandPalette';
import { EditorContainer } from '@/features/editor/containers/EditorContainer';
import { ActivityRail } from '@/features/workbench/components/ActivityRail';
import { PanelHost } from '@/features/workbench/components/PanelHost';
import { useWorkbenchStore } from '@/features/workbench/store/workbenchStore';
import { getActivityItem } from '@/features/workbench/config/activityRegistry';
import { Suspense, lazy } from 'react';

// Lazy load full-width panel components
const SettingsPanel = lazy(() => import('@/features/settings/panel/SettingsPanel'));
const ExtensionsPanel = lazy(() => import('@/features/extensions/panel/ExtensionsPanel'));

export function AppShell() {
  const { 
    activeLeftPanel, 
    isLeftPanelVisible, 
    isRightPanelVisible, 
    activeRightPanel 
  } = useWorkbenchStore();
  
  // Get activity items for the active panels
  const leftActivity = activeLeftPanel ? getActivityItem(activeLeftPanel) : null;
  
  // Determine if we should show full-width panels (settings or extensions)
  const isFullWidthPanel = leftActivity && 
    (leftActivity.id === 'settings' || leftActivity.id === 'extensions');
  
  // Show left panel when it's visible and not a full-width panel
  const showLeftPanel = isLeftPanelVisible && activeLeftPanel && !isFullWidthPanel;
  // Show right panel when it's visible
  const showRightPanel = isRightPanelVisible && activeRightPanel;
  // Show main content when not showing full-width panel
  const showMainContent = !isFullWidthPanel;

  return (
    <div className="flex flex-col h-screen bg-background text-foreground font-sans">
      <CommandPalette />
      
      <div className="flex flex-1 overflow-hidden">
        {/* Activity Rail - Always visible */}
        <ActivityRail />
        
        {/* Left Panel */}
        {showLeftPanel && (
          <PanelHost side="left" />
        )}
        
        {/* Full-width panels (Settings, Extensions) */}
        {isFullWidthPanel && isLeftPanelVisible && (
          <div className="flex-1 flex flex-col overflow-hidden">
            <Suspense fallback={
              <div className="p-4">
                <div className="space-y-2">
                  <div className="h-4 bg-muted rounded animate-pulse w-3/4"></div>
                  <div className="h-4 bg-muted rounded animate-pulse w-1/2"></div>
                  <div className="h-4 bg-muted rounded animate-pulse w-5/6"></div>
                </div>
              </div>
            }>
              {leftActivity?.id === 'settings' && <SettingsPanel />}
              {leftActivity?.id === 'extensions' && <ExtensionsPanel />}
            </Suspense>
          </div>
        )}
        
        {/* Main Content Area */}
        {showMainContent && (
          <div className="flex-1 flex flex-col overflow-hidden">
            <div className="flex-1 overflow-hidden">
              <EditorContainer />
            </div>
          </div>
        )}
        
        {/* Right Panel (Assistant) */}
        {showRightPanel && (
          <PanelHost side="right" />
        )}
      </div>
      
      {/* Status Bar */}
      <StatusBar />
    </div>
  );
}
