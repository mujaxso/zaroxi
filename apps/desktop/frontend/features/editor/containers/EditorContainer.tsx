import { useState, useEffect } from 'react';
import { CodeEditor } from '@/components/editor/CodeEditor';
import { WorkspaceService } from '@/features/workspace/services/workspaceService';
import { useWorkspaceStore } from '@/features/workspace/stores/useWorkspaceStore';
import { Icon } from '@/components/ui/Icon';

export function EditorContainer() {
  const [content, setContent] = useState<string>('');
  const [language, setLanguage] = useState<string>('plaintext');
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [fileName, setFileName] = useState<string>('editor');
  const [fileInfo, setFileInfo] = useState<{
    lineCount?: number;
    charCount?: number;
    largeFileMode?: string;
    contentTruncated?: boolean;
  }>({});
  
  // Get active file path from workspace store
  const { explorerUI } = useWorkspaceStore();
  const activeFilePath = explorerUI.activeFilePath;

  useEffect(() => {
    if (activeFilePath) {
      loadFile(activeFilePath);
    } else {
      // Default placeholder content
      setContent(`// Welcome to Zaroxi Editor
// Open a file from the workspace explorer to start editing

// Or create a new file using the command palette (Ctrl+P)`);
      setLanguage('rust');
      setFileName('editor.rs');
      setFileInfo({});
    }
    
    // Add keyboard shortcut for save (Ctrl+S)
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key === 's') {
        e.preventDefault();
        if (activeFilePath) {
          handleEditorSave();
        }
      }
    };
    
    window.addEventListener('keydown', handleKeyDown);
    return () => {
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, [activeFilePath]);


  const loadFile = async (path: string) => {
    setIsLoading(true);
    setContent('');  // Clear previous content to avoid ghosting
    try {
      const response = await WorkspaceService.openFile({ path });
      setContent(response.content);
      setLanguage(response.language || 'plaintext');
      setFileName(path.split(/[\\/]/).pop() || 'file');
      setFileInfo({
        lineCount: response.lineCount,
        charCount: response.charCount,
        largeFileMode: response.largeFileMode,
        contentTruncated: response.contentTruncated,
      });
    } catch (error) {
      // Failed to load file
      setContent(`// Error loading file: ${error instanceof Error ? error.message : 'Unknown error'}`);
      setLanguage('plaintext');
      setFileName('error.txt');
      setFileInfo({});
    } finally {
      setIsLoading(false);
    }
  };

  const handleEditorChange = (value: string) => {
    setContent(value);
  };

  const handleEditorSave = async () => {
    if (!activeFilePath) {
      // No file path to save to
      return;
    }
    
    try {
      await WorkspaceService.saveFile({
        path: activeFilePath,
        content: content,
      });
      // File saved successfully
      // Show a temporary success message
      const saveBtn = document.querySelector('.save-button');
      if (saveBtn) {
        const originalText = saveBtn.textContent;
        saveBtn.textContent = 'Saved!';
        saveBtn.classList.add('bg-green-500');
        setTimeout(() => {
          if (saveBtn.textContent === 'Saved!') {
            saveBtn.textContent = originalText;
            saveBtn.classList.remove('bg-green-500');
          }
        }, 1000);
      }
    } catch (error) {
      // Failed to save file
      const saveBtn = document.querySelector('.save-button');
      if (saveBtn) {
        const originalText = saveBtn.textContent;
        saveBtn.textContent = 'Error!';
        saveBtn.classList.add('bg-red-500');
        setTimeout(() => {
          if (saveBtn.textContent === 'Error!') {
            saveBtn.textContent = originalText;
            saveBtn.classList.remove('bg-red-500');
          }
        }, 1000);
      }
    }
  };

  return (
    <div className="h-full flex flex-col bg-editor min-h-0 w-full min-w-0">
      <div className="border-b border-divider px-4 py-2 flex items-center justify-between bg-activity-rail h-9">
        <div className="flex items-center space-x-3 min-w-0">
          <div className="flex items-center space-x-2">
            <Icon name="file" size={14} className="text-primary" />
            <span className="text-sm font-semibold text-primary leading-none">{fileName}</span>
            {isLoading && (
              <div className="w-2 h-2 rounded-full bg-accent animate-pulse" />
            )}
          </div>
          {activeFilePath && (
            <span className="text-xs text-primary/80 font-mono truncate max-w-[180px] leading-none" title={activeFilePath}>
              {activeFilePath}
            </span>
          )}
        </div>
        <div className="flex items-center space-x-2 flex-shrink-0">
          {activeFilePath && (
            <button
              onClick={handleEditorSave}
              className="save-button px-2.5 py-1 text-xs bg-accent text-on-accent rounded hover:bg-accent-hover transition-colors flex items-center space-x-1.5 font-medium leading-none"
            >
              <Icon name="save" size={12} />
              <span>Save</span>
            </button>
          )}
        </div>
      </div>
      <div className="flex-1 overflow-hidden code-editor-font min-h-0 bg-editor w-full min-w-0">
        <CodeEditor
          key={activeFilePath || 'editor'}
          initialValue={content}
          onChange={handleEditorChange}
          language={language}
          readOnly={false}
        />
      </div>
    </div>
  );
}
