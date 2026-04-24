import { Icon } from '@/components/ui/Icon';
import { cn } from '@/lib/utils';
import { ExplorerTreeNode } from '../services/workspaceService';

interface ExplorerTreeProps {
  nodes: ExplorerTreeNode[];
  expandedPaths: Set<string>;
  selectedPath: string | null;
  activeFilePath: string | null;
  onNodeClick: (node: ExplorerTreeNode) => void;
  depth?: number;
}

export function ExplorerTree({
  nodes,
  expandedPaths,
  selectedPath,
  activeFilePath,
  onNodeClick,
  depth = 0,
}: ExplorerTreeProps) {
  const handleNodeClick = (node: ExplorerTreeNode) => {
    // Allow click for all directories; children will be loaded by the container
    onNodeClick(node);
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
    if (ext === 'c') return 'c';
    if (ext === 'cpp' || ext === 'cc' || ext === 'cxx') return 'cpp';
    if (ext === 'cs') return 'csharp';
    if (ext === 'dart') return 'dart';
    if (ext === 'lua') return 'lua';
    if (ext === 'pl' || ext === 'pm') return 'perl';
    if (ext === 'r' || ext === 'rscript') return 'r';
    if (ext === 'jl') return 'julia';
    if (ext === 'elm') return 'elm';
    if (ext === 'ml' || ext === 'mli') return 'ocaml';
    if (ext === 're' || ext === 'rei') return 'reason';
    if (ext === 'rkt') return 'racket';
    if (ext === 'fs' || ext === 'fsi' || ext === 'fsx') return 'fsharp';
    if (ext === 'vim' || ext === 'vimrc') return 'vim';
    if (ext === 'zig') return 'zig';
    if (ext === 'cr') return 'crystal';
    if (ext === 'hx') return 'haxe';
    if (ext === 'groovy') return 'groovy';
    if (ext === 'm' || ext === 'mat') return 'matlab';
    if (ext === 'purs') return 'purescript';
    if (ext === 'nix') return 'nix';
    if (ext === 'gradle') return 'gradle';
    if (ext === 'cmake') return 'cmake';
    if (ext === 'makefile' || ext === 'mk') return 'makefile';
    if (ext === 'sh' || ext === 'bash' || ext === 'zsh') return 'shell';
    if (ext === 'sol') return 'solidity';
    if (ext === 'vue') return 'vue';
    if (ext === 'svelte') return 'svelte';
    if (ext === 'angular') return 'angular';
    if (ext === 'react') return 'react';
    if (ext === 'ember') return 'ember';
    if (ext === 'backbone') return 'backbone';
    if (ext === 'pug') return 'pug';
    if (ext === 'haml') return 'haml';
    if (ext === 'styl') return 'stylus';
    if (ext === 'postcss') return 'postcss';
    if (ext === 'html' || ext === 'htm') return 'html';
    if (ext === 'css') return 'css';
    if (ext === 'scss' || ext === 'sass') return 'sass';
    if (ext === 'less') return 'less';
    if (ext === 'xml') return 'xml';
    if (ext === 'sql') return 'database-sql';
    if (ext === 'php') return 'php';
    if (ext === 'rb') return 'ruby';
    if (ext === 'swift') return 'swift';
    if (ext === 'kt' || ext === 'kts') return 'kotlin';
    if (ext === 'hs') return 'haskell';
    if (ext === 'ex' || ext === 'exs') return 'elixir';
    if (ext === 'clj' || ext === 'cljs') return 'clojure';
    if (ext === 'erl' || ext === 'hrl') return 'erlang';
    if (ext === 'dockerfile') return 'docker';
    if (ext === 'tf') return 'terraform';
    if (ext === 'yaml' || ext === 'yml') return 'file-yaml';
    
    return 'file';
  };

  return (
    <div className="select-none">
      {nodes.map((node) => {
        const isExpanded = node.isDir && expandedPaths.has(node.path);
        const isSelected = selectedPath === node.path;
        const isActive = activeFilePath === node.path;
        const hasChildren = node.isDir && node.children && node.children.length > 0;
        
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
                    node.isDir ? 'text-blue-500' : 'text-foreground'
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
              {hasChildren && (
                <Icon
                  name={isExpanded ? 'chevron-down' : 'chevron-right'}
                  size={12}
                  className="text-muted-foreground ml-1 flex-shrink-0"
                  label={isExpanded ? 'Expanded' : 'Collapsed'}
                />
              )}
            </div>
            
            {/* Render children only if expanded and children exist */}
            {node.isDir && isExpanded && hasChildren && (
              <ExplorerTree
                nodes={node.children!}
                expandedPaths={expandedPaths}
                selectedPath={selectedPath}
                activeFilePath={activeFilePath}
                onNodeClick={onNodeClick}
                depth={depth + 1}
              />
            )}
          </div>
        );
      })}
    </div>
  );
}
