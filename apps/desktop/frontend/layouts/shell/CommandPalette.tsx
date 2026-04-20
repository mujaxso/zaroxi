import { useState, useEffect } from 'react';
import { cn } from '@/lib/utils';
import { Icon } from '@/components/ui/Icon';

export function CommandPalette() {
  const [isOpen, setIsOpen] = useState(false);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === 'p') {
        e.preventDefault();
        setIsOpen(true);
      }
      if (e.key === 'Escape') {
        setIsOpen(false);
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, []);

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-start justify-center pt-20">
      <div 
        className="fixed inset-0 bg-black/50" 
        onClick={() => setIsOpen(false)}
      />
      <div className="relative w-full max-w-2xl mx-4">
        <div className="bg-sidebar border border-border rounded-lg shadow-xl overflow-hidden">
          <div className="p-4 border-b border-border">
            <div className="flex items-center">
              <div className="text-muted-foreground mr-3 font-mono">
                ⌘P
              </div>
              <input
                type="text"
                placeholder="Type a command or search..."
                className="flex-1 bg-transparent outline-none text-foreground placeholder:text-muted-foreground font-sans"
                autoFocus
              />
            </div>
          </div>
          <div className="py-2 max-h-96 overflow-auto">
            <div className="px-4 py-2 text-xs font-semibold text-muted-foreground uppercase tracking-wider font-sans">
              Workspace
            </div>
            <button className="w-full px-4 py-2 text-left hover:bg-muted flex items-center font-sans">
              <Icon name="folder-open" size={16} className="mr-3 text-muted-foreground" />
              <span>Open Workspace...</span>
            </button>
            <button className="w-full px-4 py-2 text-left hover:bg-muted flex items-center font-sans">
              <Icon name="file" size={16} className="mr-3 text-muted-foreground" />
              <span>Open File...</span>
            </button>
            <div className="px-4 py-2 text-xs font-semibold text-muted-foreground uppercase tracking-wider mt-4 font-sans">
              Editor
            </div>
            <button className="w-full px-4 py-2 text-left hover:bg-muted flex items-center font-sans">
              <Icon name="save" size={16} className="mr-3 text-muted-foreground" />
              <span>Save File</span>
            </button>
            <button className="w-full px-4 py-2 text-left hover:bg-muted flex items-center font-sans">
              <Icon name="search" size={16} className="mr-3 text-muted-foreground" />
              <span>Find in File</span>
            </button>
          </div>
          <div className="p-3 border-t border-border text-xs text-muted-foreground font-sans">
            Press <kbd className="px-1 py-0.5 bg-muted rounded font-mono">Esc</kbd> to close
          </div>
        </div>
      </div>
    </div>
  );
}
