import { bridge } from '@/lib/bridge';

// Domain types (mirror Rust DTOs)
export interface OpenWorkspaceRequest {
  path: string;
}

export interface OpenWorkspaceResponse {
  workspaceId: string;
  rootPath: string;
  fileCount: number;
}

export interface ListDirectoryRequest {
  path: string;
}

export interface DirectoryEntryDto {
  path: string;
  name: string;
  isDir: boolean;
  fileType?: string;
  size?: number;
  modified?: string;
}

export interface OpenFileRequest {
  path: string;
}

export interface OpenFileResponse {
  content: string;
  language?: string;
}

export interface SaveFileRequest {
  path: string;
  content: string;
}

export interface OpenDialogResponse {
  selectedPath?: string;
}

// Workspace events
export interface WorkspaceEvent {
  type: 'workspace_opened' | 'workspace_closed' | 'directory_changed';
  payload: unknown;
}

/**
 * WorkspaceService - feature-specific business operations
 * 
 * This layer:
 * - Orchestrates multiple bridge calls
 * - Handles business logic that spans multiple operations
 * - Provides a clean API for containers/stores
 */
export class WorkspaceService {
  // Command operations
  static async openWorkspace(request: OpenWorkspaceRequest): Promise<OpenWorkspaceResponse> {
    return await bridge.invoke<OpenWorkspaceResponse>('open_workspace', { request });
  }

  static async listDirectory(request: ListDirectoryRequest): Promise<DirectoryEntryDto[]> {
    return await bridge.invoke<DirectoryEntryDto[]>('list_directory', { request });
  }

  static async openFile(request: OpenFileRequest): Promise<OpenFileResponse> {
    return await bridge.invoke<OpenFileResponse>('open_file', { request });
  }

  static async saveFile(request: SaveFileRequest): Promise<void> {
    return await bridge.invoke<void>('save_file', { request });
  }

  static async openFileDialog(): Promise<OpenDialogResponse> {
    return await bridge.invoke<OpenDialogResponse>('open_file_dialog');
  }

  // Event subscriptions
  static onWorkspaceOpened(handler: (workspaceId: string) => void) {
    return bridge.listen<{ workspaceId: string }>('workspace:opened', (event) => {
      handler(event.workspaceId);
    });
  }

  static onDirectoryChanged(handler: (path: string) => void) {
    return bridge.listen<{ path: string }>('workspace:directory_changed', (event) => {
      handler(event.path);
    });
  }

  // Business operations (combine multiple commands)
  static async openWorkspaceAndLoadRoot(
    path: string
  ): Promise<{ workspace: OpenWorkspaceResponse; rootEntries: DirectoryEntryDto[] }> {
    const workspace = await this.openWorkspace({ path });
    const rootEntries = await this.listDirectory({ path: workspace.rootPath });
    
    return { workspace, rootEntries };
  }
}
