/**
 * Zaroxi Studio – Font Token System
 *
 * Centralised font configuration that mirrors the CSS custom
 * properties defined in `lib/theme/fonts.css`.  Every component
 * should import these tokens rather than hardcoding font‑family
 * strings.
 *
 * The tokens are designed to be used with the `style` prop or
 * with Tailwind’s arbitrary‑value syntax (e.g. `font-[var(--font-editor)]`).
 */

export const FONT_TOKENS = {
  /** Font stack for the code editor and any monospaced code display. */
  editor: 'var(--font-editor)',
  /** Font stack for general UI elements (buttons, labels, panels). */
  ui: 'var(--font-ui)',
  /** Font stack for Nerd‑Font icon glyphs. */
  icon: 'var(--font-icon)',
  /** Generic monospace fallback. */
  mono: 'var(--font-mono)',
} as const;

export type FontToken = keyof typeof FONT_TOKENS;
