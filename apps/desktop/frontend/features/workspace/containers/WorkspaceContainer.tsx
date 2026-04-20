import { useEffect } from 'react';
import { ExplorerContainer } from '../containers/ExplorerContainer';
import { EditorContainer } from '@/features/editor/containers/EditorContainer';
import { useWorkspaceStore } from '../stores/useWorkspaceStore';
import { WorkspaceService } from '../services/workspaceService';

export function WorkspaceContainer() {
  const { 
    currentWorkspace, 
    explorerUI, 
    setCurrentWorkspace, 
    setWorkspaceTree, 
    setLoading, 
    setError,
    toggleExpanded
  } = useWorkspaceStore();

  // Subscribe to workspace events
  useEffect(() => {
    const unsubscribe = WorkspaceService.onWorkspaceOpened((workspaceId, rootPath) => {
      // Load workspace tree when workspace is opened
      if (currentWorkspace?.workspaceId === workspaceId) {
        loadWorkspaceTree(workspaceId, rootPath);
      }
    });

    return () => {
      unsubscribe();
    };
  }, [currentWorkspace]);

  const loadWorkspaceTree = async (workspaceId: string, rootPath: string) => {
    setLoading(true);
    try {
      const response = await WorkspaceService.getWorkspaceTree({
        workspaceId,
        rootPath
      });
      setWorkspaceTree(response.tree);
    } catch (error) {
      setError(error instanceof Error ? error.message : 'Failed to load workspace tree');
      console.error('Failed to load workspace tree:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleOpenWorkspace = async () => {
    try {
      setLoading(true);
      const dialogResult = await WorkspaceService.openFileDialog();
      
      if (dialogResult.selectedPath) {
        console.log('WorkspaceContainer: Opening workspace at:', dialogResult.selectedPath);
        const result = await WorkspaceService.openWorkspaceAndLoadTree(dialogResult.selectedPath);
        console.log('WorkspaceContainer: Workspace opened:', result.workspace);
        console.log('WorkspaceContainer: Tree received:', result.tree);
        console.log('WorkspaceContainer: Tree nodes:', result.tree.tree);
        console.log('WorkspaceContainer: Tree length:', result.tree.tree.length);
        
        setCurrentWorkspace(result.workspace);
        setWorkspaceTree(result.tree.tree);
        // Expand the root path by default
        toggleExpanded(result.workspace.rootPath);
      }
    } catch (error) {
      setError(error instanceof Error ? error.message : 'Failed to open workspace');
      console.error('Failed to open workspace:', error);
    } finally {
      setLoading(false);
    }
  };

  // If no workspace is open, show empty state
  if (!currentWorkspace) {
    return (
      <div className="h-full flex flex-col items-center justify-center p-8 bg-background">
        <div className="max-w-md text-center">
          <div className="w-16 h-16 mx-auto mb-6 rounded-full bg-muted flex items-center justify-center">
            <svg className="w-8 h-8 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
            </svg>
          </div>
          <h2 className="text-2xl font-semibold mb-2">No Workspace Open</h2>
          <p className="text-muted-foreground mb-6">
            Open a folder to start working with your code. The workspace explorer will show your project structure.
          </p>
          <button
            onClick={handleOpenWorkspace}
            className="px-6 py-3 bg-primary text-primary-foreground rounded-lg font-medium hover:bg-primary/90 transition-colors"
          >
            Open Workspace
          </button>
          <p className="mt-4 text-sm text-muted-foreground">
            You can also use the command palette (Ctrl+P) to open a workspace.
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex h-full">
      {/* Sidebar with workspace explorer */}
      <div className="w-64 border-r border-border bg-sidebar overflow-hidden flex flex-col">
        <div className="px-3 py-2 border-b border-border">
          <div className="flex items-center justify-between">
            <h3 className="text-sm font-semibold truncate" title={currentWorkspace.rootPath}>
              {currentWorkspace.rootPath.split(/[\\/]/).pop() || 'Workspace'}
            </h3>
            <span className="text-xs text-muted-foreground bg-muted px-2 py-1 rounded">
              {currentWorkspace.fileCount} items
            </span>
          </div>
          <p className="text-xs text-muted-foreground truncate mt-1" title={currentWorkspace.rootPath}>
            {currentWorkspace.rootPath}
          </p>
        </div>
        <div className="flex-1 overflow-auto">
          <ExplorerContainer />
        </div>
      </div>
      
      {/* Main editor area */}
      <div className="flex-1 overflow-hidden">
        <EditorContainer filePath={explorerUI.activeFilePath || undefined} />
      </div>
    </div>
  );
}
