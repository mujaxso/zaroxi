import React, { useMemo } from 'react';
import { GUTTER_CONFIG } from './GutterConfig';
import { GUTTER_STYLE } from './GutterStyle';
import { computeVisibleRange, computeGutterWidth, ViewportState } from './GutterLayout';

interface GutterViewProps {
  viewport: ViewportState;
  cursorLine: number; // 1-based
}

/**
 * Pure rendering component for the gutter.
 * Only renders visible lines plus overscan.
 * Never iterates over the full document.
 */
export const GutterView: React.FC<GutterViewProps> = React.memo(({ viewport, cursorLine }) => {
  const { scrollTop, lineHeight, totalLines, containerHeight } = viewport;

  const visibleRange = useMemo(
    () => computeVisibleRange(viewport),
    [scrollTop, containerHeight, lineHeight, totalLines],
  );

  const gutterWidth = useMemo(
    () => computeGutterWidth(totalLines),
    [totalLines],
  );

  // Build visible line elements
  const lineElements = useMemo(() => {
    if (!visibleRange) return null;

    const elements: React.ReactNode[] = [];
    const { firstLine, lastLine } = visibleRange;

    for (let lineIndex = firstLine; lineIndex <= lastLine; lineIndex++) {
      const lineNum = lineIndex + 1;
      const isCurrent = lineNum === cursorLine;
      // Position relative to scroll offset so line numbers align with code
      const top = lineIndex * lineHeight - scrollTop;

      elements.push(
        <div
          key={lineIndex}
          style={{
            position: 'absolute',
            top,
            left: 0,
            right: 0,
            height: lineHeight,
            lineHeight: `${lineHeight}px`,
            paddingLeft: GUTTER_CONFIG.PADDING_LEFT,
            paddingRight: GUTTER_CONFIG.PADDING_RIGHT,
            textAlign: 'right',
            fontFamily: GUTTER_STYLE.FONT_FAMILY,
            fontSize: GUTTER_STYLE.FONT_SIZE,
            color: isCurrent ? GUTTER_STYLE.CURRENT_LINE_COLOR : GUTTER_STYLE.LINE_NUMBER_COLOR,
            fontWeight: isCurrent ? 600 : 400,
            overflow: 'hidden',
            whiteSpace: 'nowrap',
            pointerEvents: 'auto', // allow click for future breakpoints
            cursor: 'pointer',
          }}
          className="gutter-line"
          data-line-index={lineIndex}
          data-line-number={lineNum}
        >
          {lineNum}
        </div>,
      );
    }
    return elements;
  }, [visibleRange, lineHeight, cursorLine, scrollTop]);

  return (
    <div
      style={{
        position: 'relative',
        width: gutterWidth,
        height: containerHeight,
        backgroundColor: GUTTER_STYLE.BACKGROUND,
        overflow: 'hidden',
        userSelect: 'none',
      }}
    >
      {/* Separator line */}
      <div
        style={{
          position: 'absolute',
          right: 0,
          top: 0,
          bottom: 0,
          width: GUTTER_STYLE.SEPARATOR_WIDTH,
          backgroundColor: GUTTER_STYLE.SEPARATOR_COLOR,
        }}
      />
      {/* Visible line numbers */}
      {lineElements}
    </div>
  );
});

GutterView.displayName = 'GutterView';
