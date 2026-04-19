import { Icon } from '@/components/ui/Icon';
import { cn } from '@/lib/utils';

interface FileItem {
  path: string;
  name: string;
  isDir: boolean;
  fileType?: string;
}

interface WorkspaceExplorerProps {
  files: FileItem[];
  isLoading: boolean;
  onOpenFile: (path: string) => void;
  onOpenFolder: (path: string) => void;
}

export function WorkspaceExplorer({ 
  files, 
  isLoading, 
  onOpenFile, 
  onOpenFolder 
}: WorkspaceExplorerProps) {
  if (isLoading) {
    return (
      <div className="p-4">
        <div className="animate-pulse space-y-2">
          <div className="h-4 bg-muted rounded w-3/4"></div>
          <div className="h-4 bg-muted rounded w-1/2"></div>
        </div>
      </div>
    );
  }

  if (files.length === 0) {
    return (
      <div className="p-4 text-center text-muted-foreground">
        <p>No files to display</p>
        <p className="text-sm">Open a workspace to get started</p>
      </div>
    );
  }

  return (
    <div className="py-2">
      <div className="px-3 py-2 text-xs font-semibold text-muted-foreground uppercase tracking-wider">
        Explorer
      </div>
      <div className="space-y-1">
        {files.map((file) => (
          <button
            key={file.path}
            onClick={() => file.isDir ? onOpenFolder(file.path) : onOpenFile(file.path)}
            className={cn(
              'w-full flex items-center px-3 py-1.5 text-sm hover:bg-muted transition-colors',
              'focus:outline-none focus:bg-muted'
            )}
          >
            <Icon 
              name={file.isDir ? 'folder' : 'file'} 
              size={16}
              className="mr-2 text-muted-foreground"
            />
            <span className="truncate">{file.name}</span>
            {file.fileType && (
              <span className="ml-auto text-xs text-muted-foreground">
                {file.fileType}
              </span>
            )}
          </button>
        ))}
      </div>
    </div>
  );
}
