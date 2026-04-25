import { useRef, useMemo } from 'react';
import { GUTTER_CONFIG } from './GutterConfig';

interface Props {
  lineCount: number;
  cursorLine: number;
  lineHeight: number;
  scrollTop: number;
  containerHeight: number;
  innerRef?: React.RefObject<HTMLDivElement>;
}

export function LineNumberGutter({
  lineCount,
  cursorLine,
  lineHeight,
  scrollTop,
  containerHeight,
  innerRef,
}: Props) {
  const ref = useRef<HTMLDivElement>(null);

  // Virtualized visible line range
  const start = useMemo(() => {
    const effectiveH = containerHeight > 0 ? containerHeight : 500; // fallback if container not yet measured
    const scrollLine = Math.floor(scrollTop / lineHeight);
    return Math.max(0, scrollLine - 1);
  }, [scrollTop, containerHeight, lineHeight]);

  const visibleLineCount = useMemo(() => {
    const effectiveH = containerHeight > 0 ? containerHeight : 500;
    return Math.ceil(effectiveH / lineHeight);
  }, [containerHeight, lineHeight]);

  const end = useMemo(() => {
    if (lineCount === 0) return 0;
    return Math.min(lineCount, start + visibleLineCount + 3);
  }, [start, visibleLineCount, lineCount]);

  // Gutter width based on number of digits of the last line
  const gutterWidth = useMemo(() => {
    const digits = String(lineCount).length;
    return Math.max(
      GUTTER_CONFIG.MIN_WIDTH,
      digits * GUTTER_CONFIG.DIGIT_WIDTH +
        GUTTER_CONFIG.PADDING_LEFT +
        GUTTER_CONFIG.PADDING_RIGHT,
    );
  }, [lineCount]);

  // Build the virtualized line‑number list using absolute positioning
  const numbers = [];
  for (let i = start; i < end; i++) {
    const lineNum = i + 1;
    const isCurrent = lineNum === cursorLine;
    numbers.push(
      <div
        key={i}
        style={{
          position: 'absolute',
          top: i * lineHeight,
          left: 0,
          right: 0,
          height: lineHeight,
          lineHeight: `${lineHeight}px`,
          paddingRight: GUTTER_CONFIG.PADDING_RIGHT,
          paddingLeft: GUTTER_CONFIG.PADDING_LEFT,
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

  return (
    <div
      ref={ref}
      className="h-full overflow-hidden shrink-0 border-r border-[rgba(128,128,128,0.18)]"
      style={{
        width: gutterWidth,
        pointerEvents: 'none',
      }}
    >
      <div
        ref={innerRef}
        className="min-w-full relative"
        style={{
          height: lineCount * lineHeight,
        }}
      >
        {numbers}
      </div>
    </div>
  );
}
