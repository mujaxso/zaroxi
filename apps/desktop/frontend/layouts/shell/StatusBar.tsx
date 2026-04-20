import { useWorkspaceStore } from '@/features/workspace/stores/useWorkspaceStore';
import { Icon } from '@/components/ui/Icon';

export function StatusBar() {
  const { currentWorkspace, isLoading } = useWorkspaceStore();
  
  return (
    <div className="h-6 border-t border-border bg-sidebar flex items-center justify-between px-3 text-xs font-sans">
      <div className="flex items-center space-x-4">
        <div className="flex items-center space-x-1">
          <Icon name="workspace" size={12} className="text-muted-foreground" label="Workspace" />
          <span className="text-muted-foreground">Workspace:</span>
          <span className="font-medium">
            {currentWorkspace ? currentWorkspace.name : 'No workspace open'}
          </span>
          {currentWorkspace && (
            <span className="text-muted-foreground ml-2 font-mono text-[11px]">
              ({currentWorkspace.rootPath})
            </span>
          )}
        </div>
        
        {isLoading && (
          <div className="flex items-center space-x-1">
            <div className="w-2 h-2 rounded-full bg-blue-500 animate-pulse" />
            <span className="text-muted-foreground">Loading...</span>
          </div>
        )}
      </div>
      
      <div className="flex items-center space-x-4 font-mono">
        <div className="flex items-center space-x-1">
          <Icon name="file-code" size={12} className="text-muted-foreground" label="Encoding" />
          <span className="text-muted-foreground">UTF-8</span>
        </div>
        <div className="flex items-center space-x-1">
          <Icon name="indent" size={12} className="text-muted-foreground" label="Indentation" />
          <span className="text-muted-foreground">Spaces: 2</span>
        </div>
        <div className="flex items-center space-x-1">
          <Icon name="cursor" size={12} className="text-muted-foreground" label="Cursor position" />
          <span className="text-muted-foreground">LN 1, COL 1</span>
        </div>
      </div>
    </div>
  );
}
