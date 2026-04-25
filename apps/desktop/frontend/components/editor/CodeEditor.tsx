import { useEffect, useRef, useState, useMemo, useCallback } from 'react';
import { cn } from '@/lib/utils';
import { useTabsStore } from '@/features/tabs/store';
import { LineNumberGutter } from './gutter/LineNumberGutter';
import { GUTTER_CONFIG } from './gutter/GutterConfig';

interface CodeEditorProps {
  initialValue: string;
  onChange: (value: string) => void;
  filePath?: string;
  language?: string;
  readOnly?: boolean;
  className?: string;
}

export function CodeEditor({
  initialValue,
  onChange,
  filePath,
  language = 'plaintext',
  readOnly = false,
  className,
}: CodeEditorProps) {
  const [value, setValue] = useState(initialValue);
  const initialRef = useRef(initialValue);

  // Refs for scroll synchronisation
  const textAreaRef = useRef<HTMLTextAreaElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const gutterInnerRef = useRef<HTMLDivElement>(null);

  // Editor state we need to expose to the gutter
  const [cursorLine, setCursorLine] = useState(1);

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

  // Compute logical line count and current cursor line
  const lineCount = useMemo(
    () => (value.match(/\n/g) || []).length + 1,
    [value],
  );

  const lineHeight = GUTTER_CONFIG.LINE_HEIGHT;

  const handleScroll = useCallback(() => {
    const ta = textAreaRef.current;
    if (!ta) return;
    const st = ta.scrollTop;

    // Apply pixel‑perfect transform to the gutter’s inner container (no React re‑render)
    if (gutterInnerRef.current) {
      gutterInnerRef.current.style.transform = `translateY(-${st}px)`;
    }
  }, []);

  const handleSelectionChange = useCallback(() => {
    const ta = textAreaRef.current;
    if (!ta) return;
    const selStart = ta.selectionStart;
    const beforeNewlines = value.slice(0, selStart).match(/\n/g);
    const line = beforeNewlines ? beforeNewlines.length + 1 : 1;
    setCursorLine(line);
  }, [value]);

  const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    if (readOnly) return;
    const newValue = e.target.value;
    setValue(newValue);
    onChange(newValue);
    if (filePath) {
      useTabsStore.getState().markDirty(filePath);
    }
    // Update cursor line after a change (will be refined on selection change)
    handleSelectionChange();
  };

  // Common class for the code area (textarea and pre)
  const codeClass = cn(
    'font-mono text-sm leading-[22px] p-0 hide-scrollbar text-editor-foreground',
  );

  const codeStyle: React.CSSProperties = {
    height: '100%',
    width: '100%',
    margin: 0,
    border: 0,
    padding: 0,
    overflow: 'auto',
    scrollbarWidth: 'none',
    msOverflowStyle: 'none',
    whiteSpace: 'pre',
    wordBreak: 'normal',
  };

  const gutter = (
    <LineNumberGutter
      lineCount={lineCount}
      cursorLine={cursorLine}
      lineHeight={lineHeight}
      innerRef={gutterInnerRef}
    />
  );

  if (readOnly) {
    return (
      <div
        ref={containerRef}
        className={cn('flex flex-row h-full w-full gap-1 bg-editor', className)}
        onScroll={handleScroll}
      >
        {gutter}
        <pre
          ref={textAreaRef as unknown as React.RefObject<HTMLPreElement>}
          className={cn(codeClass, 'bg-editor flex-1')}
          style={{
            ...codeStyle,
            overflow: 'auto',
          }}
        >
          {value}
        </pre>
      </div>
    );
  }

  return (
    <div
      ref={containerRef}
      className={cn('flex flex-row h-full w-full gap-1', className)}
      onScroll={handleScroll}
    >
      {gutter}
      <textarea
        ref={textAreaRef}
        className={cn(
          codeClass,
          'bg-transparent caret-foreground outline-none resize-none flex-1',
        )}
        style={{
          ...codeStyle,
          border: 'none',
        }}
        value={value}
        onChange={handleChange}
        onScroll={handleScroll}
        onSelect={handleSelectionChange}
        spellCheck={false}
        autoComplete="off"
        autoCorrect="off"
        wrap="off"
      />
    </div>
  );
}
