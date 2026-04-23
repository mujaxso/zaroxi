// Layout constants for the workbench shell
// All sizing policies live here so panels and app shells refer to the same values.

export const LAYOUT = {
  /** Width of the activity rail (px) */
  activityRailWidth: 48,

  /** Left‑side panel (explorer, search, git, debug etc.) – intentionally narrower */
  panelLeft: {
    minWidth: 100,
    defaultWidth: 110,
    maxWidth: 200,
    /** Minimum width when the window is narrow (< breakpoints.narrow). */
    minNarrowWidth: 110,
    /** Maximum width when the window is narrow. */
    maxNarrowWidth: 150,
  },

  /** Right‑side panel (assistant, extensions etc.) – wider for chat/reading */
  panelRight: {
    minWidth: 180,
    defaultWidth: 260,
    maxWidth: 350,
    /** Minimum width when the window is narrow (must remain readable). */
    minNarrowWidth: 140,
    /** Maximum width when the window is narrow. */
    maxNarrowWidth: 240,
  },

  /** Height of the compact top bar (px) */
  topBarHeight: 40,

  /** Height of the status bar (px) */
  statusBarHeight: 24,

  /**
   * Window width (px) below which side panels are automatically collapsed
   * to protect the editor area.
   */
  collapseThreshold: 700,

  /**
   * Width breakpoints for layout modes used by `useLayoutMode`.
   * These are intentionally lower than typical IDE breakpoints so that
   * the app *feels* like a desktop IDE even in a tiled half‑screen window.
   */
  breakpoints: {
    wide: 1400,
    medium: 1000,
    narrow: 800,
  },
} as const;
