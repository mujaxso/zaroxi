import React, { useMemo } from 'react';
import { GutterModel } from './GutterModel';
import { GutterView } from './GutterView';

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
  // Create the model (memoized based on inputs)
  const model = useMemo(
    () =>
      new GutterModel(
        scrollTop,
        lineHeight,
        lineCount,
        containerHeight,
        cursorLine,
        3, // overscan
      ),
    [scrollTop, lineHeight, lineCount, containerHeight, cursorLine],
  );

  // Early return for empty document
  if (lineCount === 0) {
    return (
      <div
        style={{
          width: model.width,
          pointerEvents: 'none',
          position: 'relative',
          overflow: 'visible',
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
        overflow: 'visible',
      }}
    >
      <GutterView model={model} />
    </div>
  );
};
