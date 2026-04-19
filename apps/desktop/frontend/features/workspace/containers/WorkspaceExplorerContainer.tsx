import { useWorkspaceStore } from '../stores/useWorkspaceStore';
import { WorkspaceExplorer } from '../components/WorkspaceExplorer';
import { useEffect } from 'react';

export function WorkspaceExplorerContainer() {
  const { fileTree, isLoading, refreshFileTree, currentWorkspace } = useWorkspaceStore();

  useEffect(() => {
    if (currentWorkspace) {
      refreshFileTree();
    }
  }, [currentWorkspace, refreshFileTree]);

  const handleOpenFile = (path: string) => {
    console.log('Opening file:', path);
    // TODO: Open file in editor
  };

  const handleOpenFolder = (path: string) => {
    console.log('Opening folder:', path);
    // TODO: Navigate into folder
  };

  return (
    <WorkspaceExplorer
      files={fileTree}
      isLoading={isLoading}
      onOpenFile={handleOpenFile}
      onOpenFolder={handleOpenFolder}
    />
  );
}
