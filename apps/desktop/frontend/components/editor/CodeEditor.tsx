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

  // Inject CSS that hides native scrollbars (professional IDE look)
  useEffect(() => {
    const style = document.createElement('style');
    style.innerHTML = `
      .hide-scrollbar::-webkit-scrollbar {
        display: none;
      }
    `;
    document.head.appendChild(style);
    return () => {
      document.head.removeChild(style);
    };
  }, []);

  const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    if (readOnly) return;
    const newValue = e.target.value;
    setValue(newValue);
    onChange(newValue);
  };

  // Common class and style for both read‑only (pre) and editable (textarea)
  const commonClass = cn(
    'relative font-mono text-sm leading-[22px] p-0 hide-scrollbar',
    'text-editor-foreground',
    className,
  );

  const commonStyle: React.CSSProperties = {
    height: '100%',
    width: '100%',
    overflow: 'auto',
    margin: 0,
    border: 0,
    padding: 0,
    scrollbarWidth: 'none',
    msOverflowStyle: 'none',
    wordBreak: 'break-all',
    whiteSpace: 'pre-wrap',
  };

  if (readOnly) {
    return (
      <pre
        className={cn(commonClass, 'bg-editor')}
        style={commonStyle}
      >
        {value}
      </pre>
    );
  }

  return (
    <textarea
      className={cn(
        commonClass,
        'bg-transparent caret-foreground outline-none resize-none',
      )}
      style={{ ...commonStyle, border: 'none' }}
      value={value}
      onChange={handleChange}
      spellCheck={false}
      autoComplete="off"
      autoCorrect="off"
      wrap="on"
    />
  );
}
