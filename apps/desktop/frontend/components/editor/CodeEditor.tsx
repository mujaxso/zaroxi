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
  const [value, setValue] = useState(initialValue);
  const initialRef = useRef(initialValue);

  // Sync when the parent supplies a new `initialValue`
  useEffect(() => {
    if (initialRef.current !== initialValue) {
      initialRef.current = initialValue;
      setValue(initialValue);
    }
  }, [initialValue]);

  const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    if (readOnly) return;
    const newValue = e.target.value;
    setValue(newValue);
    onChange(newValue);
  };

  if (readOnly) {
    return (
      <pre
        className={cn(
          'relative h-full w-full overflow-auto p-0 font-mono text-sm leading-[22px]',
          'bg-editor text-editor-foreground',
          className
        )}
      >
        {value}
      </pre>
    );
  }

  return (
    <textarea
      className={cn(
        'relative h-full w-full resize-none font-mono text-sm leading-[22px] p-0',
        'bg-transparent text-editor-foreground caret-foreground outline-none',
        className
      )}
      value={value}
      onChange={handleChange}
      spellCheck={false}
      autoComplete="off"
      autoCorrect="off"
      wrap="off"
    />
  );
}
