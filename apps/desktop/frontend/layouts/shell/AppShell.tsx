import { StatusBar } from './StatusBar';
import { CommandPalette } from './CommandPalette';
import { EditorContainer } from '@/features/editor/containers/EditorContainer';
import { ActivityRail } from '@/features/workbench/components/ActivityRail';
import { PanelHost } from '@/features/workbench/components/PanelHost';
import { useWorkbenchStore } from '@/features/workbench/store/workbenchStore';
import { cn } from '@/lib/utils';

export function AppShell() {
  const { activeLeftPanel, isLeftPanelVisible, isRightPanelVisible, activeRightPanel } = useWorkbenchStore();
  
  const isSettingsActive = activeLeftPanel === 'settings';
  const isAssistantActive = activeRightPanel === 'assistant' && isRightPanelVisible;

  return (
    <div className="flex flex-col h-screen bg-background text-foreground font-sans">
      <CommandPalette />
      
      <div className="flex flex-1 overflow-hidden">
        {/* Activity Rail */}
        <ActivityRail />
        
        {/* Left Panel (Explorer, Search, Git, Debug, Settings) */}
        {isLeftPanelVisible && activeLeftPanel && (
          <PanelHost side="left" />
        )}
        
        {/* Main Content Area */}
        <div className={cn(
          "flex-1 flex flex-col overflow-hidden",
          isSettingsActive && "hidden"
        )}>
          <div className="flex-1 overflow-hidden">
            <EditorContainer />
          </div>
        </div>
        
        {/* Settings Panel (full width when active) */}
        {isSettingsActive && (
          <div className="flex-1 flex flex-col overflow-hidden">
            <PanelHost side="left" />
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
