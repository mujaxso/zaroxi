import { useEffect, useRef, useState, useCallback, useMemo } from 'react';
import { cn } from '@/lib/utils';
import { useTabsStore } from '@/features/tabs/store';
import { LineNumberGutter } from './gutter/LineNumberGutter';
import { GUTTER_CONFIG } from './gutter/GutterConfig';
import { computeGutterWidth } from './gutter/GutterLayout';
import { FONT_TOKENS } from '@/lib/theme/font-tokens';
import { invoke } from '@tauri-apps/api/core';

interface CodeEditorProps {
  initialValue: string;
  onChange: (value: string) => void;
  filePath?: string;
  language?: string;
  readOnly?: boolean;
  className?: string;
}

/** Maximum number of lines we allow to be rendered at once.
 * Files exceeding this limit will show a preview of the first lines and forbid editing. */
const MAX_VISIBLE_LINES = 200_000;

/** Fast line‑counting that stops scanning once we exceed MAX_VISIBLE_LINES. */
function fastLineCount(text: string): number | 'exceeds' {
  let lines = 1;
  const len = text.length;
  let i = 0;
  while (i < len) {
    if (text.charCodeAt(i) === 10) {
      lines++;
      if (lines > MAX_VISIBLE_LINES) {
        return 'exceeds';
      }
    }
    i++;
  }
  return lines;
}

/** Return a substring that contains at most `maxLines` lines. */
function truncateToNLines(text: string, maxLines: number): string {
  let newlineCount = 0;
  const len = text.length;
  let i = 0;
  while (i < len) {
    if (text.charCodeAt(i) === 10) {
      newlineCount++;
      if (newlineCount >= maxLines) {
        return text.slice(0, i + 1);
      }
    }
    i++;
  }
  return text;
}

/** Return an array of character offsets where each line starts (first element is always 0). */
function computeLineOffsets(s: string): number[] {
  const offsets: number[] = [0];
  const len = s.length;
  let i = 0;
  while (i < len) {
    if (s.charCodeAt(i) === 10 /* \n */) {
      offsets.push(i + 1);
    }
    i++;
  }
  return offsets;
}

function VirtualEditor({
  displayValue,
  cursorLine,
  lineHeight,
  scrollTop,
  displayLineCount,
  largeFileBanner,
  onScroll,
  styledSpans,
  editable,
  onValueChange,
}: {
  displayValue: string;
  cursorLine: number;
  lineHeight: number;
  scrollTop: number;
  displayLineCount: number;
  largeFileBanner: React.ReactNode;
  onScroll: (st: number) => void;
  styledSpans: Array<{start: number; end: number; color: string}>;
  editable: boolean;
  onValueChange?: (newValue: string) => void;
}) {
  const textAreaRef = useRef<HTMLTextAreaElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [containerHeight, setContainerHeight] = useState(0);
  const rafRef = useRef<number | null>(null);

  useEffect(() => {
    const update = () => {
      if (containerRef.current) {
        setContainerHeight(containerRef.current.clientHeight);
      }
    };
    update();
    const observer = new ResizeObserver(update);
    if (containerRef.current) {
      observer.observe(containerRef.current);
    }
    return () => observer.disconnect();
  }, []);

  const lineOffsets = useMemo(() => computeLineOffsets(displayValue), [displayValue]);
  const sentinel = useMemo(
    () =>
      lineOffsets.length > 0
        ? [...lineOffsets, displayValue.length]
        : [0],
    [lineOffsets, displayValue.length],
  );
  const localLineCount = lineOffsets.length;

  const overscan = 5;
  const totalHeight = localLineCount * lineHeight;
  const gutterWidth = computeGutterWidth(displayLineCount);

  const { firstLine, lastLine } = useMemo(() => {
    // Use a default container height if not yet measured
    const effectiveContainerHeight = containerHeight > 0 ? containerHeight : 600;
    if (lineHeight <= 0) {
      return { firstLine: -1, lastLine: -1 };
    }
    const effectiveScrollTop = Math.max(0, scrollTop);
    const first = Math.max(0, Math.floor(effectiveScrollTop / lineHeight) - overscan);
    const last = Math.min(
      localLineCount - 1,
      Math.ceil((effectiveScrollTop + effectiveContainerHeight) / lineHeight) + overscan - 1,
    );
    if (!Number.isFinite(first) || !Number.isFinite(last)) {
      return { firstLine: -1, lastLine: -1 };
    }
    return { firstLine: first, lastLine: last };
  }, [scrollTop, lineHeight, localLineCount, containerHeight]);

  // Build a map from character offset to color for fast lookup
  const colorMap = useMemo(() => {
    const map = new Map<number, string>();
    for (const span of styledSpans) {
      for (let i = span.start; i < span.end; i++) {
        map.set(i, span.color);
      }
    }
    return map;
  }, [styledSpans]);

  const codeRows = useMemo(() => {
    if (firstLine < 0 || lastLine < 0) {
      return null;
    }
    const rows: React.ReactNode[] = [];
    for (let idx = firstLine; idx <= lastLine; idx++) {
      const start = sentinel[idx];
      const end = sentinel[idx + 1];
      const raw = displayValue.slice(start, end);
      const text = raw.replace(/\r?\n$/, '');

      const lineStart = start;
      const lineEnd = start + text.length; // character offset after stripping newline

      // Build segments for this line using the color map
      const segments: React.ReactNode[] = [];
      let currentPos = lineStart;
      while (currentPos < lineEnd) {
        const color = colorMap.get(currentPos);
        // Find the end of this color run
        let runEnd = currentPos + 1;
        while (runEnd < lineEnd && colorMap.get(runEnd) === color) {
          runEnd++;
        }
        const segmentText = text.slice(currentPos - lineStart, runEnd - lineStart);
        if (segmentText.length > 0) {
          segments.push(
            <span
              key={`${currentPos}-${runEnd}`}
              style={{ color: color ?? undefined }}
            >
              {segmentText}
            </span>
          );
        }
        currentPos = runEnd;
      }

      rows.push(
        <div
          key={idx}
          style={{
            position: 'absolute',
            left: 0,
            top: idx * lineHeight,
            right: 0,
            height: lineHeight,
            lineHeight: `${lineHeight}px`,
            whiteSpace: 'pre',
            overflow: 'hidden',
            fontFamily: FONT_TOKENS.editor,
            fontSize: 'inherit',
            pointerEvents: 'none',
          }}
          className="text-sm p-0 text-editor-foreground"
        >
          {segments.length > 0 ? segments : text}
        </div>
      );
    }
    return rows;
  }, [firstLine, lastLine, lineHeight, displayValue, sentinel, colorMap]);

  const handleScroll = useCallback(() => {
    if (rafRef.current != null) {
      cancelAnimationFrame(rafRef.current);
    }
    rafRef.current = requestAnimationFrame(() => {
      if (textAreaRef.current) {
        onScroll(textAreaRef.current.scrollTop);
      }
      rafRef.current = null;
    });
  }, [onScroll]);

  const handleChange = useCallback(
    (e: React.ChangeEvent<HTMLTextAreaElement>) => {
      if (!editable || !onValueChange) return;
      onValueChange(e.target.value);
    },
    [editable, onValueChange],
  );

  const handleSelectionChange = useCallback(() => {
    const ta = textAreaRef.current;
    if (!ta) return;
    const selStart = ta.selectionStart;
    const beforeNewlines = displayValue.slice(0, selStart).match(/\n/g);
    const line = beforeNewlines ? beforeNewlines.length + 1 : 1;
    // cursorLine is passed from parent, but we don't update it here for now
  }, [displayValue]);

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
    fontFamily: FONT_TOKENS.editor,
    fontSize: 'inherit',
    lineHeight: `${lineHeight}px`,
    resize: 'none',
    outline: 'none',
    background: 'transparent',
    caretColor: 'inherit',
    color: 'transparent',
  };

  return (
    <div className="flex flex-col h-full w-full bg-editor overflow-hidden">
      <div
        ref={containerRef}
        className="relative flex-1 overflow-hidden"
      >
        {/* Gutter */}
        <div
          style={{
            position: 'absolute',
            left: 0,
            top: 0,
            width: gutterWidth,
            height: '100%',
            pointerEvents: 'none',
            overflow: 'hidden',
            zIndex: 2,
          }}
        >
          <LineNumberGutter
            lineCount={displayLineCount}
            cursorLine={cursorLine}
            lineHeight={lineHeight}
            scrollTop={scrollTop}
            containerHeight={containerHeight}
          />
        </div>
        {/* Styled overlay – this is the single source of visible text */}
        <div
          style={{
            position: 'absolute',
            left: gutterWidth,
            top: 0,
            right: 0,
            height: totalHeight,
            pointerEvents: 'none',
            overflow: 'hidden',
            fontFamily: FONT_TOKENS.editor,
            fontSize: 'inherit',
            lineHeight: `${lineHeight}px`,
            whiteSpace: 'pre',
            zIndex: 1,
          }}
          className="text-sm p-0 text-editor-foreground"
        >
          {codeRows}
        </div>
        {/* Textarea for editing – transparent, only captures input */}
        <textarea
          ref={textAreaRef}
          style={{
            ...codeStyle,
            position: 'absolute',
            left: gutterWidth,
            top: 0,
            right: 0,
            bottom: 0,
            zIndex: 0,
          }}
          className="text-sm p-0"
          value={displayValue}
          onChange={handleChange}
          onScroll={handleScroll}
          onSelect={handleSelectionChange}
          spellCheck={false}
          autoComplete="off"
          autoCorrect="off"
          wrap="off"
          readOnly={!editable}
        />
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
  const [isLarge, setIsLarge] = useState<boolean>(() => {
    if (initialValue.length > 10_000_000) {
      return true;
    }
    const r = fastLineCount(initialValue);
    return r === 'exceeds';
  });
  const [displayValue, setDisplayValue] = useState<string>(() => {
    if (isLarge) {
      return truncateToNLines(initialValue, MAX_VISIBLE_LINES);
    }
    return initialValue;
  });

  const fullValueRef = useRef(initialValue);
  const initialRef = useRef(initialValue);

  const [cursorLine, setCursorLine] = useState(() => {
    const beforeNewlines = initialValue.slice(0, 0).match(/\n/g);
    return beforeNewlines ? beforeNewlines.length + 1 : 1;
  });

  const [scrollTop, setScrollTop] = useState(0);
  const [styledSpans, setStyledSpans] = useState<Array<{start: number; end: number; color: string}>>([]);
  const [highlightVersion, setHighlightVersion] = useState(0);

  // Ref to store the last valid styled spans to avoid flash when fetch is in progress
  const lastValidSpansRef = useRef<Array<{start: number; end: number; color: string}>>([]);
  // Ref to store the last fetched line range to avoid redundant fetches
  const lastFetchedRangeRef = useRef<{firstLine: number; lastLine: number} | null>(null);
  // Ref for debounce timer
  const debounceTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  // Re-fetch highlights when scroll position changes (debounced)
  const scrollRef = useRef(scrollTop);
  scrollRef.current = scrollTop;

  useEffect(() => {
    if (!filePath) return;

    // Compute visible line range for the current scroll position
    const lineHeight = GUTTER_CONFIG.LINE_HEIGHT;
    const containerHeight = 600; // approximate; will be refined
    const overscan = 5;
    const effectiveScrollTop = Math.max(0, scrollTop);
    const firstLine = Math.max(0, Math.floor(effectiveScrollTop / lineHeight) - overscan);
    const lastLine = Math.ceil((effectiveScrollTop + containerHeight) / lineHeight) + overscan - 1;

    // Skip fetch if the range hasn't changed significantly (within 2 lines)
    const lastRange = lastFetchedRangeRef.current;
    if (lastRange && Math.abs(lastRange.firstLine - firstLine) <= 2 && Math.abs(lastRange.lastLine - lastLine) <= 2) {
      return;
    }

    // Clear previous debounce timer
    if (debounceTimerRef.current) {
      clearTimeout(debounceTimerRef.current);
    }

    // Debounce the fetch by 50ms to avoid excessive requests during scrolling
    debounceTimerRef.current = setTimeout(() => {
      console.log('[CodeEditor] Fetching styled spans for:', filePath, 'lines', firstLine, '-', lastLine);
      invoke('get_styled_spans', {
        path: filePath,
        startLine: firstLine,
        endLine: lastLine,
      })
        .then((spans: any) => {
          console.log('[CodeEditor] Received styled spans:', spans?.length);
          const newSpans = spans || [];
          setStyledSpans(newSpans);
          lastValidSpansRef.current = newSpans;
          lastFetchedRangeRef.current = { firstLine, lastLine };
          // Do NOT update highlightVersion here to avoid infinite loop
        })
        .catch((err: any) => {
          console.error('[CodeEditor] Failed to get styled spans:', err);
          // Fallback: keep last valid spans instead of clearing to avoid flash
          // Only clear if we have no valid spans at all
          if (lastValidSpansRef.current.length === 0) {
            setStyledSpans([]);
          }
        });
    }, 50);

    return () => {
      if (debounceTimerRef.current) {
        clearTimeout(debounceTimerRef.current);
      }
    };
  }, [filePath, scrollTop]);

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
      // Invalidate highlights when content changes
      setHighlightVersion(v => v + 1);
    }
  }, [initialValue]);

  useEffect(() => {
    if (document.getElementById('hide-scrollbar-style')) {
      return;
    }
    const style = document.createElement('style');
    style.id = 'hide-scrollbar-style';
    style.innerHTML = `
      .hide-scrollbar::-webkit-scrollbar {
        display: none;
      }
    `;
    document.head.appendChild(style);
    return () => {};
  }, []);

  const lineHeight = GUTTER_CONFIG.LINE_HEIGHT;

  const handleValueChange = useCallback(
    (newValue: string) => {
      fullValueRef.current = newValue;
      setDisplayValue(newValue);
      onChange(newValue);
      if (filePath) {
        useTabsStore.getState().markDirty(filePath);
      }
      // Invalidate highlights when content changes
      setHighlightVersion(v => v + 1);
    },
    [onChange, filePath],
  );

  const displayLineCount = useMemo(() => {
    const r = fastLineCount(displayValue);
    return r === 'exceeds' ? MAX_VISIBLE_LINES + 1 : r;
  }, [displayValue]);

  const largeFileBanner = isLarge ? (
    <div className="absolute bottom-0 left-0 right-0 bg-editor z-10 p-2 text-xs text-muted-foreground border-t border-[rgba(128,128,128,0.18)]">
      File is larger than {MAX_VISIBLE_LINES.toLocaleString()} lines.
      Showing a read‑only preview of the first {MAX_VISIBLE_LINES.toLocaleString()} lines.
    </div>
  ) : null;

  const effectiveReadOnly = readOnly || isLarge;

  return (
    <VirtualEditor
      displayValue={displayValue}
      cursorLine={cursorLine}
      lineHeight={lineHeight}
      scrollTop={scrollTop}
      displayLineCount={displayLineCount}
      largeFileBanner={largeFileBanner}
      onScroll={setScrollTop}
      styledSpans={styledSpans}
      editable={!effectiveReadOnly}
      onValueChange={effectiveReadOnly ? undefined : handleValueChange}
    />
  );
}
