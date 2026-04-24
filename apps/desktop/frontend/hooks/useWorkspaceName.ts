import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export function useWorkspaceName(): string | null {
  const [workspacePath, setWorkspacePath] = useState<string | null>(null);

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    const fetchWorkspace = async () => {
      try {
        const path = await invoke<string | null>('get_current_workspace_path');
        setWorkspacePath(path);
      } catch {
        setWorkspacePath(null);
      }
    };
    fetchWorkspace();

    const setupListener = async () => {
      try {
        unlisten = await listen<string | null>('workspace-changed', (event) => {
          setWorkspacePath(event.payload);
        });
      } catch {
        // ignore
      }
    };
    setupListener();

    return () => {
      if (unlisten) unlisten();
    };
  }, []);

  return workspacePath;
}
