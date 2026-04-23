import { useWorkspaceStore } from '@/features/workspace/stores/useWorkspaceStore';
import { Icon } from '@/components/ui/Icon';
import { cn } from '@/lib/utils';
import { useEffect, useState } from 'react';
import { WorkspaceService } from '@/features/workspace/services/workspaceService';

interface StatusBarProps {
  className?: string;
}

export function StatusBar({ className }: StatusBarProps) {
  const { currentWorkspace, isLoading, explorerUI } = useWorkspaceStore();
  const activeFilePath = explorerUI?.activeFilePath ?? null;
  const [currentTime, setCurrentTime] = useState(new Date());
  const [branchName, setBranchName] = useState<string | null>(null);
  const [fileInfo, setFileInfo] = useState<{
    lineCount?: number;
    charCount?: number;
    largeFileMode?: string;
    contentTruncated?: boolean;
  } | null>(null);

  // Update time every minute
  useEffect(() => {
    const timer = setInterval(() => {
      setCurrentTime(new Date());
    }, 60000); // Update every minute

    return () => clearInterval(timer);
  }, []);

  // Fetch file info when activeFilePath changes
  useEffect(() => {
    let cancelled = false;
    if (activeFilePath) {
      WorkspaceService.openFile({ path: activeFilePath }).then((resp) => {
        if (!cancelled) {
          setFileInfo({
            lineCount: resp.lineCount,
            charCount: resp.charCount,
            largeFileMode: resp.largeFileMode,
            contentTruncated: resp.contentTruncated,
          });
        }
      });
    } else {
      setFileInfo(null);
    }
    return () => {
      cancelled = true;
    };
  }, [activeFilePath]);

  // Format time as HH:MM
  const formattedTime = currentTime.toLocaleTimeString([], { 
    hour: '2-digit', 
    minute: '2-digit',
    hour12: false 
  });

  return (
    <div 
      className={cn(
        "h-7 flex items-center justify-between px-3 text-xs font-sans",
        "text-primary font-medium",
        className
      )}
      style={{
        backgroundColor: 'var(--status-bar-background)',
      }}
    >
      {/* Left section: Workspace info */}
      <div className="flex items-center space-x-4">
        <div className="flex items-center space-x-2">
          <Icon name="workspace" size={12} className="text-primary" label="Workspace" />
          <span className="text-primary font-medium">
            {currentWorkspace ? currentWorkspace.name : 'No workspace'}
          </span>
          {currentWorkspace && (
            <span className="text-primary/80 ml-1 font-mono text-[10px] hidden md:inline">
              ({currentWorkspace.rootPath.split('/').pop()})
            </span>
          )}
        </div>
        
        {isLoading && (
          <div className="flex items-center space-x-2">
            <div className="w-2 h-2 rounded-full bg-accent animate-pulse" />
            <span className="text-primary/80">Loading...</span>
          </div>
        )}

        {/* Git branch info */}
        {branchName && (
          <div className="flex items-center space-x-2">
            <Icon name="git-branch" size={12} className="text-primary" label="Git branch" />
            <span className="text-primary font-medium">{branchName}</span>
          </div>
        )}
      </div>
      
      {/* Center section: File info */}
      <div className="flex items-center space-x-4">
        {fileInfo && (
          <>
            <div className="flex items-center space-x-2">
              <Icon name="file" size={12} className="text-primary" label="File info" />
              <span className="text-primary font-medium">
                {fileInfo.lineCount} lines · {fileInfo.charCount} chars
              </span>
            </div>
            {fileInfo.largeFileMode === 'VeryLarge' && (
              <span className="text-yellow-500 font-medium" title="File is very large – only first part is shown">
                very large
              </span>
            )}
            {fileInfo.largeFileMode === 'Large' && (
              <span className="text-yellow-500 font-medium" title="File is large – may affect performance">
                large
              </span>
            )}
            {fileInfo.contentTruncated && (
              <span className="text-yellow-500 font-semibold" title="Only the first part of the file is shown to keep the editor responsive.">
                truncated
              </span>
            )}
          </>
        )}
      </div>
      
      {/* Right section: Editor status and time */}
      <div className="flex items-center space-x-4 font-mono">
        <div className="flex items-center space-x-2">
          <Icon name="file-code" size={12} className="text-primary" label="Encoding" />
          <span className="text-primary font-medium">UTF-8</span>
        </div>
        <div className="flex items-center space-x-2">
          <Icon name="indent" size={12} className="text-primary" label="Indentation" />
          <span className="text-primary font-medium">Spaces: 2</span>
        </div>
        <div className="flex items-center space-x-2">
          <Icon name="cursor" size={12} className="text-primary" label="Cursor position" />
          <span className="text-primary font-medium">Ln 1, Col 1</span>
        </div>
        <div className="flex items-center space-x-2">
          <Icon name="clock" size={12} className="text-primary" label="Current time" />
          <span className="text-primary font-medium">{formattedTime}</span>
        </div>
      </div>
    </div>
  );
}
