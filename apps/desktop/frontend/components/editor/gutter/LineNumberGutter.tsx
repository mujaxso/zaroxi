import { useRef, useMemo } from 'react';
import { GUTTER_CONFIG } from './GutterConfig';

interface Props {
  lineCount: number;
  cursorLine: number;
  lineHeight: number;
  innerRef?: React.RefObject<HTMLDivElement>;
}

export function LineNumberGutter({
  lineCount,
  cursorLine,
  lineHeight,
  innerRef,
}: Props) {
  const ref = useRef<HTMLDivElement>(null);

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

  // Always render every line (no virtualisation). This eliminates re‑renders caused by scroll‑top changes.
  const numbers = [];
  for (let i = 0; i < lineCount; i++) {
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
