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
const MAX_VISIBLE_LINES = 200_000;

/** Fast line‑counting that stops scanning once we exceed MAX_VISIBLE_LINES.
 *  This avoids scanning gigabyte files completely. */
function fastLineCount(text: string): number | 'exceeds' {
  let lines = 1;
  for (let i = 0; i < text.length; i++) {
    if (text.charCodeAt(i) === 10) {
      lines++;
      if (lines > MAX_VISIBLE_LINES) {
        return 'exceeds';
      }
    }
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

function ReadOnlyContent({
  displayValue,
  cursorLine,
  lineHeight,
  scrollTop,
  displayLineCount,
  largeFileBanner,
  onScroll,
}: {
  displayValue: string;
  cursorLine: number;
  lineHeight: number;
  scrollTop: number;
  displayLineCount: number;
  largeFileBanner: React.ReactNode;
  onScroll: (st: number) => void;
}) {
  const scrollContainerRef = useRef<HTMLDivElement>(null);
  const [containerHeight, setContainerHeight] = useState(0);
  const rafRef = useRef<number | null>(null);

  useEffect(() => {
    const update = () => {
      if (scrollContainerRef.current) {
        setContainerHeight(scrollContainerRef.current.clientHeight);
      }
    };
    update();
    const observer = new ResizeObserver(update);
    if (scrollContainerRef.current) {
      observer.observe(scrollContainerRef.current);
    }
    return () => observer.disconnect();
  }, []);

  const gutterWidth = useMemo(() => {
    const digits = String(displayLineCount).length;
    return Math.max(
      GUTTER_CONFIG.MIN_WIDTH,
      digits * GUTTER_CONFIG.DIGIT_WIDTH +
        GUTTER_CONFIG.PADDING_LEFT +
        GUTTER_CONFIG.PADDING_RIGHT,
    );
  }, [displayLineCount]);

  const overscan = 5;
  const totalHeight = displayLineCount * lineHeight;

  const { firstLine, lastLine } = useMemo(() => {
    if (containerHeight === 0 || lineHeight <= 0) {
      return { firstLine: -1, lastLine: -1 };
    }
    const effectiveScrollTop = Math.max(0, scrollTop);
    const first = Math.max(0, Math.floor(effectiveScrollTop / lineHeight) - overscan);
    const last = Math.min(
      displayLineCount - 1,
      Math.ceil((effectiveScrollTop + containerHeight) / lineHeight) + overscan - 1,
    );
    if (!Number.isFinite(first) || !Number.isFinite(last)) {
      return { firstLine: -1, lastLine: -1 };
    }
    return { firstLine: first, lastLine: last };
  }, [scrollTop, lineHeight, displayLineCount, containerHeight]);

  const lineNumbers = useMemo(() => {
    if (firstLine < 0 || lastLine < 0) {
      return null;
    }
    const ops: React.ReactNode[] = [];
    for (let idx = firstLine; idx <= lastLine; idx++) {
      const lineNum = idx + 1;
      const isCurrent = lineNum === cursorLine;
      ops.push(
        <div
          key={idx}
          style={{
            position: 'absolute',
            top: idx * lineHeight,
            left: 0,
            right: 0,
            height: lineHeight,
            lineHeight: `${lineHeight}px`,
            paddingRight: GUTTER_CONFIG.PADDING_RIGHT,
            paddingLeft: GUTTER_CONFIG.PADDING_LEFT,
            pointerEvents: 'none',
          }}
          className={`text-right text-sm font-mono tabular-nums select-none ${
            isCurrent
              ? 'text-accent font-semibold bg-accent/15'
              : 'text-editor-foreground opacity-40'
          }`}
        >
          {lineNum}
        </div>,
      );
    }
    return ops;
  }, [firstLine, lastLine, cursorLine, lineHeight]);

  const handleScroll = useCallback(() => {
    if (rafRef.current != null) {
      cancelAnimationFrame(rafRef.current);
    }
    rafRef.current = requestAnimationFrame(() => {
      if (scrollContainerRef.current) {
        onScroll(scrollContainerRef.current.scrollTop);
      }
      rafRef.current = null;
    });
  }, [onScroll]);

  return (
    <div className="flex flex-col h-full w-full bg-editor overflow-hidden">
      <div
        ref={scrollContainerRef}
        className="overflow-auto relative flex-1"
        onScroll={handleScroll}
      >
        <div
          style={{
            position: 'relative',
            height: totalHeight,
            width: '100%',
            minWidth: '100%',
          }}
        >
          <div
            className="shrink-0 border-r border-[rgba(128,128,128,0.18)]"
            style={{
              position: 'absolute',
              left: 0,
              top: 0,
              width: gutterWidth,
              height: totalHeight,
              pointerEvents: 'none',
              overflow: 'hidden',
            }}
          >
            {lineNumbers}
          </div>
          <pre
            className="font-mono text-sm leading-[22px] p-0 hide-scrollbar text-editor-foreground bg-editor"
            style={{
              position: 'absolute',
              left: gutterWidth,
              top: 0,
              right: 0,
              height: totalHeight,
              margin: 0,
              border: 0,
              padding: 0,
              whiteSpace: 'pre',
              wordBreak: 'normal',
              overflow: 'visible',
            }}
          >
            {displayValue}
          </pre>
        </div>
      </div>
      {largeFileBanner}
    </div>
  );
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
  const [isLarge, setIsLarge] = useState<boolean>(() => {
    const r = fastLineCount(initialValue);
    return r === 'exceeds';
  });
  const [displayValue, setDisplayValue] = useState<string>(() => {
    if (isLarge) {
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

  // Editor state for the gutter (only cursor line, no scroll state)
  const [cursorLine, setCursorLine] = useState(() => {
    const beforeNewlines = initialValue.slice(0, 0).match(/\n/g);
    return beforeNewlines ? beforeNewlines.length + 1 : 1;
  });

  /** Scroll offset of the text content, used by the gutter to determine visible lines. */
  const [scrollTop, setScrollTop] = useState(0);

  // Update displayValue when initialValue changes from the outside
  useEffect(() => {
    if (initialRef.current !== initialValue) {
      initialRef.current = initialValue;
      fullValueRef.current = initialValue;
      const lineCountResult = fastLineCount(initialValue);
      if (lineCountResult === 'exceeds') {
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

  const rafScrollRef = useRef<number | null>(null);
  const handleScroll = useCallback(() => {
    if (rafScrollRef.current != null) {
      cancelAnimationFrame(rafScrollRef.current);
    }
    rafScrollRef.current = requestAnimationFrame(() => {
      const el = textAreaRef.current;
      if (!el) return;
      setScrollTop(el.scrollTop);
      rafScrollRef.current = null;
    });
  }, []);

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
  const displayLineCount = useMemo(() => {
    const r = fastLineCount(displayValue);
    return r === 'exceeds' ? MAX_VISIBLE_LINES + 1 : r; // fallback if something went wrong
  }, [displayValue]);

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
      scrollTop={scrollTop}
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
      <ReadOnlyContent
        displayValue={displayValue}
        cursorLine={cursorLine}
        lineHeight={lineHeight}
        scrollTop={scrollTop}
        displayLineCount={displayLineCount}
        largeFileBanner={largeFileBanner}
        onScroll={setScrollTop}
      />
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
