import { StatusBar } from './StatusBar';
// import { CommandPalette } from './CommandPalette';
import { EditorContainer } from '@/features/editor/containers/EditorContainer';
import { ActivityRail } from '@/features/workbench/components/ActivityRail';
import { PanelHost } from '@/features/workbench/components/PanelHost';
import { TopBar } from '@/features/workbench/components/TopBar';
import { useWorkbenchStore } from '@/features/workbench/store/workbenchStore';
import { getActivityItem } from '@/features/workbench/config/activityRegistry';
import { Suspense, lazy, useEffect } from 'react';
import { setupWindowControls } from '@/lib/platform/windowControls';

// Lazy load full-width panel components
const SettingsPanel = lazy(() => import('@/features/settings/panel/SettingsPanel'));

export function AppShell() {
  const { 
    activeLeftPanel, 
    isLeftPanelVisible, 
    isRightPanelVisible, 
    activeRightPanel 
  } = useWorkbenchStore();
  
  // Get activity items for the active panels
  const leftActivity = activeLeftPanel ? getActivityItem(activeLeftPanel) : null;
  
  // Determine if we should show full-width panel (only settings)
  const isSettingsActive = leftActivity?.id === 'settings' && isLeftPanelVisible;
  
  // Show left panel when it's visible and not settings
  const showLeftPanel = isLeftPanelVisible && activeLeftPanel && !isSettingsActive;
  // Show right panel when it's visible
  const showRightPanel = isRightPanelVisible && activeRightPanel;
  // Show main content when not showing settings
  const showMainContent = !isSettingsActive;

  useEffect(() => {
    let cleanup: (() => void) | undefined;
    
    const init = async () => {
      cleanup = await setupWindowControls();
    };
    
    init();
    
    return () => {
      if (cleanup && typeof cleanup === 'function') {
        cleanup();
      }
    };
  }, []);

  return (
    <div className="flex flex-col h-screen bg-app text-primary font-sans overflow-hidden">
      
      {/* <CommandPalette /> */}
      
      {/* Compact Top Bar */}
      <TopBar />
      
      <div className="flex flex-1 overflow-hidden">
        {/* Activity Rail - Always visible */}
        <div className="h-full border-r border-divider">
          <ActivityRail />
        </div>
        
        {/* Left Panel (for all left-side panels except settings) */}
        {showLeftPanel && (
          <PanelHost side="left" />
        )}
        
        {/* Settings Panel (full width when active) */}
        {isSettingsActive && (
          <div className="flex-1 flex flex-col overflow-hidden bg-panel border-l border-divider">
            <Suspense fallback={
              <div className="p-4 bg-panel">
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
        )}
        
        {/* Main Content Area (Editor) */}
        {showMainContent && (
          <div className="flex-1 flex flex-col overflow-hidden">
            <div className="flex-1 overflow-hidden bg-editor">
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
