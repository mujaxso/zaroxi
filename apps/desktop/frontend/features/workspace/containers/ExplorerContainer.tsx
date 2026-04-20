import { useEffect } from 'react';
import { ExplorerTree } from '../components/ExplorerTree';
import { useWorkspaceStore } from '../stores/useWorkspaceStore';
import { WorkspaceService } from '../services/workspaceService';

export function ExplorerContainer() {
  const {
    currentWorkspace,
    workspaceTree,
    explorerUI,
    isLoading,
    toggleExpanded,
    setSelectedPath,
    setActiveFilePath,
    setWorkspaceTree,
    setCurrentWorkspace,
    setLoading,
    setError,
  } = useWorkspaceStore();

  const handleOpenWorkspace = async () => {
    try {
      setLoading(true);
      setError(null);
      console.log('[ExplorerContainer] Calling openFileDialog...');
      const dialogResult = await WorkspaceService.openFileDialog();
      console.log('[ExplorerContainer] Dialog result:', dialogResult);
      console.log('[ExplorerContainer] Dialog result selectedPath:', dialogResult.selectedPath);
      
      if (dialogResult.selectedPath) {
        console.log('[ExplorerContainer] Opening workspace at:', dialogResult.selectedPath);
        try {
          const workspace = await WorkspaceService.openWorkspace({ path: dialogResult.selectedPath });
          console.log('[ExplorerContainer] Workspace opened:', workspace);
          
          const tree = await WorkspaceService.getWorkspaceTree({
            workspaceId: workspace.workspaceId,
            rootPath: workspace.rootPath
          });
          console.log('[ExplorerContainer] Tree received with', tree.tree.length, 'nodes');
          console.log('[ExplorerContainer] Tree sample:', tree.tree.slice(0, 3));
          
          setCurrentWorkspace(workspace);
          setWorkspaceTree(tree.tree);
          // Expand the root path by default
          toggleExpanded(workspace.rootPath);
        } catch (error) {
          console.error('[ExplorerContainer] Error:', error);
          throw error;
        }
      } else {
        console.log('[ExplorerContainer] No path selected - dialog was cancelled or failed');
        setError('No directory selected. The file dialog may have been cancelled or encountered an issue. If you\'re using Wayland (Hyprland), ensure xdg-desktop-portal is installed and running. Try installing xdg-desktop-portal-gtk or xdg-desktop-portal-kde.');
      }
    } catch (error) {
      const errorMsg = error instanceof Error ? error.message : 'Failed to open workspace';
      setError(errorMsg);
      console.error('[ExplorerContainer] Failed to open workspace:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleNodeClick = async (node: ExplorerTreeNode) => {
    setSelectedPath(node.path);
    
    if (node.isDir) {
      toggleExpanded(node.path);
    } else {
      // Open file in editor
      setActiveFilePath(node.path);
      
      // Check if we're in Tauri environment - use multiple detection methods
      const isTauri = 
        typeof window !== 'undefined' && 
        (window.__TAURI__ !== undefined || 
         (window as any).__TAURI_INTERNALS__ !== undefined ||
         navigator.userAgent.includes('Tauri'));
      
      if (!isTauri) {
        console.warn('[ExplorerContainer] Not in Tauri environment - file operations disabled');
        setError('File operations require running the app through Tauri (npm run tauri dev)');
        return;
      }
      
      try {
        await WorkspaceService.openFileInEditor(node.path);
      } catch (error) {
        setError(error instanceof Error ? error.message : 'Failed to open file');
        console.error('Failed to open file:', error);
      }
    }
  };

  const handleLoadChildren = async (path: string) => {
    setLoading(true);
    try {
      const children = await WorkspaceService.loadDirectoryChildren(path);
      // Update the tree with loaded children
      const updateTree = (nodes: ExplorerTreeNode[]): ExplorerTreeNode[] => {
        return nodes.map(node => {
          if (node.path === path && node.isDir) {
            return {
              ...node,
              children: children.map(child => ({
                id: child.path,
                path: child.path,
                name: child.name,
                isDir: child.isDir,
                fileType: child.fileType,
                size: child.size,
                modified: child.modified,
                children: child.isDir ? [] : undefined,
                parentPath: path,
              }))
            };
          }
          if (node.children) {
            return {
              ...node,
              children: updateTree(node.children)
            };
          }
          return node;
        });
      };
      
      setWorkspaceTree(prevTree => updateTree(prevTree));
    } catch (error) {
      setError(error instanceof Error ? error.message : 'Failed to load directory children');
      console.error('Failed to load directory children:', error);
    } finally {
      setLoading(false);
    }
  };

  // If no workspace is open, show open workspace button
  if (!currentWorkspace) {
    return (
      <div className="h-full flex flex-col items-center justify-center p-8">
        <div className="max-w-md text-center">
          <div className="w-16 h-16 mx-auto mb-6 rounded-full bg-muted flex items-center justify-center">
            <svg className="w-8 h-8 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
            </svg>
          </div>
          <h2 className="text-lg font-semibold mb-2">No Workspace Open</h2>
          <p className="text-sm text-muted-foreground mb-6">
            Open a folder to browse files and folders in the explorer.
          </p>
          <div className="space-y-3">
            <button
              onClick={handleOpenWorkspace}
              disabled={isLoading}
              className="w-full px-4 py-2 bg-primary text-primary-foreground rounded-md font-medium hover:bg-primary/90 transition-colors text-sm disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {isLoading ? 'Opening...' : 'Open Workspace (Dialog)'}
            </button>
            <div className="text-xs text-muted-foreground text-center">or</div>
            <div className="flex space-x-2">
              <input
                type="text"
                placeholder="/path/to/workspace"
                className="flex-1 px-3 py-1 text-sm border border-border rounded bg-background"
                id="manual-path-input"
              />
              <button
                onClick={async () => {
                  const input = document.getElementById('manual-path-input') as HTMLInputElement;
                  const path = input.value.trim();
                  if (path) {
                    setLoading(true);
                    setError(null);
                    try {
                      const workspace = await WorkspaceService.openWorkspace({ path });
                      const tree = await WorkspaceService.getWorkspaceTree({
                        workspaceId: workspace.workspaceId,
                        rootPath: workspace.rootPath
                      });
                      setCurrentWorkspace(workspace);
                      setWorkspaceTree(tree.tree);
                      toggleExpanded(workspace.rootPath);
                    } catch (err) {
                      setError(err instanceof Error ? err.message : 'Failed to open workspace');
                    } finally {
                      setLoading(false);
                    }
                  }
                }}
                disabled={isLoading}
                className="px-3 py-1 text-sm bg-muted text-muted-foreground rounded hover:bg-muted/80 transition-colors disabled:opacity-50"
              >
                Open
              </button>
            </div>
          </div>
          <p className="mt-4 text-xs text-muted-foreground">
            You can also use the folder icon in the activity rail.
          </p>
        </div>
      </div>
    );
  }

  console.log('ExplorerContainer render:', {
    currentWorkspace: currentWorkspace ? {
      workspaceId: currentWorkspace.workspaceId,
      rootPath: currentWorkspace.rootPath,
      fileCount: currentWorkspace.fileCount
    } : null,
    workspaceTreeLength: workspaceTree.length,
    isLoading,
    hasWorkspace: !!currentWorkspace
  });

  // Loading state
  if (isLoading) {
    return (
      <div className="p-4">
        <div className="space-y-2">
          <div className="h-4 bg-muted rounded animate-pulse w-3/4"></div>
          <div className="h-4 bg-muted rounded animate-pulse w-1/2"></div>
          <div className="h-4 bg-muted rounded animate-pulse w-5/6"></div>
          <div className="h-4 bg-muted rounded animate-pulse w-2/3"></div>
          <div className="h-4 bg-muted rounded animate-pulse w-4/5"></div>
        </div>
        <p className="text-xs text-muted-foreground mt-4 text-center">Loading workspace tree...</p>
      </div>
    );
  }

  // Workspace is open but tree is empty (could be empty directory or error)
  if (currentWorkspace && workspaceTree.length === 0) {
    return (
      <div className="p-8 text-center text-muted-foreground">
        <p>No files found in workspace.</p>
        <p className="text-sm mt-2">Path: {currentWorkspace.rootPath}</p>
        <p className="text-xs mt-1">File count: {currentWorkspace.fileCount}</p>
        <button
          onClick={handleOpenWorkspace}
          className="mt-4 px-4 py-2 text-sm bg-muted hover:bg-muted/80 rounded-md transition-colors"
        >
          Open Different Workspace
        </button>
      </div>
    );
  }

  return (
    <div className="py-2">
      <ExplorerTree
        nodes={workspaceTree}
        expandedPaths={explorerUI.expandedPaths}
        selectedPath={explorerUI.selectedPath}
        activeFilePath={explorerUI.activeFilePath}
        onNodeClick={handleNodeClick}
        onLoadChildren={handleLoadChildren}
      />
    </div>
  );
}
