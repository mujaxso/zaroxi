import { useEffect, useRef, useState, useCallback, useMemo } from 'react';
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

/** Maximum number of lines we allow to be rendered in the textarea at once.
 * Files exceeding this limit will show a preview of the first lines and forbid editing. */
const MAX_VISIBLE_LINES = 10_000;

function countLines(text: string): number {
  let lines = 1;
  for (let i = 0; i < text.length; i++) {
    if (text.charCodeAt(i) === 10) lines++;
  }
  return lines;
}

/** Return a substring that contains at most `maxLines` lines. */
function truncateToNLines(text: string, maxLines: number): string {
  let newlineCount = 0;
  for (let i = 0; i < text.length; i++) {
    if (text.charCodeAt(i) === 10) {
      newlineCount++;
      if (newlineCount >= maxLines) {
        // Include the newline we just counted, then stop.
        return text.slice(0, i + 1);
      }
    }
  }
  return text; // not enough lines to truncate
}

export function CodeEditor({
  initialValue,
  onChange,
  filePath,
  language = 'plaintext',
  readOnly = false,
  className,
}: CodeEditorProps) {
  // Determine whether the file is too large to edit safely.
  const [isLarge, setIsLarge] = useState<boolean>(false);
  const [displayValue, setDisplayValue] = useState<string>(() => {
    if (countLines(initialValue) > MAX_VISIBLE_LINES) {
      setIsLarge(true);
      return truncateToNLines(initialValue, MAX_VISIBLE_LINES);
    }
    return initialValue;
  });

  // The whole original content is stored so that `onChange` receives it even when truncated.
  const fullValueRef = useRef(initialValue);
  const initialRef = useRef(initialValue);

  // Refs for scroll synchronisation
  const textAreaRef = useRef<HTMLTextAreaElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const gutterInnerRef = useRef<HTMLDivElement>(null);

  // Editor state for the gutter (only cursor line, no scroll state)
  const [cursorLine, setCursorLine] = useState(() => {
    const beforeNewlines = initialValue.slice(0, 0).match(/\n/g);
    return beforeNewlines ? beforeNewlines.length + 1 : 1;
  });

  // Update displayValue when initialValue changes from the outside
  useEffect(() => {
    if (initialRef.current !== initialValue) {
      initialRef.current = initialValue;
      fullValueRef.current = initialValue;
      const newLineCount = countLines(initialValue);
      if (newLineCount > MAX_VISIBLE_LINES) {
        setIsLarge(true);
        setDisplayValue(truncateToNLines(initialValue, MAX_VISIBLE_LINES));
      } else {
        setIsLarge(false);
        setDisplayValue(initialValue);
      }
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

  const lineHeight = GUTTER_CONFIG.LINE_HEIGHT;

  // Handle scroll: directly set the gutter container transform, no React state updates
  const handleScroll = useCallback(() => {
    const ta = textAreaRef.current;
    if (!ta) return;
    const st = ta.scrollTop;

    // Move the gutter inner container directly (no re‑render)
    if (gutterInnerRef.current) {
      gutterInnerRef.current.style.transform = `translateY(-${st}px)`;
    }
  }, []);

  // No rAF, no scrollTop state, no containerHeight state for the gutter.

  const handleSelectionChange = useCallback(() => {
    const ta = textAreaRef.current;
    if (!ta) return;
    const selStart = ta.selectionStart;
    // Count newlines in the displayed (truncated) text only
    const beforeNewlines = displayValue.slice(0, selStart).match(/\n/g);
    const line = beforeNewlines ? beforeNewlines.length + 1 : 1;
    setCursorLine(line);
  }, [displayValue]);

  const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    // If the file was too large, editing is disabled.
    if (readOnly || isLarge) return;
    const newValue = e.target.value;
    // Keep track of the full content (may be larger than displayed, but here editing is only when file fits)
    fullValueRef.current = newValue;
    setDisplayValue(newValue);
    onChange(newValue);
    if (filePath) {
      useTabsStore.getState().markDirty(filePath);
    }
    handleSelectionChange();
  };

  // Compute the line count of the *displayed* content (used for gutter)
  const displayLineCount = useMemo(() => countLines(displayValue), [displayValue]);

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
      lineCount={displayLineCount}
      cursorLine={cursorLine}
      lineHeight={lineHeight}
      ref={gutterInnerRef}
    />
  );

  // Large‑file message (Read‑only preview)
  const largeFileBanner = isLarge ? (
    <div className="absolute bottom-0 left-0 right-0 bg-editor z-10 p-2 text-xs text-muted-foreground border-t border-[rgba(128,128,128,0.18)]">
      File is larger than {MAX_VISIBLE_LINES.toLocaleString()} lines.
      Showing a read‑only preview of the first {MAX_VISIBLE_LINES.toLocaleString()} lines.
    </div>
  ) : null;

  // Because the actual rendered content (displayValue) may be shorter than fullValueRef,
  // we need to tell the user the editor is read‑only when isLarge is true.
  const effectiveReadOnly = readOnly || isLarge;

  if (effectiveReadOnly) {
    return (
      <div
        ref={containerRef}
        className={cn('flex flex-row h-full w-full bg-editor', className)}
      >
        {gutter}
        <pre
          ref={textAreaRef as unknown as React.RefObject<HTMLPreElement>}
          className={cn(codeClass, 'bg-editor flex-1')}
          style={{
            ...codeStyle,
            overflow: 'auto',
          }}
          onScroll={handleScroll}
        >
          {displayValue}
        </pre>
        {largeFileBanner}
      </div>
    );
  }

  const containerClassName = cn(
    'flex flex-row h-full w-full gap-1',
    'relative',
    className,
  );

  return (
    <div
      ref={containerRef}
      className={containerClassName}
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
        value={displayValue}
        onChange={handleChange}
        onScroll={handleScroll}
        onSelect={handleSelectionChange}
        spellCheck={false}
        autoComplete="off"
        autoCorrect="off"
        wrap="off"
      />
      {largeFileBanner}
    </div>
  );
}
