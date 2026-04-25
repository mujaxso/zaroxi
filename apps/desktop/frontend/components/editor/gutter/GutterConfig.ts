export const GUTTER_CONFIG = {
  /** Editor line height in pixels (must match the `leading-[22px]` class) */
  LINE_HEIGHT: 22,
  /** Approximate pixel width of a single digit character at 12px font size */
  DIGIT_WIDTH: 8,
  /** Left padding inside the gutter */
  PADDING_LEFT: 6,
  /** Right padding inside the gutter */
  PADDING_RIGHT: 10,
  /** Minimum gutter width in pixels (for 1‑digit files) */
  MIN_WIDTH: 32,
} as const;
