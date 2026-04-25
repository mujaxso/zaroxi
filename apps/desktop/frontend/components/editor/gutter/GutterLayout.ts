//! Layout computation for the gutter.
//! Pure functions that compute visible line ranges and positions.
//! No React dependencies.

import { GUTTER_CONFIG } from './GutterConfig';

export interface ViewportState {
  scrollTop: number;
  containerHeight: number;
  lineHeight: number;
  totalLines: number;
}

export interface VisibleLineRange {
  firstLine: number;
  lastLine: number;
}

/**
 * Compute the visible line range from viewport state.
 * Returns `null` when the container hasn't been measured or there are no lines.
 * This is O(1) – never iterates over the document.
 */
export function computeVisibleRange(viewport: ViewportState): VisibleLineRange | null {
  const { scrollTop, containerHeight, lineHeight, totalLines } = viewport;

  if (containerHeight <= 0 || lineHeight <= 0 || totalLines === 0) {
    return null;
  }

  const effectiveScrollTop = Math.max(0, scrollTop);
  const overscan = 3; // small overscan for smooth scrolling

  const firstLine = Math.max(
    0,
    Math.floor(effectiveScrollTop / lineHeight) - overscan,
  );
  const lastLine = Math.min(
    totalLines - 1,
    Math.ceil((effectiveScrollTop + containerHeight) / lineHeight) + overscan - 1,
  );

  if (!Number.isFinite(firstLine) || !Number.isFinite(lastLine)) {
    return null;
  }

  return { firstLine, lastLine };
}

/**
 * Compute the gutter width based on the total number of lines.
 * This is O(1) – just digit count.
 */
export function computeGutterWidth(totalLines: number): number {
  const digits = String(totalLines).length;
  return Math.max(
    GUTTER_CONFIG.MIN_WIDTH,
    digits * GUTTER_CONFIG.DIGIT_WIDTH +
      GUTTER_CONFIG.PADDING_LEFT +
      GUTTER_CONFIG.PADDING_RIGHT,
  );
}

/**
 * Compute the total height of all lines.
 */
export function computeTotalHeight(lineCount: number, lineHeight: number): number {
  return lineCount * lineHeight;
}
