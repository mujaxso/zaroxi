import { useEffect, useRef, useState, useCallback, useMemo } from 'react';
import { cn } from '@/lib/utils';
import { useTabsStore } from '@/features/tabs/store';
import { LineNumberGutter } from './gutter/LineNumberGutter';
import { GUTTER_CONFIG } from './gutter/GutterConfig';
import { computeGutterWidth } from './gutter/GutterLayout';
import { FONT_TOKENS } from '@/lib/theme/font-tokens';
import { invoke } from '@tauri-apps/api/core';

/* ------------------------------------------------------------------ */
/*  Custom hook: fetch full‑file syntax highlights once per file       */
/* ------------------------------------------------------------------ */
interface HighlightSpan {
  start: number;
  end: number;
  token_type: string;
  color?: string;
}
interface HighlightLine {
  index: number;
  text: string;
  spans: HighlightSpan[];
}
interface HighlightResponse {
  lines: HighlightLine[];
}

const FULL_LINES_LIMIT = 100_000;

function useFullHighlight(
  documentId: string | null,
  enabled: boolean,
  theme?: 'dark' | 'light',
) {
  const [lines, setLines] = useState<HighlightLine[]>([]);

  useEffect(() => {
    if (!documentId || !enabled) {
      setLines([]);
      return;
    }

    let cancelled = false;

    async function fetch() {
      try {
        const res: HighlightResponse = await invoke('highlight_document', {
          request: {
            documentId,
            startLine: 0,
            count: FULL_LINES_LIMIT,
            theme: theme ?? 'dark',
          },
        });
        if (!cancelled) {
          setLines(res.lines);
        }
      } catch (err) {
        console.warn('full highlight failed:', err);
        if (!cancelled) setLines([]);
      }
    }

    fetch();
    return () => {
      cancelled = true;
    };
  }, [documentId, enabled, theme]);

  return lines;
}

/**
 * Merge overlapping / nested highlight spans into a non‑overlapping
 * sequence.  The innermost (shortest) span wins when two spans
 * compete for the same character.
 */
function mergeSpans(spans: HighlightSpan[], lineLen: number): HighlightSpan[] {
  if (spans.length === 0 || lineLen === 0) return [];

  const sorted = [...spans].sort((a, b) => (a.end - a.start) - (b.end - b.start));

  const charTokens: Array<{ tokenType: string; color?: string } | null> =
    new Array(lineLen).fill(null);

  for (const sp of sorted) {
    const tok = sp.token_type;
    const color = sp.color;
    const from = Math.max(0, sp.start);
    const to = Math.min(lineLen, sp.end);
    for (let i = from; i < to; i++) {
      if (charTokens[i] === null) {
        charTokens[i] = { tokenType: tok, color };
      }
    }
  }

  const merged: HighlightSpan[] = [];
  let i = 0;
  while (i < lineLen) {
    const cur = charTokens[i];
    if (cur) {
      let j = i;
      while (j < lineLen && charTokens[j] && charTokens[j]!.tokenType === cur.tokenType) {
        j++;
      }
      merged.push({
        start: i,
        end: j,
        token_type: cur.tokenType,
        color: cur.color,
      });
      i = j;
    } else {
      i++;
    }
  }
  return merged;
}

function renderSpans(spans: HighlightSpan[], lineText: string) {
  if (spans.length === 0) {
    return lineText;
  }

  const merged = mergeSpans(spans, lineText.length);
  if (merged.length === 0) {
    return lineText;
  }

  const segments: React.ReactNode[] = [];
  let last = 0;
  for (let i = 0; i < merged.length; i++) {
    const sp = merged[i];
    if (sp.start > last) {
      segments.push(lineText.slice(last, sp.start));
    }
    const key = `${sp.start}-${i}`;
    segments.push(
      <span
        key={key}
        style={sp.color ? { color: sp.color } : undefined}
      >
        {lineText.slice(sp.start, sp.end)}
      </span>,
    );
    last = sp.end;
  }
  if (last < lineText.length) {
    segments.push(lineText.slice(last));
  }
  return segments;
}

/* ------------------------------------------------------------------ */
/*  CodeEditor component                                              */
/* ------------------------------------------------------------------ */

interface CodeEditorProps {
  initialValue: string;
  onChange: (value: string) => void;
  filePath?: string;
  language?: string;
  readOnly?: boolean;
  className?: string;
  contentTruncated?: boolean;
  theme?: 'dark' | 'light';
}

const TRUNCATE_CHARS = 50_000;

function computeLineStarts(text: string): number[] {
  const starts: number[] = [0];
  let pos = 0;
  while (pos < text.length) {
    const next = text.indexOf('\n', pos);
    if (next === -1) break;
    starts.push(next + 1);
    pos = next + 1;
  }
  return starts;
}

export function CodeEditor({
  initialValue,
  onChange,
  filePath,
  readOnly = false,
  className,
  contentTruncated,
  theme = 'dark',
}: CodeEditorProps) {
  const [value, setValue] = useState(initialValue);
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const highlightLayerRef = useRef<HTMLDivElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [scrollTop, setScrollTop] = useState(0);
  const [scrollLeft, setScrollLeft] = useState(0);
  const [cursorLine, setCursorLine] = useState(1);

  const largeFile = contentTruncated ?? (initialValue.length >= TRUNCATE_CHARS);

  useEffect(() => {
    setValue(initialValue);
  }, [initialValue]);

  const handleTextareaScroll = useCallback(() => {
    const ta = textareaRef.current;
    if (!ta) return;
    setScrollTop(ta.scrollTop);
    setScrollLeft(ta.scrollLeft);
  }, []);

  const handleSelectionChange = useCallback(() => {
    const ta = textareaRef.current;
    if (!ta) return;
    const sel = ta.selectionStart;
    const before = value.slice(0, sel).match(/\n/g);
    const line = before ? before.length + 1 : 1;
    setCursorLine(line);
  }, [value]);

  const handleChange = useCallback(
    (e: React.ChangeEvent<HTMLTextAreaElement>) => {
      if (readOnly) return;
      const newVal = e.target.value;
      setValue(newVal);
      onChange(newVal);
      if (filePath) {
        useTabsStore.getState().markDirty(filePath);
      }
      const selStart = e.target.selectionStart;
      const before = newVal.slice(0, selStart).match(/\n/g);
      const line = before ? before.length + 1 : 1;
      setCursorLine(line);
    },
    [onChange, readOnly, filePath],
  );

  const lineHeight = GUTTER_CONFIG.LINE_HEIGHT;
  const lineStarts = useMemo(() => computeLineStarts(value), [value]);
  const totalLines = lineStarts.length;

  const highlightsEnabled = !largeFile && !!filePath;
  const allHighlighted = useFullHighlight(
    filePath ?? null,
    highlightsEnabled,
    theme,
  );

  // Virtualise the overlay content – only render a couple of screens of lines.
  const containerHeight = Math.max(containerRef.current?.clientHeight ?? 0, lineHeight);
  const visibleStartLine = Math.floor(scrollTop / lineHeight);
  const visibleCount = Math.ceil((containerHeight + lineHeight) / lineHeight) * 2;
  const visibleEndLine = Math.min(visibleStartLine + visibleCount, totalLines);

  const visibleHighlighted = useMemo(
    () => allHighlighted.filter((l) => l.index >= visibleStartLine && l.index < visibleEndLine),
    [allHighlighted, visibleStartLine, visibleEndLine],
  );

  // Sync overlay scroll position to match the textarea.
  useEffect(() => {
    const overlay = highlightLayerRef.current;
    if (overlay && highlightsEnabled) {
      overlay.scrollTop = scrollTop;
      overlay.scrollLeft = scrollLeft;
    }
  }, [scrollTop, scrollLeft, highlightsEnabled]);

  const gutterWidth = largeFile ? 0 : computeGutterWidth(totalLines);
  const effectiveReadOnly = readOnly || largeFile;

  return (
    <div ref={containerRef} className={cn('flex h-full', className)}>
      {!largeFile && (
        <div
          className="shrink-0 relative overflow-hidden"
          style={{ width: gutterWidth }}
        >
          <LineNumberGutter
            lineCount={totalLines}
            cursorLine={cursorLine}
            lineHeight={lineHeight}
            scrollTop={scrollTop}
            containerHeight={containerHeight}
          />
        </div>
      )}

      <div className="flex-1 flex flex-col overflow-hidden relative">
        {largeFile && (
          <div className="text-muted-foreground text-xs p-1 bg-muted/80 shrink-0">
            File &gt; 5 MB – read‑only preview (first 50 000 characters shown)
          </div>
        )}

        {/* Highlight overlay – absolutely positioned over the textarea area
            with explicit overflow‑scroll (hidden scrollbar) so that programmatic
            scrollTop/scrollLeft actually displays the contents. */}
        {highlightsEnabled && (
          <div
            ref={highlightLayerRef}
            aria-hidden="true"
            className="absolute inset-0 pointer-events-none font-mono text-sm whitespace-pre select-none text-editor-foreground"
            style={{
              lineHeight: `${lineHeight}px`,
              fontFamily: FONT_TOKENS.editor,
              whiteSpace: 'pre',
              overflowWrap: 'normal',
              overflow: 'scroll',
              scrollbarWidth: 'none', // Firefox
              msOverflowStyle: 'none', // IE/Edge
            }}
          >
            {/* Total height placeholder that creates a scrollable area equal to the textarea */}
            <div style={{ height: totalLines * lineHeight, position: 'relative' }}>
              {/* Only render the visible lines, shifted to their correct position */}
              <div
                style={{
                  position: 'absolute',
                  top: 0,
                  left: 0,
                  width: '100%',
                  transform: `translateY(${visibleStartLine * lineHeight}px)`,
                }}
              >
                {visibleHighlighted.map((hl) => (
                  <div key={hl.index}>
                    {renderSpans(hl.spans, hl.text)}
                  </div>
                ))}
              </div>
            </div>
            {/* Hide scrollbar chrome for WebKit browsers */}
            <style>{`
              ::-webkit-scrollbar { display: none; }
            `}</style>
          </div>
        )}

        {/* Editable textarea – text is transparent so the highlight layer shows through */}
        <textarea
          ref={textareaRef}
          className="flex-1 resize-none outline-none bg-transparent font-mono text-sm p-0 overflow-auto scrollbar-none relative z-10"
          style={{
            lineHeight: `${lineHeight}px`,
            fontFamily: FONT_TOKENS.editor,
            whiteSpace: 'pre',
            overflowWrap: 'normal',
            wrap: 'off',
            color: highlightsEnabled ? 'transparent' : undefined,
            caretColor: highlightsEnabled
              ? 'var(--editor-cursor-color, #E2E8F0)'
              : effectiveReadOnly
                ? 'transparent'
                : undefined,
          }}
          value={value}
          readOnly={effectiveReadOnly}
          onChange={handleChange}
          onScroll={handleTextareaScroll}
          onSelect={handleSelectionChange}
          onClick={() => textareaRef.current?.focus()}
          spellCheck={false}
          autoComplete="off"
          autoCorrect="off"
        />
      </div>
    </div>
  );
}
