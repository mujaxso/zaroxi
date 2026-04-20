import { CodeEditor } from '@/components/editor/CodeEditor';

export function EditorContainer() {
  const handleEditorChange = (value: string) => {
    console.log('Editor content changed:', value.length, 'chars');
  };

  const handleEditorSave = () => {
    console.log('Saving editor content');
  };

  return (
    <div className="h-full flex flex-col">
      <div className="border-b border-border px-4 py-2 flex items-center justify-between">
        <div className="text-sm font-medium">editor.rs</div>
        <div className="flex items-center space-x-2">
          <button
            onClick={handleEditorSave}
            className="px-3 py-1 text-xs bg-primary text-primary-foreground rounded hover:bg-primary/90"
          >
            Save
          </button>
        </div>
      </div>
      <div className="flex-1 overflow-hidden">
        <CodeEditor
          initialValue={`// Welcome to Zaroxi Editor
// This is a placeholder for the actual editor

fn main() {
    println!("Hello, Zaroxi!");
}`}
          onChange={handleEditorChange}
          language="rust"
          readOnly={false}
        />
      </div>
    </div>
  );
}
