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
 * - For **large** (read‑only) files a static preview is shown.
 */
export function CodeEditor({
  initialValue,
  onChange,
  filePath,
  readOnly = false,
  className,
}: CodeEditorProps) {
  // ── Local state ──────────────────────────────────────────────────
  const [value, setValue] = useState(initialValue);
  const [largeFile, setLargeFile] = useState(false);
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [scrollTop, setScrollTop] = useState(0);
  const [cursorLine, setCursorLine] = useState(1);

  // Detect large file from the content length (the back‑end already truncated if needed)
  useEffect(() => {
    const isLarge = initialValue.length >= TRUNCATE_CHARS;
    setLargeFile(isLarge);
  }, [initialValue]);

  // Keep local value in sync with prop changes (e.g. when switching tabs)
  useEffect(() => {
    setValue(initialValue);
  }, [initialValue]);

  // ── Scroll & selection ───────────────────────────────────────────
  const handleScroll = useCallback(() => {
    const ta = textareaRef.current;
    if (!ta) return;
    setScrollTop(ta.scrollTop);
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

  // ── Derived metrics ───────────────────────────────────────────────
  const lineHeight = GUTTER_CONFIG.LINE_HEIGHT;
  const totalLines = fastLineCount(value);
  const gutterWidth = computeGutterWidth(totalLines);

  // ── Layout ────────────────────────────────────────────────────────
  return (
    <div ref={containerRef} className={cn('flex h-full', className)}>
      {/* Fixed gutter */}
      <div style={{ width: gutterWidth, flexShrink: 0, position: 'relative', overflow: 'hidden' }}>
        <LineNumberGutter
          lineCount={totalLines}
          cursorLine={cursorLine}
          lineHeight={lineHeight}
          scrollTop={scrollTop}
          containerHeight={containerRef.current ? containerRef.current.clientHeight : 0}
        />
      </div>

      {/* Scrollable text area */}
      {largeFile ? (
        <div className="flex-1 overflow-hidden bg-editor p-2 font-mono text-sm whitespace-pre">
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
          onScroll={handleScroll}
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
