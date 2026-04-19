import { useWorkspaceStore } from '@/features/workspace/stores/useWorkspaceStore';

export function StatusBar() {
  const { currentWorkspace, isLoading } = useWorkspaceStore();
  
  return (
    <div className="h-6 border-t border-border bg-sidebar flex items-center justify-between px-3 text-xs">
      <div className="flex items-center space-x-4">
        <div className="flex items-center space-x-1">
          <span className="text-muted-foreground">Workspace:</span>
          <span className="font-medium">
            {currentWorkspace ? currentWorkspace.name : 'No workspace open'}
          </span>
        </div>
        
        {isLoading && (
          <div className="flex items-center space-x-1">
            <div className="w-2 h-2 rounded-full bg-blue-500 animate-pulse" />
            <span className="text-muted-foreground">Loading...</span>
          </div>
        )}
      </div>
      
      <div className="flex items-center space-x-4">
        <div className="text-muted-foreground">
          UTF-8
        </div>
        <div className="text-muted-foreground">
          Spaces: 2
        </div>
        <div className="text-muted-foreground">
          LN 1, COL 1
        </div>
      </div>
    </div>
  );
}
