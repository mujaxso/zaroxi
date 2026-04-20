import { Icon } from '@/components/ui/Icon';
import { cn } from '@/lib/utils';
import { ExplorerTreeNode } from '../services/workspaceService';

interface ExplorerTreeProps {
  nodes: ExplorerTreeNode[];
  expandedPaths: Set<string>;
  selectedPath: string | null;
  activeFilePath: string | null;
  onNodeClick: (node: ExplorerTreeNode) => void;
  onLoadChildren?: (path: string) => void;
  depth?: number;
}

export function ExplorerTree({
  nodes,
  expandedPaths,
  selectedPath,
  activeFilePath,
  onNodeClick,
  onLoadChildren,
  depth = 0,
}: ExplorerTreeProps) {
  const handleNodeClick = (node: ExplorerTreeNode) => {
    onNodeClick(node);
    
    // If it's a directory and not expanded, load children
    if (node.isDir && !expandedPaths.has(node.path) && onLoadChildren) {
      onLoadChildren(node.path);
    }
  };

  const getFileIcon = (node: ExplorerTreeNode): keyof typeof import('@/lib/theme/nerd-font-icons').nerdFontIcons => {
    if (node.isDir) {
      return expandedPaths.has(node.path) ? 'folder-open' : 'folder';
    }
    
    // File type based icons
    const ext = node.fileType?.toLowerCase();
    if (ext === 'rs') return 'rust';
    if (ext === 'ts' || ext === 'tsx') return 'typescript';
    if (ext === 'js' || ext === 'jsx') return 'javascript';
    if (ext === 'json') return 'file-json';
    if (ext === 'md') return 'file-markdown';
    if (ext === 'toml' || ext === 'yaml' || ext === 'yml') return 'file-config';
    if (ext === 'png' || ext === 'jpg' || ext === 'jpeg' || ext === 'gif' || ext === 'svg') return 'file-image';
    if (ext === 'py') return 'python';
    if (ext === 'go') return 'go';
    if (ext === 'java') return 'java';
    
    return 'file';
  };

  return (
    <div className="select-none">
      {nodes.map((node) => {
        const isExpanded = node.isDir && expandedPaths.has(node.path);
        const isSelected = selectedPath === node.path;
        const isActive = activeFilePath === node.path;
        
        return (
          <div key={node.id} className="relative">
            <div
              className={cn(
                'flex items-center py-1 px-2 hover:bg-accent/50 cursor-pointer transition-colors',
                depth > 0 && 'pl-6',
                isSelected && 'bg-accent',
                isActive && 'bg-primary/10'
              )}
              style={{ paddingLeft: `${depth * 1.5 + 0.5}rem` }}
              onClick={() => handleNodeClick(node)}
              title={node.path}
            >
              <div className="flex items-center flex-1 min-w-0">
                <Icon
                  name={getFileIcon(node)}
                  size={16}
                  className={cn(
                    'mr-2 flex-shrink-0',
                    node.isDir ? 'text-blue-500' : 'text-muted-foreground'
                  )}
                  label={node.isDir ? 'Folder' : 'File'}
                />
                <span className={cn(
                  'truncate text-sm font-sans',
                  isSelected ? 'text-accent-foreground font-medium' : 'text-foreground',
                  isActive && 'text-primary'
                )}>
                  {node.name}
                </span>
              </div>
              {node.isDir && (
                <Icon
                  name={isExpanded ? 'chevron-down' : 'chevron-right'}
                  size={12}
                  className="text-muted-foreground ml-1 flex-shrink-0"
                  label={isExpanded ? 'Expanded' : 'Collapsed'}
                />
              )}
            </div>
            
            {/* Render children if expanded */}
            {node.isDir && isExpanded && node.children && (
              <ExplorerTree
                nodes={node.children}
                expandedPaths={expandedPaths}
                selectedPath={selectedPath}
                activeFilePath={activeFilePath}
                onNodeClick={onNodeClick}
                onLoadChildren={onLoadChildren}
                depth={depth + 1}
              />
            )}
          </div>
        );
      })}
    </div>
  );
}
