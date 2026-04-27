import { useEffect, useRef, useState, useCallback, useMemo } from 'react';
import { cn } from '@/lib/utils';
import { useTabsStore } from '@/features/tabs/store';
import { LineNumberGutter } from './gutter/LineNumberGutter';
import { GUTTER_CONFIG } from './gutter/GutterConfig';
import { computeGutterWidth } from './gutter/GutterLayout';
import { FONT_TOKENS } from '@/lib/theme/font-tokens';
import { invoke } from '@tauri-apps/api/core';

/* ------------------------------------------------------------------ */
/*  Custom hook: fetch full-file syntax highlights once per file     */
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
      console.log('[CodeEditor] fetching full highlights for', documentId);
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

/* ------------------------------------------------------------------ */
/*  Token‑type → CSS class mapping (used as fallback when inline       */
/*  colour is absent).  The mapping now covers every token name that   */
/*  the backend may emit.                                              */
/* ------------------------------------------------------------------ */
const tokenStyleMap: Record<string, string> = {
  // lower‑case keys match the backend highlight_tag_to_string output
  keyword: 'text-keyword',
  string: 'text-string',
  comment: 'text-comment italic',
  function: 'text-function',
  type: 'text-type',
  variable: 'text-variable',
  constant: 'text-constant',
  number: 'text-number',
  operator: 'text-operator',
  punctuation: 'text-punctuation',
  // pascal‑case variants (older captures may arrive like this)
  Keyword: 'text-keyword',
  String: 'text-string',
  Comment: 'text-comment italic',
  Function: 'text-function',
  Type: 'text-type',
  Variable: 'text-variable',
  Constant: 'text-constant',
  Number: 'text-number',
  Operator: 'text-operator',
  Punctuation: 'text-punctuation',
  // additional semantic tokens
  Attribute: 'text-attribute',
  attribute: 'text-attribute',
  Property: 'text-property',
  property: 'text-property',
  Namespace: 'text-namespace',
  namespace: 'text-namespace',
  Tag: 'text-tag',
  tag: 'text-tag',
  Macro: 'text-macro',
  macro: 'text-macro',
  Plain: 'text-plain',
  plain: 'text-plain',
};

/**
 * Merge overlapping / nested highlight spans into a non‑overlapping
 * sequence.  The innermost (shortest) span wins when two spans
 * compete for the same character.
 */
function mergeSpans(spans: HighlightSpan[], lineLen: number): HighlightSpan[] {
  if (spans.length === 0 || lineLen === 0) return [];

  // Sort shortest → longest (innermost first)
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

  // Compress contiguous runs with the same token into spans.
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

  // Remove overlaps so a character is never painted twice.
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
    const tokenClass = tokenStyleMap[sp.token_type] ?? '';
    const key = `${sp.start}-${i}`;
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
  const highlightedLines = useFullHighlight(
    filePath ?? null,
    highlightsEnabled,
    theme,
  );

  // Keep overlay scroll in sync with textarea
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
            containerHeight={containerRef.current?.clientHeight ?? 0}
          />
        </div>
      )}

      <div className="flex-1 flex flex-col overflow-hidden relative">
        {largeFile && (
          <div className="text-muted-foreground text-xs p-1 bg-muted/80 shrink-0">
            File &gt; 5 MB – read‑only preview (first 50 000 characters shown)
          </div>
        )}

        {/* Highlight overlay – synced via its scrollTop/scrollLeft */}
        {highlightsEnabled && (
          <div
            ref={highlightLayerRef}
            aria-hidden="true"
            className="absolute inset-0 overflow-hidden pointer-events-none font-mono text-sm whitespace-pre select-none text-editor-foreground"
            style={{
              lineHeight: `${lineHeight}px`,
              fontFamily: FONT_TOKENS.editor,
              whiteSpace: 'pre',
              overflowWrap: 'normal',
            }}
          >
            <div
              style={{
                height: totalLines * lineHeight,
                position: 'relative',
              }}
            >
              {Array.from({ length: totalLines }).map((_, lineIdx) => {
                const startByte = lineStarts[lineIdx];
                const endByte = lineStarts[lineIdx + 1] ?? value.length;
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
        )}

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
