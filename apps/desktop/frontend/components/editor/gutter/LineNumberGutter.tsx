import { useRef, useMemo, useState, useLayoutEffect } from 'react';
import { GUTTER_CONFIG } from './GutterConfig';

interface Props {
  lineCount: number;
  cursorLine: number;
  lineHeight: number;
  scrollTop: number;
}

/** Number of extra lines above and below the visible area to keep rendered. */
const OVERSCAN = 3;

export const LineNumberGutter = ({
  lineCount,
  cursorLine,
  lineHeight,
  scrollTop,
}: Props) => {
  const outerRef = useRef<HTMLDivElement>(null);
  const [containerHeight, setContainerHeight] = useState(0);

  // Measure our own height as soon as the component mounts (and on resize)
  useLayoutEffect(() => {
    const updateHeight = () => {
      if (outerRef.current) {
        setContainerHeight(outerRef.current.clientHeight);
      }
    };
    updateHeight();
    const observer = new ResizeObserver(updateHeight);
    if (outerRef.current) observer.observe(outerRef.current);
    return () => observer.disconnect();
  }, []);

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

  // Visible line range (clamped, with overscan)
  const { firstLine, lastLine } = useMemo(() => {
    if (lineCount === 0 || containerHeight === 0) {
      return { firstLine: 0, lastLine: 0 };
    }
    const effectiveScrollTop = Math.max(0, scrollTop);
    const first = Math.max(0, Math.floor(effectiveScrollTop / lineHeight) - OVERSCAN);
    const last = Math.min(
      lineCount - 1,
      Math.ceil((effectiveScrollTop + containerHeight) / lineHeight) + OVERSCAN - 1,
    );
    return { firstLine: first, lastLine: last };
  }, [scrollTop, lineHeight, lineCount, containerHeight]);

  // Render only the visible line numbers
  const lineNumbers = useMemo(() => {
    const items = [];
    for (let lineIndex = firstLine; lineIndex <= lastLine; lineIndex++) {
      const lineNum = lineIndex + 1;
      const isCurrent = lineNum === cursorLine;
      items.push(
        <div
          key={lineIndex}
          style={{
            position: 'absolute',
            top: lineIndex * lineHeight,
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
    return items;
  }, [firstLine, lastLine, cursorLine, lineHeight]);

  const totalHeight = lineCount * lineHeight;

  return (
    <div
      ref={outerRef}
      className="h-full overflow-hidden shrink-0 border-r border-[rgba(128,128,128,0.18)]"
      style={{
        width: gutterWidth,
        pointerEvents: 'none',
        position: 'relative',
      }}
    >
      {/* Virtual scroll container: same size as the whole document but clipped */}
      <div
        className="min-w-full will-change-transform"
        style={{
          height: totalHeight,
          transform: `translateY(-${Math.max(0, scrollTop)}px)`,
          position: 'relative',
        }}
      >
        {lineNumbers}
      </div>
    </div>
  );
};
