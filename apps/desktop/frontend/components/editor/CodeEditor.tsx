import { useEffect, useRef, useState, useCallback } from 'react';
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
  /** set to true when the backend returns content_truncated (large file with preview) */
  contentTruncated?: boolean;
}

/** Maximum characters returned from back‑end for large files. */
const TRUNCATE_CHARS = 50_000;

/** Fast line‑counting that stops scanning once we exceed a safe limit. */
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
 * Simple plain‑text editor for normal / medium files.
 *
 * - Uses a single textarea for editing (content is the full rope text).
 * - Prevents any rendering of hidden lines; the native scroll handles everything.
 * - Horizontal scrolling works because `wrap="off"`.
 * - For **large** (read‑only) files a scrollable preview is shown **without a gutter**.
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

  // Determine whether this is a large file (content truncation flag from backend).
  // If no flag is provided, fall back to a simple length check for backward compatibility.
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

  const handlePreviewScroll = useCallback((e: React.UIEvent<HTMLDivElement>) => {
    setScrollTop(e.currentTarget.scrollTop);
  }, []);

  const handleSelectionChange = useCallback(() => {
    const ta = textareaRef.current;
    if (!ta) return;
    const sel = ta.selectionStart;
    // Cursor line is 1‑based
    const before = value.slice(0, sel).match(/\n/g);
    const line = before ? before.length + 1 : 1;
    setCursorLine(line);
  }, [value]);

  const handleChange = useCallback(
    (e: React.ChangeEvent<HTMLTextAreaElement>) => {
      if (readOnly || largeFile) return;
      const newVal = e.target.value;
      setValue(newVal);
      onChange(newVal);
      if (filePath) {
        useTabsStore.getState().markDirty(filePath);
      }
      // Update cursor line after change (since selection is delayed, use a cheap estimate)
      const selStart = e.target.selectionStart;
      const before = newVal.slice(0, selStart).match(/\n/g);
      const line = before ? before.length + 1 : 1;
      setCursorLine(line);
    },
    [onChange, readOnly, largeFile, filePath],
  );

  // ── Derived metrics (only needed for normal/medium files) ─────────
  const lineHeight = GUTTER_CONFIG.LINE_HEIGHT;
  const totalLines = fastLineCount(value);
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

      {/* Scrollable text area (or large‑file preview) */}
      {largeFile ? (
        <div
          className="flex-1 overflow-auto bg-editor p-2 font-mono text-sm whitespace-pre"
          style={{
            lineHeight: `${lineHeight}px`,
            fontFamily: FONT_TOKENS.editor,
          }}
          onScroll={handlePreviewScroll}
        >
          <div className="text-muted-foreground text-xs mb-1">
            File is too large for editing (showing preview of first {TRUNCATE_CHARS.toLocaleString()} characters).
          </div>
          <div>{value}</div>
        </div>
      ) : (
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
      )}
    </div>
  );
}
