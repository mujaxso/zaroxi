import { ActivityRail } from './ActivityRail';
import { StatusBar } from './StatusBar';
import { CommandPalette } from './CommandPalette';
import { WorkspaceExplorerContainer } from '@/features/workspace/containers/WorkspaceExplorerContainer';
import { EditorContainer } from '@/features/editor/containers/EditorContainer';
import { AssistantPanelContainer } from '@/features/assistant/containers/AssistantPanelContainer';
import { useState } from 'react';

export function AppShell() {
  const [activePanel, setActivePanel] = useState<'explorer' | 'search' | 'git' | 'debug'>('explorer');
  const [isAssistantOpen, setIsAssistantOpen] = useState(false);

  return (
    <div className="flex flex-col h-screen bg-background text-foreground font-sans">
      <CommandPalette />
      
      <div className="flex flex-1 overflow-hidden">
        {/* Activity Rail */}
        <ActivityRail 
          activePanel={activePanel}
          onPanelChange={setActivePanel}
          onAssistantToggle={() => setIsAssistantOpen(!isAssistantOpen)}
        />
        
        {/* Side Panel */}
        <div className="w-64 border-r border-border bg-sidebar overflow-y-auto">
          {activePanel === 'explorer' && <WorkspaceExplorerContainer />}
          {/* Other panels would go here */}
        </div>
        
        {/* Main Content */}
        <div className="flex-1 flex flex-col overflow-hidden">
          <div className="flex-1 overflow-hidden">
            <EditorContainer />
          </div>
        </div>
        
        {/* Assistant Panel */}
        {isAssistantOpen && (
          <div className="w-96 border-l border-border bg-sidebar overflow-y-auto">
            <AssistantPanelContainer />
          </div>
        )}
      </div>
      
      {/* Status Bar */}
      <StatusBar />
    </div>
  );
}
