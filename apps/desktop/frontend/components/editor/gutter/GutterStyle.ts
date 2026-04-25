//! Visual design constants for the gutter.
//! These define the premium IDE look.

export const GUTTER_STYLE = {
  /** Background color of the gutter (subtle distinction from code area) */
  BACKGROUND: 'rgba(128, 128, 128, 0.06)',
  /** Color for line numbers (low contrast) */
  LINE_NUMBER_COLOR: 'rgba(128, 128, 128, 0.5)',
  /** Color for the current line number (subtle emphasis) */
  CURRENT_LINE_COLOR: 'rgba(200, 200, 200, 0.9)',
  /** Separator between gutter and code */
  SEPARATOR_COLOR: 'rgba(128, 128, 128, 0.15)',
  /** Separator width */
  SEPARATOR_WIDTH: 1,
  /** Font family for line numbers */
  FONT_FAMILY: 'inherit',
  /** Font size for line numbers - use a reasonable size that matches the editor */
  FONT_SIZE: '12px',
  /** Line height for line numbers (should match editor line height) */
  LINE_HEIGHT: 22,
} as const;
