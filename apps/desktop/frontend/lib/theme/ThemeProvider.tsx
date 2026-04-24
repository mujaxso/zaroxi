import { ReactNode, useEffect, useState } from 'react';
import { useThemeStore, initializeTheme } from './theme-store';
import type { SemanticColors } from './types';

interface ThemeProviderProps {
  children: ReactNode;
}

/**
 * Apply semantic color tokens as CSS custom properties on the document root.
 * This allows all components to consume theme tokens via CSS variables
 * without hardcoding raw hex values.
 */
function applyThemeColors(colors: SemanticColors) {
  const root = document.documentElement;
  
  // Background surfaces
  root.style.setProperty('--color-app-background', colors.app_background);
  root.style.setProperty('--color-shell-background', colors.shell_background);
  root.style.setProperty('--color-panel-background', colors.panel_background);
  root.style.setProperty('--color-elevated-panel-background', colors.elevated_panel_background);
  root.style.setProperty('--color-editor-background', colors.editor_background);
  root.style.setProperty('--color-input-background', colors.input_background);
  root.style.setProperty('--color-status-bar-background', colors.status_bar_background);
  root.style.setProperty('--color-title-bar-background', colors.title_bar_background);
  root.style.setProperty('--color-activity-rail-background', colors.activity_rail_background);
  root.style.setProperty('--color-sidebar-background', colors.sidebar_background);
  root.style.setProperty('--color-tab-background', colors.tab_background);
  root.style.setProperty('--color-tab-active-background', colors.tab_active_background);
  root.style.setProperty('--color-assistant-panel-background', colors.assistant_panel_background);
  
  // Text colors
  root.style.setProperty('--color-text-primary', colors.text_primary);
  root.style.setProperty('--color-text-secondary', colors.text_secondary);
  root.style.setProperty('--color-text-muted', colors.text_muted);
  root.style.setProperty('--color-text-faint', colors.text_faint);
  root.style.setProperty('--color-text-on-accent', colors.text_on_accent);
  root.style.setProperty('--color-text-on-surface', colors.text_on_surface);
  root.style.setProperty('--color-text-disabled', colors.text_disabled);
  root.style.setProperty('--color-text-link', colors.text_link);
  
  // UI elements
  root.style.setProperty('--color-border', colors.border);
  root.style.setProperty('--color-border-subtle', colors.border_subtle);
  root.style.setProperty('--color-divider', colors.divider);
  root.style.setProperty('--color-divider-subtle', colors.divider_subtle);
  root.style.setProperty('--color-panel-header-background', colors.panel_header_background);
  root.style.setProperty('--color-nested-surface-background', colors.nested_surface_background);
  root.style.setProperty('--color-app-chrome-background', colors.app_chrome_background);
  root.style.setProperty('--color-tab-strip-background', colors.tab_strip_background);
  root.style.setProperty('--color-accent', colors.accent);
  root.style.setProperty('--color-accent-hover', colors.accent_hover);
  root.style.setProperty('--color-accent-soft', colors.accent_soft);
  root.style.setProperty('--color-accent-soft-background', colors.accent_soft_background);
  
  // States
  root.style.setProperty('--color-hover-background', colors.hover_background);
  root.style.setProperty('--color-active-background', colors.active_background);
  root.style.setProperty('--color-selected-background', colors.selected_background);
  root.style.setProperty('--color-selected-text-background', colors.selected_text_background);
  root.style.setProperty('--color-selected-editor-background', colors.selected_editor_background);
  
  // Status colors
  root.style.setProperty('--color-success', colors.success);
  root.style.setProperty('--color-warning', colors.warning);
  root.style.setProperty('--color-error', colors.error);
  root.style.setProperty('--color-info', colors.info);
  
  // Focus
  root.style.setProperty('--color-focus-ring', colors.focus_ring);
  
  // Editor specific
  root.style.setProperty('--color-editor-gutter-background', colors.editor_gutter_background);
  root.style.setProperty('--color-editor-line-highlight', colors.editor_line_highlight);
  root.style.setProperty('--color-editor-cursor', colors.editor_cursor);
  root.style.setProperty('--color-editor-selection', colors.editor_selection);
  root.style.setProperty('--color-editor-find-highlight', colors.editor_find_highlight);
  
  // Syntax colors
  root.style.setProperty('--color-syntax-keyword', colors.syntax_keyword);
  root.style.setProperty('--color-syntax-function', colors.syntax_function);
  root.style.setProperty('--color-syntax-string', colors.syntax_string);
  root.style.setProperty('--color-syntax-comment', colors.syntax_comment);
  root.style.setProperty('--color-syntax-type', colors.syntax_type);
  root.style.setProperty('--color-syntax-variable', colors.syntax_variable);
  root.style.setProperty('--color-syntax-constant', colors.syntax_constant);
}

export function ThemeProvider({ children }: ThemeProviderProps) {
  const { themeData, isLoading } = useThemeStore();
  const [themeReady, setThemeReady] = useState(false);
  
  // Apply theme colors whenever themeData changes
  useEffect(() => {
    if (themeData?.colors) {
      applyThemeColors(themeData.colors);
    }
  }, [themeData]);
  
  useEffect(() => {
    // Theme is already applied immediately when the module loads
    // Wait for the next animation frame to ensure CSS is applied
    const frameId = requestAnimationFrame(() => {
      setThemeReady(true);
    });
    
    // Initialize the full theme logic (backend, listeners, etc.)
    const cleanup = initializeTheme();
    
    return () => {
      cancelAnimationFrame(frameId);
      cleanup();
    };
  }, []);
  
  // Don't render children until theme is ready
  // This prevents flash of unstyled content
  if (!themeReady) {
    return null;
  }
  
  return <div className="min-h-screen w-full h-full">{children}</div>;
}
