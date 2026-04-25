import { useRef, useMemo, forwardRef, useImperativeHandle } from 'react';
import { GUTTER_CONFIG } from './GutterConfig';

interface Props {
  lineCount: number;
  cursorLine: number;
  lineHeight: number;
}

export const LineNumberGutter = forwardRef<HTMLDivElement, Props>(
  ({ lineCount, cursorLine, lineHeight }, outerRef) => {
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

    // Static full list of line numbers (max 10k lines when using truncation)
    const numbers = useMemo(() => {
      const items = [];
      for (let i = 0; i < lineCount; i++) {
        const lineNum = i + 1;
        const isCurrent = lineNum === cursorLine;
        items.push(
          <div
            key={i}
            style={{
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
      return items;
    }, [lineCount, cursorLine, lineHeight]);

    return (
      <div
        className="h-full overflow-hidden shrink-0 border-r border-[rgba(128,128,128,0.18)]"
        style={{
          width: gutterWidth,
          pointerEvents: 'none',
        }}
      >
        <div
          ref={outerRef}
          className="min-w-full"
          style={{
            height: lineCount * lineHeight,
            willChange: 'transform',
          }}
        >
          {numbers}
        </div>
      </div>
    );
  },
);
