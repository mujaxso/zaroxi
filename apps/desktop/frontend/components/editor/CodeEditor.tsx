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
    if (containerHeight === 0 || lineHeight <= 0) {
      return { firstLine: -1, lastLine: -1 };
    }
    const effectiveScrollTop = Math.max(0, scrollTop);
    const first = Math.max(0, Math.floor(effectiveScrollTop / lineHeight) - overscan);
    const last = Math.min(
      localLineCount - 1,
      Math.ceil((effectiveScrollTop + containerHeight) / lineHeight) + overscan - 1,
    );
    if (!Number.isFinite(first) || !Number.isFinite(last)) {
      return { firstLine: -1, lastLine: -1 };
    }
    return { firstLine: first, lastLine: last };
  }, [scrollTop, lineHeight, localLineCount, containerHeight]);

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
      const lineEnd = end;
      const lineSpans = styledSpans.filter(
        (s) => s.start >= lineStart && s.end <= lineEnd
      );

      const segments: React.ReactNode[] = [];
      let currentPos = lineStart;
      for (const span of lineSpans) {
        if (span.start > currentPos) {
          segments.push(
            <span key={`${currentPos}-plain`}>
              {text.slice(currentPos - lineStart, span.start - lineStart)}
            </span>
          );
        }
        segments.push(
          <span
            key={`${span.start}-styled`}
            style={{ color: span.color }}
          >
            {text.slice(span.start - lineStart, span.end - lineStart)}
          </span>
        );
        currentPos = span.end;
      }
      if (currentPos < lineEnd) {
        segments.push(
          <span key={`${currentPos}-plain-end`}>
            {text.slice(currentPos - lineStart)}
          </span>
        );
      }

      rows.push(
        <div
          key={idx}
          style={{
            position: 'absolute',
            left: gutterWidth,
            top: idx * lineHeight,
            right: 0,
            height: lineHeight,
            lineHeight: `${lineHeight}px`,
            whiteSpace: 'pre',
            overflow: 'hidden',
            fontFamily: FONT_TOKENS.editor,
            fontSize: 'inherit',
          }}
          className="text-sm p-0 text-editor-foreground"
        >
          {segments.length > 0 ? segments : text}
        </div>,
      );
    }
    return rows;
  }, [firstLine, lastLine, lineHeight, displayValue, sentinel, styledSpans]);

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

  // Handle input events for editable mode
  const handleInput = useCallback(
    (e: React.FormEvent<HTMLDivElement>) => {
      if (!editable || !onValueChange) return;
      const newText = (e.target as HTMLElement).innerText;
      onValueChange(newText);
    },
    [editable, onValueChange],
  );

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
          {/* Gutter */}
          <div
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
            <LineNumberGutter
              lineCount={displayLineCount}
              cursorLine={cursorLine}
              lineHeight={lineHeight}
              scrollTop={scrollTop}
              containerHeight={containerHeight}
            />
          </div>
          {/* Virtualised code rows */}
          <div
            contentEditable={editable}
            suppressContentEditableWarning
            onInput={handleInput}
            style={{
              position: 'absolute',
              left: gutterWidth,
              top: 0,
              right: 0,
              height: totalHeight,
              outline: 'none',
              whiteSpace: 'pre',
              overflow: 'hidden',
              fontFamily: FONT_TOKENS.editor,
              fontSize: 'inherit',
              lineHeight: `${lineHeight}px`,
            }}
            className="text-sm p-0 text-editor-foreground"
          >
            {codeRows}
          </div>
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

  const [styledSpans, setStyledSpans] = useState<Array<{start: number; end: number; color: string}>>([]);

  useEffect(() => {
    if (!filePath) return;
    console.log('[CodeEditor] Fetching styled spans for:', filePath);
    invoke('get_styled_spans', { path: filePath })
      .then((spans: any) => {
        console.log('[CodeEditor] Received styled spans:', spans);
        setStyledSpans(spans);
      })
      .catch((err: any) => {
        console.error('[CodeEditor] Failed to get styled spans:', err);
      });
  }, [filePath]);

  const lineHeight = GUTTER_CONFIG.LINE_HEIGHT;

  const handleValueChange = useCallback(
    (newValue: string) => {
      fullValueRef.current = newValue;
      setDisplayValue(newValue);
      onChange(newValue);
      if (filePath) {
        useTabsStore.getState().markDirty(filePath);
      }
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
