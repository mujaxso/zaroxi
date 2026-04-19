import { useEffect, useRef, useState } from 'react';
import { cn } from '@/lib/utils';

interface CodeEditorProps {
  initialValue: string;
  onChange: (value: string) => void;
  language?: string;
  readOnly?: boolean;
  className?: string;
}

export function CodeEditor({
  initialValue,
  onChange,
  language = 'plaintext',
  readOnly = false,
  className,
}: CodeEditorProps) {
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const [value, setValue] = useState(initialValue);

  useEffect(() => {
    setValue(initialValue);
  }, [initialValue]);

  useEffect(() => {
    if (textareaRef.current) {
      // Simple auto-resize
      textareaRef.current.style.height = 'auto';
      textareaRef.current.style.height = `${textareaRef.current.scrollHeight}px`;
    }
  }, [value]);

  const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    const newValue = e.target.value;
    setValue(newValue);
    onChange(newValue);
    
    // Auto-resize
    e.target.style.height = 'auto';
    e.target.style.height = `${e.target.scrollHeight}px`;
  };

  return (
    <div className={cn('relative h-full', className)}>
      <div className="absolute inset-0 bg-editor font-mono text-sm">
        <div className="absolute left-0 top-0 bottom-0 w-8 bg-editor border-r border-border flex flex-col items-center pt-4">
          {Array.from({ length: Math.ceil(value.split('\n').length) }).map((_, i) => (
            <div key={i} className="text-xs text-muted-foreground py-0.5">
              {i + 1}
            </div>
          ))}
        </div>
        <textarea
          ref={textareaRef}
          value={value}
          onChange={handleChange}
          readOnly={readOnly}
          className="w-full h-full bg-transparent text-editor-foreground resize-none outline-none pl-12 pr-4 py-4 leading-relaxed whitespace-pre overflow-auto scrollbar-ide"
          spellCheck="false"
          style={{ tabSize: 2 }}
          placeholder="Start typing..."
        />
      </div>
    </div>
  );
}
