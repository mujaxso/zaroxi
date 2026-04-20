/**
 * Nerd Font icon glyph mapping for Zaroxi IDE
 * 
 * This provides a structured way to use Nerd Font icons throughout the app
 * without scattering raw Unicode glyphs across components.
 * 
 * Reference: https://www.nerdfonts.com/cheat-sheet
 */
export const nerdFontIcons = {
  // Files and folders
  'file': '',
  'file-code': '',
  'file-json': 'ﬥ',
  'file-markdown': '',
  'file-config': '',
  'file-image': '',
  'folder': '',
  'folder-open': '',
  'folder-code': '',
  
  // UI icons
  'chevron-right': '',
  'chevron-down': '',
  'search': '',
  'settings': '',
  'terminal': '',
  'git-branch': '',
  'debug': '',
  'play': '',
  'stop': '',
  'refresh': '',
  'close': '',
  'menu': '',
  
  // Status indicators
  'check': '',
  'error': '',
  'warning': '',
  'info': '',
  'question': '',
  
  // Editor actions
  'save': '',
  'copy': '',
  'cut': '',
  'paste': '',
  'undo': '',
  'redo': '',
  
  // Workspace
  'workspace': '',
  'project': '',
  'explorer': '',
  'assistant': '',
  
  // Git
  'git-add': '',
  'git-commit': '',
  'git-push': '',
  'git-pull': '',
  
  // Programming languages
  'rust': '',
  'typescript': 'ﯤ',
  'javascript': '',
  'python': '',
  'go': '',
  'java': '',
  
  // Status bar icons
  'indent': '',
  'cursor': '',
} as const;

export type NerdFontIconName = keyof typeof nerdFontIcons;
