import { useEffect, useRef, useState, useCallback, useMemo } from 'react';
import { cn } from '@/lib/utils';
import { useTabsStore } from '@/features/tabs/store';
import { LineNumberGutter } from './gutter/LineNumberGutter';
import { GUTTER_CONFIG } from './gutter/GutterConfig';
import { computeGutterWidth } from './gutter/GutterLayout';
import { FONT_TOKENS } from '@/lib/theme/font-tokens';
import { invoke } from '@tauri-apps/api/core';

/* ------------------------------------------------------------------ */
/*  Custom hook: fetch syntax highlights for the current viewport     */
/*  Accepts an optional `theme` to request theme‑aware colours.       */
/* ------------------------------------------------------------------ */
interface HighlightSpan {
  start: number;
  end: number;
  token_type: string;
  /** optional hex colour string, e.g. "#FF6B6B" */
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

function useHighlight(
  documentId: string | null,
  startLine: number,
  count: number,
  enabled: boolean,
  theme?: 'dark' | 'light',
) {
  const [lines, setLines] = useState<HighlightLine[]>([]);

  useEffect(() => {
    if (!documentId || !enabled || count === 0) {
      setLines([]);
      return;
    }

    let cancelled = false;

    async function fetch() {
      try {
        // IMPORTANT: Tauri command expects a single `request` struct with camelCase keys
        const res: HighlightResponse = await invoke('highlight_document', {
          request: {
            documentId,
            startLine,
            count,
            theme: theme ?? 'dark',
          },
        });
        if (!cancelled) {
          setLines(res.lines);
        }
      } catch (err) {
        console.warn('highlight_document failed:', err);
        if (!cancelled) setLines([]);
      }
    }

    fetch();
    return () => {
      cancelled = true;
    };
  }, [documentId, startLine, count, enabled, theme]);

  return lines;
}

/* ------------------------------------------------------------------ */
/*  Simple token‑type → CSS class mapping (kept as fallback)          */
/* ------------------------------------------------------------------ */
const tokenStyleMap: Record<string, string> = {
  keyword: 'text-purple-400',
  string: 'text-green-400',
  comment: 'text-gray-500 italic',
  function: 'text-blue-400',
  type: 'text-cyan-400',
  variable: 'text-orange-300',
  constant: 'text-yellow-300',
  number: 'text-pink-400',
  operator: 'text-red-300',
  punctuation: 'text-slate-400',
};

function renderSpans(spans: HighlightSpan[], lineText: string) {
  if (spans.length === 0) {
    return lineText;
  }

  const segments: React.ReactNode[] = [];
  let last = 0;
  for (let i = 0; i < spans.length; i++) {
    const sp = spans[i];
    if (sp.start > last) {
      segments.push(lineText.slice(last, sp.start));
    }
    // Rely on the inline color from the backend; fall back to CSS class only as a safety net.
    const tokenClass = tokenStyleMap[sp.token_type] ?? '';
    const key = `${sp.start}-${i}`; // unique per span
    segments.push(
      <span
        key={key}
        className={tokenClass}
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
  /** The active colour theme for syntax highlighting. */
  theme?: 'dark' | 'light';
}

const TRUNCATE_CHARS = 50_000;

/** Build an array of byte positions where each line starts (including newline). */
function computeLineStarts(text: string): number[] {
  const starts: number[] = [0];
  let pos = 0;
  while (pos < text.length) {
    const next = text.indexOf('\n', pos);
    if (next === -1) break;
    starts.push(next + 1); // start of next line
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
  // ── Local state ──────────────────────────────────────────────────
  const [value, setValue] = useState(initialValue);
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const highlightLayerRef = useRef<HTMLDivElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [scrollTop, setScrollTop] = useState(0);
  const [cursorLine, setCursorLine] = useState(1);

  const largeFile = contentTruncated ?? (initialValue.length >= TRUNCATE_CHARS);

  useEffect(() => {
    setValue(initialValue);
  }, [initialValue]);

  // ── Scroll & selection ───────────────────────────────────────────

  const handleTextareaScroll = useCallback(() => {
    setScrollTop(textareaRef.current?.scrollTop ?? 0);
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

  // ── Derived metrics ──────────────────────────────────────────────

  const lineHeight = GUTTER_CONFIG.LINE_HEIGHT;
  const lineStarts = useMemo(() => computeLineStarts(value), [value]);
  const totalLines = lineStarts.length;

  // Visible range for syntax highlight fetching
  const containerHeight = containerRef.current?.clientHeight ?? 0;
  const visibleStartLine = Math.floor(scrollTop / lineHeight);
  const visibleCount = Math.ceil((containerHeight + lineHeight) / lineHeight) * 2;

  const highlightsEnabled = !largeFile && !!filePath;
  const highlightedLines = useHighlight(
    filePath ?? null,
    visibleStartLine,
    visibleCount,
    highlightsEnabled,
    theme,
  );

  // ── Gutter metrics ───────────────────────────────────────────────

  const gutterWidth = largeFile ? 0 : computeGutterWidth(totalLines);
  const effectiveReadOnly = readOnly || largeFile;

  /* ---------- Layout ---------- */
  return (
    <div ref={containerRef} className={cn('flex h-full', className)}>
      {/* Gutter – disabled for large files */}
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

      {/* Scrollable text area with syntax overlay */}
      <div className="flex-1 flex flex-col overflow-hidden relative">
        {largeFile && (
          <div className="text-muted-foreground text-xs p-1 bg-muted/80 shrink-0">
            File &gt; 5 MB – read‑only preview (first 50 000 characters shown)
          </div>
        )}

        {/* Highlighted background layer – only visible lines are rendered */}
        {highlightsEnabled && (
          <div
            ref={highlightLayerRef}
            aria-hidden="true"
            className="absolute inset-0 overflow-hidden pointer-events-none font-mono text-sm whitespace-pre select-none"
            style={{
              lineHeight: `${lineHeight}px`,
              fontFamily: FONT_TOKENS.editor,
              whiteSpace: 'pre',
              overflowWrap: 'normal',
              color: 'transparent',
            }}
          >
            <div
              style={{
                height: totalLines * lineHeight,
                position: 'relative',
              }}
            >
              <div
                style={{
                  position: 'absolute',
                  top: 0,
                  left: 0,
                  width: '100%',
                  transform: `translateY(${-visibleStartLine * lineHeight}px)`,
                }}
              >
                {Array.from({ length: visibleCount }).map((_, i) => {
                  const lineIdx = visibleStartLine + i;
                  if (lineIdx >= totalLines) return null;
                  const startByte = lineStarts[lineIdx];
                  const endByte =
                    lineStarts[lineIdx + 1] ?? value.length;
                  let raw = value.slice(startByte, endByte);
                  if (raw.endsWith('\n')) raw = raw.slice(0, -1);
                  const hl = highlightedLines.find((l) => l.index === lineIdx);
                  return (
                    <div key={lineIdx}>
                      {hl ? renderSpans(hl.spans, hl.text) : raw}
                    </div>
                  );
                })}
              </div>
            </div>
          </div>
        )}

        {/* Editable textarea – text is transparent when highlights are enabled so the
            syntax layer shows through.  The caret remains visible via caretColor. */}
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
            caretColor: effectiveReadOnly ? 'transparent' : undefined,
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
