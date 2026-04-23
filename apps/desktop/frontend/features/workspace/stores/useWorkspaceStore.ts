import { create } from 'zustand';
import { devtools, persist } from 'zustand/middleware';
import { WorkspaceService, type ExplorerTreeNode, type OpenWorkspaceResponse } from '../services/workspaceService';

// UI-only state types
export interface WorkspaceUI {
  id: string;
  name: string;
  rootPath: string;
}

export interface ExplorerUIState {
  // Expanded paths in the tree
  expandedPaths: Set<string>;
  // Selected file/folder path
  selectedPath: string | null;
  // Active file path (open in editor)
  activeFilePath: string | null;
}

export interface WorkspaceStoreState {
  // Backend-driven state
  currentWorkspace: OpenWorkspaceResponse | null;
  workspaceTree: ExplorerTreeNode[];
  
  // UI state
  explorerUI: ExplorerUIState;
  
  // Loading states
  isLoading: boolean;
  error: string | null;
  
  // UI actions for explorer
  toggleExpanded: (path: string) => void;
  setSelectedPath: (path: string | null) => void;
  setActiveFilePath: (path: string | null) => void;
  
  // Backend state setters (called by services)
  setCurrentWorkspace: (workspace: OpenWorkspaceResponse | null) => void;
  setWorkspaceTree: (tree: ExplorerTreeNode[]) => void;
  
  // Utility
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  
  // Derived state getters
  isPathExpanded: (path: string) => boolean;
}

/**
 * WorkspaceStore - Manages both UI state and backend-driven state
 * 
 * This store:
 * - Manages UI state (expanded nodes, selection)
 * - Stores backend-driven workspace data
 * - Provides actions to update both
 */
export const useWorkspaceStore = create<WorkspaceStoreState>()(
  devtools(
    persist(
      (set, get) => ({
        currentWorkspace: null,
        workspaceTree: [],
        explorerUI: {
          expandedPaths: new Set<string>(),
          selectedPath: null,
          activeFilePath: null,
        },
        isLoading: false,
        error: null,
        
        // UI actions
        toggleExpanded: (path: string) => set((state) => {
          const newExpanded = new Set(state.explorerUI.expandedPaths);
          if (newExpanded.has(path)) {
            newExpanded.delete(path);
          } else {
            newExpanded.add(path);
          }
          return {
            explorerUI: {
              ...state.explorerUI,
              expandedPaths: newExpanded,
            },
          };
        }),
        
        setSelectedPath: (path: string | null) => set((state) => ({
          explorerUI: {
            ...state.explorerUI,
            selectedPath: path,
          },
        })),
        
        setActiveFilePath: (path: string | null) => set((state) => ({
          explorerUI: {
            ...state.explorerUI,
            activeFilePath: path,
          },
        })),
        
        // Backend state setters
        setCurrentWorkspace: (workspace) => {
          console.log('[WorkspaceStore] setCurrentWorkspace called with:', workspace);
          set({ 
            currentWorkspace: workspace,
            // Reset UI state when workspace changes
            explorerUI: {
              expandedPaths: new Set<string>(),
              selectedPath: null,
              activeFilePath: null,
            }
          });
        },
        setWorkspaceTree: (tree) => {
          console.log('[WorkspaceStore] setWorkspaceTree called with tree of length:', tree.length);
          if (tree.length > 0) {
            console.log('[WorkspaceStore] First few nodes:', tree.slice(0, 3));
          } else {
            console.log('[WorkspaceStore] Tree is empty');
          }
          set({ workspaceTree: tree });
        },
        
        // Utility
        setLoading: (loading) => set({ isLoading: loading }),
        setError: (error) => set({ error }),
        
        // Derived state
        isPathExpanded: (path: string) => {
          return get().explorerUI.expandedPaths.has(path);
        },
      }),
      {
        name: 'workspace-ui-storage',
        partialize: (state) => ({
          // Only persist UI state (excluding activeFilePath to avoid reopening large files)
          explorerUI: {
            expandedPaths: Array.from(state.explorerUI.expandedPaths),
            selectedPath: state.explorerUI.selectedPath,
            // activeFilePath is intentionally excluded to prevent reopening large files on startup
          },
        }),
        merge: (persistedState: any, currentState: any) => {
          // Convert expandedPaths array back to Set
          if (persistedState?.explorerUI?.expandedPaths) {
            persistedState.explorerUI.expandedPaths = new Set(persistedState.explorerUI.expandedPaths);
          }
          return {
            ...currentState,
            ...persistedState,
          };
        },
      }
    )
  )
);
