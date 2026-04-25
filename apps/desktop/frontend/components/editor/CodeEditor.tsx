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

// Cache for styled spans keyed by (filePath, version, firstLine, lastLine)
// This cache persists across renders and scrolls, preventing re-fetching
const styledSpansCache = new Map<string, {
  spans: Array<{start: number; end: number; color: string}>;
  version: number;
  firstLine: number;
  lastLine: number;
}>();

function getCachedSpansKey(filePath: string, version: number, firstLine: number, lastLine: number): string {
  return `${filePath}:${version}:${firstLine}:${lastLine}`;
}

// Global stable highlight state that persists across scrolls
// Keyed by filePath, stores the most recent valid spans for each line range
const stableHighlightState = new Map<string, {
  version: number;
  // Map from line range key to spans
  rangeSpans: Map<string, Array<{start: number; end: number; color: string}>>;
}>();

// Helper to merge two arrays of spans, keeping the most recent for overlapping ranges
function mergeSpanArrays(
  existing: Array<{start: number; end: number; color: string}>,
  incoming: Array<{start: number; end: number; color: string}>
): Array<{start: number; end: number; color: string}> {
  // Build a map from start position to span for quick lookup
  const spanMap = new Map<number, {start: number; end: number; color: string}>();
  
  // Add existing spans
  for (const span of existing) {
    spanMap.set(span.start, span);
  }
  
  // Override with incoming spans (they are more recent)
  for (const span of incoming) {
    spanMap.set(span.start, span);
  }
  
  // Convert back to array and sort by start position
  return Array.from(spanMap.values()).sort((a, b) => a.start - b.start);
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
  containerHeightRef,
  filePath,
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
  containerHeightRef: React.MutableRefObject<number>;
  filePath?: string;
}) {
  const textAreaRef = useRef<HTMLTextAreaElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [containerHeight, setContainerHeight] = useState(600);

  useEffect(() => {
    const update = () => {
      if (containerRef.current) {
        const h = containerRef.current.clientHeight;
        if (h > 0 && h !== containerHeightRef.current) {
          containerHeightRef.current = h;
          setContainerHeight(h);
        }
      }
    };
    update();
    const observer = new ResizeObserver(update);
    if (containerRef.current) {
      observer.observe(containerRef.current);
    }
    return () => observer.disconnect();
  }, [containerHeightRef]);

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

  // Compute visible range deterministically from scrollTop, containerHeight, lineHeight
  const { firstLine, lastLine } = useMemo(() => {
    const effectiveContainerHeight = containerHeight > 0 ? containerHeight : 600;
    if (lineHeight <= 0 || localLineCount === 0) {
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
  // This uses ALL available spans, not just the current viewport range
  const colorMap = useMemo(() => {
    const map = new Map<number, string>();
    // Use the global stable state if available for this file
    if (filePath) {
      const stableState = stableHighlightState.get(filePath);
      if (stableState) {
        for (const [, spans] of stableState.rangeSpans) {
          for (const span of spans) {
            for (let i = span.start; i < span.end; i++) {
              map.set(i, span.color);
            }
          }
        }
      }
    }
    // Also include current styledSpans (may overlap but that's fine)
    for (const span of styledSpans) {
      for (let i = span.start; i < span.end; i++) {
        map.set(i, span.color);
      }
    }
    return map;
  }, [styledSpans, filePath]);

  // Render rows for the computed visible range
  const codeRows = useMemo(() => {
    if (firstLine < 0 || lastLine < 0 || localLineCount === 0) {
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
  }, [firstLine, lastLine, lineHeight, displayValue, sentinel, colorMap, localLineCount]);

  // Synchronous scroll handler – no requestAnimationFrame delay
  const handleScroll = useCallback(() => {
    if (textAreaRef.current) {
      const ta = textAreaRef.current;
      onScroll(ta.scrollTop);
      // Sync horizontal scroll to the parent container
      const parent = ta.parentElement;
      if (parent && parent.scrollLeft !== ta.scrollLeft) {
        parent.scrollLeft = ta.scrollLeft;
      }
    }
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
    width: 'auto',
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
    caretColor: '#E6EAF2', // visible caret color (dark theme primary text)
    color: 'transparent',
  };

  return (
    <div className="flex flex-col h-full w-full bg-editor overflow-hidden">
      <div
        ref={containerRef}
        className="relative flex-1 overflow-hidden"
      >
        {/* Fixed gutter – does not scroll */}
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
        {/* Scrollable area – starts after the gutter */}
        <div
          style={{
            position: 'absolute',
            left: gutterWidth,
            top: 0,
            right: 0,
            bottom: 0,
            overflow: 'auto',
            scrollbarWidth: 'none',
            msOverflowStyle: 'none',
          }}
          className="hide-scrollbar"
          onScroll={(e) => {
            const target = e.target as HTMLDivElement;
            // Sync scrollTop from this container to the textarea
            const st = target.scrollTop;
            if (textAreaRef.current && textAreaRef.current.scrollTop !== st) {
              textAreaRef.current.scrollTop = st;
            }
            // Sync scrollLeft from this container to the textarea
            const sl = target.scrollLeft;
            if (textAreaRef.current && textAreaRef.current.scrollLeft !== sl) {
              textAreaRef.current.scrollLeft = sl;
            }
            onScroll(st);
          }}
        >
          {/* Styled overlay – positioned relative to this scrollable container */}
          <div
            style={{
              position: 'relative',
              left: 0,
              top: 0,
              height: totalHeight,
              pointerEvents: 'none',
              overflow: 'visible',
              fontFamily: FONT_TOKENS.editor,
              fontSize: 'inherit',
              lineHeight: `${lineHeight}px`,
              whiteSpace: 'pre',
              zIndex: 1,
              width: 'auto',
              minWidth: '100%',
            }}
            className="text-xs p-0 text-editor-foreground"
          >
            {codeRows}
          </div>
          {/* Textarea for editing – transparent, only captures input */}
          <textarea
            ref={textAreaRef}
            style={{
              ...codeStyle,
              position: 'absolute',
              left: 0,
              top: 0,
              height: totalHeight,
              zIndex: 0,
              width: 'auto',
              minWidth: '100%',
            }}
            className="text-xs p-0"
            value={displayValue}
            onChange={handleChange}
            onScroll={handleScroll}
            onSelect={handleSelectionChange}
            onClick={() => {
              // Ensure textarea receives focus on click
              if (textAreaRef.current && editable) {
                textAreaRef.current.focus();
              }
            }}
            spellCheck={false}
            autoComplete="off"
            autoCorrect="off"
            wrap="off"
            readOnly={!editable}
          />
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
    if (initialValue.length > 10_000_000) { return true; }
    const r = fastLineCount(initialValue);
    return r === 'exceeds';
  });
  const [displayValue, setDisplayValue] = useState<string>(() => {
    if (isLarge) { return truncateToNLines(initialValue, MAX_VISIBLE_LINES); }
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

  const lastValidSpansRef = useRef<Array<{start: number; end: number; color: string}>>([]);
  const styledSpansRef = useRef<Array<{start: number; end: number; color: string}>>([]);
  const allSpansRef = useRef<Array<{start: number; end: number; color: string}>>([]);
  const lastFetchedRangeRef = useRef<{firstLine: number; lastLine: number} | null>(null);
  const containerHeightRef = useRef(600);
  const rafRef = useRef<number | null>(null);
  const abortControllerRef = useRef<AbortController | null>(null);
  const versionRef = useRef<number>(0);
  const lastFetchTimeRef = useRef<number>(0);

  const scrollRef = useRef(scrollTop);
  scrollRef.current = scrollTop;

  const fetchStyledSpans = useCallback(async (
    filePath: string,
    firstLine: number,
    lastLine: number,
    version: number,
  ) => {
    // Check the global stable state first - this persists across scrolls
    const stableState = stableHighlightState.get(filePath);
    if (stableState && stableState.version === version) {
      const rangeKey = `${firstLine}:${lastLine}`;
      const cachedRange = stableState.rangeSpans.get(rangeKey);
      if (cachedRange) {
        // Use stable cached spans without any visual change
        // Merge with existing allSpansRef to ensure coverage
        const mergedSpans = mergeSpanArrays(allSpansRef.current, cachedRange);
        allSpansRef.current = mergedSpans;
        if (JSON.stringify(styledSpansRef.current) !== JSON.stringify(cachedRange)) {
          setStyledSpans(cachedRange);
          styledSpansRef.current = cachedRange;
        }
        lastValidSpansRef.current = cachedRange;
        lastFetchedRangeRef.current = { firstLine, lastLine };
        return;
      }
    }

    const cacheKey = getCachedSpansKey(filePath, version, firstLine, lastLine);
    const cached = styledSpansCache.get(cacheKey);
    if (cached) {
      // Update stable state
      if (!stableHighlightState.has(filePath)) {
        stableHighlightState.set(filePath, { version, rangeSpans: new Map() });
      }
      const state = stableHighlightState.get(filePath)!;
      state.rangeSpans.set(`${firstLine}:${lastLine}`, cached.spans);

      // Merge with allSpansRef
      const mergedSpans = mergeSpanArrays(allSpansRef.current, cached.spans);
      allSpansRef.current = mergedSpans;

      // Only update if spans actually changed
      if (JSON.stringify(styledSpansRef.current) !== JSON.stringify(cached.spans)) {
        setStyledSpans(cached.spans);
        styledSpansRef.current = cached.spans;
      }
      lastValidSpansRef.current = cached.spans;
      lastFetchedRangeRef.current = { firstLine, lastLine };
      return;
    }

    if (abortControllerRef.current) { abortControllerRef.current.abort(); }
    const controller = new AbortController();
    abortControllerRef.current = controller;

    try {
      const spans: any = await invoke('get_styled_spans', {
        path: filePath,
        startLine: firstLine,
        endLine: lastLine,
        version: version,
      });

      if (controller.signal.aborted) { return; }

      const newSpans = spans || [];
      styledSpansCache.set(cacheKey, { spans: newSpans, version, firstLine, lastLine });
      if (styledSpansCache.size > 200) {
        const firstKey = styledSpansCache.keys().next().value;
        if (firstKey) { styledSpansCache.delete(firstKey); }
      }

      // Update stable state
      if (!stableHighlightState.has(filePath)) {
        stableHighlightState.set(filePath, { version, rangeSpans: new Map() });
      }
      const state = stableHighlightState.get(filePath)!;
      state.rangeSpans.set(`${firstLine}:${lastLine}`, newSpans);

      // Merge with allSpansRef
      const mergedSpans = mergeSpanArrays(allSpansRef.current, newSpans);
      allSpansRef.current = mergedSpans;

      // Only update state if the spans actually changed (avoid flash)
      const currentJson = JSON.stringify(styledSpansRef.current);
      const newJson = JSON.stringify(newSpans);
      if (currentJson !== newJson) {
        setStyledSpans(newSpans);
        styledSpansRef.current = newSpans;
      }
      lastValidSpansRef.current = newSpans;
      lastFetchedRangeRef.current = { firstLine, lastLine };
    } catch (err: any) {
      if (err?.message?.includes('abort') || err?.name === 'AbortError') { return; }
      console.error('[CodeEditor] Failed to get styled spans:', err);
      // Keep last valid spans instead of clearing
      if (lastValidSpansRef.current.length > 0) {
        setStyledSpans(lastValidSpansRef.current);
        styledSpansRef.current = lastValidSpansRef.current;
      }
    }
  }, []);

  // Fetch the actual document version from the backend on mount
  useEffect(() => {
    if (!filePath) return;
    // Reset allSpansRef when file changes
    allSpansRef.current = [];
    // Don't clear stable state - keep it for immediate reuse on tab switch
    // Only clear if we're opening a completely different file (not tab switch)
    // We'll let the version check handle invalidation
    (async () => {
      try {
        const response: any = await invoke('open_document', { path: filePath });
        versionRef.current = response.version;
        // After getting the new version, check if stable state is still valid
        const stableState = stableHighlightState.get(filePath);
        if (stableState && stableState.version !== response.version) {
          // Version changed, clear stale state
          stableHighlightState.delete(filePath);
        }
      } catch (err) {
        console.error('[CodeEditor] Failed to get document version:', err);
      }
    })();
  }, [filePath]);

  useEffect(() => {
    if (!filePath) return;
    if (rafRef.current) { cancelAnimationFrame(rafRef.current); }

    rafRef.current = requestAnimationFrame(() => {
      const now = Date.now();
      const THROTTLE_MS = 50; // reduced throttle for smoother updates
      if (now - lastFetchTimeRef.current < THROTTLE_MS) {
        return;
      }

      const lineHeight = GUTTER_CONFIG.LINE_HEIGHT;
      const containerHeight = containerHeightRef.current;
      const overscan = 10; // increased overscan to pre-fetch more lines
      const effectiveScrollTop = Math.max(0, scrollTop);
      const firstLine = Math.max(0, Math.floor(effectiveScrollTop / lineHeight) - overscan);
      const lastLine = Math.ceil((effectiveScrollTop + containerHeight) / lineHeight) + overscan - 1;

      // Check if we already have stable state for this range
      const stableState = stableHighlightState.get(filePath);
      if (stableState && stableState.version === versionRef.current) {
        const rangeKey = `${firstLine}:${lastLine}`;
        if (stableState.rangeSpans.has(rangeKey)) {
          // Already have this range cached, no need to fetch
          return;
        }
      }

      // Skip fetch if the visible range hasn't changed significantly
      const lastRange = lastFetchedRangeRef.current;
      if (lastRange && Math.abs(lastRange.firstLine - firstLine) <= 2 && Math.abs(lastRange.lastLine - lastLine) <= 2) {
        return;
      }

      lastFetchTimeRef.current = now;
      fetchStyledSpans(filePath, firstLine, lastLine, versionRef.current);
    });

    return () => { if (rafRef.current) { cancelAnimationFrame(rafRef.current); } };
  }, [filePath, scrollTop, fetchStyledSpans]);

  useEffect(() => {
    if (!filePath) return;
    const lineHeight = GUTTER_CONFIG.LINE_HEIGHT;
    const containerHeight = containerHeightRef.current;
    const overscan = 10;
    const firstLine = 0;
    const lastLine = Math.ceil(containerHeight / lineHeight) + overscan - 1;
    
    // Check stable state first - this persists across tab switches
    const stableState = stableHighlightState.get(filePath);
    if (stableState && stableState.version === versionRef.current) {
      const rangeKey = `${firstLine}:${lastLine}`;
      if (stableState.rangeSpans.has(rangeKey)) {
        // Already have this range cached - use it immediately
        const cachedSpans = stableState.rangeSpans.get(rangeKey)!;
        if (JSON.stringify(styledSpansRef.current) !== JSON.stringify(cachedSpans)) {
          setStyledSpans(cachedSpans);
          styledSpansRef.current = cachedSpans;
        }
        lastValidSpansRef.current = cachedSpans;
        lastFetchedRangeRef.current = { firstLine, lastLine };
        return;
      }
    }
    
    // If we have any stable state for this file, use the first available range
    if (stableState && stableState.version === versionRef.current) {
      const firstRange = stableState.rangeSpans.values().next().value;
      if (firstRange) {
        if (JSON.stringify(styledSpansRef.current) !== JSON.stringify(firstRange)) {
          setStyledSpans(firstRange);
          styledSpansRef.current = firstRange;
        }
        lastValidSpansRef.current = firstRange;
        lastFetchedRangeRef.current = { firstLine, lastLine };
        return;
      }
    }
    
    fetchStyledSpans(filePath, firstLine, lastLine, versionRef.current);
  }, [filePath, fetchStyledSpans]);

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
      setHighlightVersion(v => v + 1);
    }
  }, [initialValue]);

  useEffect(() => {
    if (document.getElementById('hide-scrollbar-style')) { return; }
    const style = document.createElement('style');
    style.id = 'hide-scrollbar-style';
    style.innerHTML = `.hide-scrollbar::-webkit-scrollbar { display: none; }`;
    document.head.appendChild(style);
    return () => {};
  }, []);

  const lineHeight = GUTTER_CONFIG.LINE_HEIGHT;

  const handleValueChange = useCallback(
    (newValue: string) => {
      fullValueRef.current = newValue;
      setDisplayValue(newValue);
      onChange(newValue);
      if (filePath) { useTabsStore.getState().markDirty(filePath); }
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
      containerHeightRef={containerHeightRef}
      filePath={filePath}
    />
  );
}
