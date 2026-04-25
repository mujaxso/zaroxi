import React, { useMemo } from 'react';
import { GutterModel } from './GutterModel';
import { GutterView } from './GutterView';
import { ViewportState } from './GutterLayout';

interface Props {
  lineCount: number;
  cursorLine: number;
  lineHeight: number;
  scrollTop: number;
  containerHeight: number;
}

/**
 * Thin wrapper that creates a GutterModel from props.
 *
 * This component is the public API for the gutter subsystem.
 * All layout logic lives in `GutterModel`; all rendering lives in `GutterView`.
 */
export const LineNumberGutter: React.FC<Props> = ({
  lineCount,
  cursorLine,
  lineHeight,
  scrollTop,
  containerHeight,
}) => {
  // Build viewport state
  const viewport: ViewportState = useMemo(
    () => ({
      scrollTop,
      containerHeight,
      lineHeight,
      totalLines: lineCount,
    }),
    [scrollTop, containerHeight, lineHeight, lineCount],
  );

  // Create the model (memoized based on inputs)
  const model = useMemo(
    () => new GutterModel(viewport, cursorLine),
    [viewport, cursorLine],
  );

  // Early return for empty document
  if (lineCount === 0) {
    return (
      <div
        style={{
          width: model.width,
          pointerEvents: 'none',
          position: 'relative',
          overflow: 'hidden',
          height: containerHeight,
        }}
      />
    );
  }

  return (
    <div
      style={{
        width: model.width,
        pointerEvents: 'none',
        position: 'relative',
        overflow: 'hidden',
        height: containerHeight,
      }}
    >
      <GutterView viewport={viewport} cursorLine={cursorLine} />
    </div>
  );
};
