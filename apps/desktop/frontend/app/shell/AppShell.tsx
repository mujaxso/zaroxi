import React from 'react';
import { ActivityRail } from '../../../layouts/shell/ActivityRail';
import { StatusBar } from '../../../layouts/shell/StatusBar';
import { CommandPalette } from '../../../layouts/shell/CommandPalette';
import { WorkspaceExplorerContainer } from '../../features/workspace/containers/WorkspaceExplorerContainer';
import { EditorContainer } from '../../features/editor/containers/EditorContainer';
import { AssistantPanelContainer } from '../../features/assistant/containers/AssistantPanelContainer';

export function AppShell() {
  const [activePanel, setActivePanel] = React.useState('explorer');
  const [assistantOpen, setAssistantOpen] = React.useState(false);

  return (
    <div className="flex flex-col h-screen bg-background text-foreground">
      <div className="flex flex-1 overflow-hidden">
        <ActivityRail 
          activePanel={activePanel as any}
          onPanelChange={(panel) => setActivePanel(panel)}
          onAssistantToggle={() => setAssistantOpen(!assistantOpen)}
        />
        <div className="flex-1 flex overflow-hidden">
          {activePanel === 'explorer' && (
            <div className="w-64 border-r border-border overflow-auto">
              <WorkspaceExplorerContainer />
            </div>
          )}
          <div className="flex-1 overflow-hidden">
            <EditorContainer />
          </div>
          {assistantOpen && (
            <div className="w-96 border-l border-border overflow-auto">
              <AssistantPanelContainer />
            </div>
          )}
        </div>
      </div>
      <StatusBar />
      <CommandPalette />
    </div>
  );
}
