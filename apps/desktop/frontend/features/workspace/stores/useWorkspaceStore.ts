import { create } from 'zustand';
import { devtools, persist } from 'zustand/middleware';

interface Workspace {
  id: string;
  name: string;
  rootPath: string;
}

interface DirectoryEntryDto {
  path: string;
  name: string;
  isDir: boolean;
  fileType?: string;
}

interface WorkspaceStore {
  // State
  currentWorkspace: Workspace | null;
  fileTree: DirectoryEntryDto[];
  isLoading: boolean;
  error: string | null;
  
  // Actions
  openWorkspace: (path: string) => Promise<void>;
  refreshFileTree: () => Promise<void>;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
}

export const useWorkspaceStore = create<WorkspaceStore>()(
  devtools(
    persist(
      (set, get) => ({
        currentWorkspace: null,
        fileTree: [],
        isLoading: false,
        error: null,
        
        openWorkspace: async (path: string) => {
          set({ isLoading: true, error: null });
          try {
            // TODO: Replace with actual Tauri command
            await new Promise(resolve => setTimeout(resolve, 1000));
            
            const workspace: Workspace = {
              id: '1',
              name: path.split('/').pop() || 'Workspace',
              rootPath: path,
            };
            
            set({ 
              currentWorkspace: workspace,
              isLoading: false 
            });
            
            // Refresh file tree after opening
            await get().refreshFileTree();
          } catch (error) {
            set({ 
              error: error instanceof Error ? error.message : 'Unknown error',
              isLoading: false 
            });
          }
        },
        
        refreshFileTree: async () => {
          const { currentWorkspace } = get();
          if (!currentWorkspace) return;
          
          try {
            // TODO: Replace with actual Tauri command
            await new Promise(resolve => setTimeout(resolve, 500));
            
            const entries: DirectoryEntryDto[] = [
              { path: `${currentWorkspace.rootPath}/Cargo.toml`, name: 'Cargo.toml', isDir: false, fileType: 'toml' },
              { path: `${currentWorkspace.rootPath}/src`, name: 'src', isDir: true },
              { path: `${currentWorkspace.rootPath}/README.md`, name: 'README.md', isDir: false, fileType: 'markdown' },
            ];
            
            set({ fileTree: entries });
          } catch (error) {
            console.error('Failed to refresh file tree:', error);
          }
        },
        
        setLoading: (loading) => set({ isLoading: loading }),
        setError: (error) => set({ error }),
      }),
      {
        name: 'workspace-storage',
        partialize: (state) => ({
          currentWorkspace: state.currentWorkspace,
        }),
      }
    )
  )
);
