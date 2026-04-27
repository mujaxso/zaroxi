import { useEffect, useRef, useState, useCallback } from 'react';
import { cn } from '@/lib/utils';
import { useTabsStore } from '@/features/tabs/store';
import { LineNumberGutter } from './gutter/LineNumberGutter';
import { GUTTER_CONFIG } from './gutter/GutterConfig';
import { computeGutterWidth } from './gutter/GutterLayout';
import { FONT_TOKENS } from '@/lib/theme/font-tokens';

interface CodeEditorProps {
  initialValue: string;
  onChange: (value: string) => void;
  filePath?: string;
  language?: string;
  readOnly?: boolean;
  className?: string;
  /** set to true when the backend returns content_truncated (large file) */
  contentTruncated?: boolean;
}

/** Maximum characters used for the legacy length‑check (kept for backward compat). */
const TRUNCATE_CHARS = 50_000;

/** Fast line‑counting (O(n), not used for large files where the gutter is hidden). */
function fastLineCount(text: string): number {
  let lines = 1;
  const len = text.length;
  let i = 0;
  while (i < len) {
    if (text.charCodeAt(i) === 10) lines++;
    i++;
  }
  return lines;
}

/**
 * Plain‑text editor for **all** files, including large ones.
 *
 * For normal/medium files a gutter with line numbers is shown.
 * For large files the gutter is hidden to avoid expensive computations,
 * but the full text is still editable via a textarea.
 */
export function CodeEditor({
  initialValue,
  onChange,
  filePath,
  readOnly = false,
  className,
  contentTruncated,
}: CodeEditorProps) {
  // ── Local state ──────────────────────────────────────────────────
  const [value, setValue] = useState(initialValue);
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [scrollTop, setScrollTop] = useState(0);
  const [cursorLine, setCursorLine] = useState(1);

  const largeFile = contentTruncated ?? (initialValue.length >= TRUNCATE_CHARS);

  // Keep local value in sync with prop changes (e.g. when switching tabs)
  useEffect(() => {
    setValue(initialValue);
  }, [initialValue]);

  // ── Scroll & selection ───────────────────────────────────────────
  const handleTextareaScroll = useCallback(() => {
    const ta = textareaRef.current;
    if (!ta) return;
    setScrollTop(ta.scrollTop);
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

  // ── Derived metrics (only needed when gutter is shown) ───────────
  const lineHeight = GUTTER_CONFIG.LINE_HEIGHT;
  const totalLines = largeFile ? 0 : fastLineCount(value);
  const gutterWidth = largeFile ? 0 : computeGutterWidth(totalLines);

  // ── Layout ────────────────────────────────────────────────────────
  return (
    <div ref={containerRef} className={cn('flex h-full', className)}>
      {/* Gutter – disabled for large files to avoid crash from
          ginormous line counts */}
      {!largeFile && (
        <div style={{ width: gutterWidth, flexShrink: 0, position: 'relative', overflow: 'hidden' }}>
          <LineNumberGutter
            lineCount={totalLines}
            cursorLine={cursorLine}
            lineHeight={lineHeight}
            scrollTop={scrollTop}
            containerHeight={containerRef.current ? containerRef.current.clientHeight : 0}
          />
        </div>
      )}

      {/* Scrollable text area */}
      <div className="flex-1 flex flex-col overflow-hidden">
        {largeFile && (
          <div className="text-muted-foreground text-xs p-1 bg-muted/80 shrink-0">
            Large file – editing may be slow
          </div>
        )}
        <textarea
          ref={textareaRef}
          className="flex-1 resize-none outline-none bg-editor text-editor-foreground font-mono text-sm p-0 overflow-auto scrollbar-none"
          style={{
            lineHeight: `${lineHeight}px`,
            fontFamily: FONT_TOKENS.editor,
            whiteSpace: 'pre',
            overflowWrap: 'normal',
            wrap: 'off',
          }}
          value={value}
          readOnly={readOnly}
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
