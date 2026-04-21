import { PanelId } from '../store/workbenchStore';
import { IconName } from '@/components/ui/Icon';
import { lazy } from 'react';

// Lazy load panel components for better performance
const ExplorerPanel = lazy(() => import('@/features/explorer/panel/ExplorerPanel'));
const SearchPanel = lazy(() => import('@/features/search/panel/SearchPanel'));
const SourceControlPanel = lazy(() => import('@/features/scm/panel/SourceControlPanel'));
const DebugPanel = lazy(() => import('@/features/debug/panel/DebugPanel'));
const AssistantPanel = lazy(() => import('@/features/assistant/panel/AssistantPanel'));
const SettingsPanel = lazy(() => import('@/features/settings/panel/SettingsPanel'));

export interface ActivityItem {
  id: PanelId;
  label: string;
  icon: IconName;
  // Component to render when this panel is active
  panelComponent: React.ComponentType;
  // Whether this feature is available (for future feature flags)
  available: boolean;
  // Optional badge count (for notifications, etc.)
  badge?: number;
  // Keyboard shortcut for quick access
  shortcut?: string;
  // Description for tooltips
  description?: string;
}

export const ACTIVITY_REGISTRY: ActivityItem[] = [
  {
    id: 'explorer',
    label: 'Explorer',
    icon: 'folder',
    panelComponent: ExplorerPanel,
    available: true,
    description: 'Browse and manage workspace files',
  },
  {
    id: 'search',
    label: 'Search',
    icon: 'search',
    panelComponent: SearchPanel,
    available: true,
    shortcut: 'Ctrl+Shift+F',
    description: 'Search across workspace',
  },
  {
    id: 'git',
    label: 'Source Control',
    icon: 'git-branch',
    panelComponent: SourceControlPanel,
    available: true,
    badge: 0,
    description: 'Git version control operations',
  },
  {
    id: 'debug',
    label: 'Debug',
    icon: 'bug',
    panelComponent: DebugPanel,
    available: true,
    description: 'Debug and run your code',
  },
  {
    id: 'extensions',
    label: 'Extensions',
    icon: 'puzzle',
    panelComponent: SettingsPanel, // Temporary, will be replaced with actual ExtensionsPanel
    available: true,
    description: 'Manage extensions and add-ons',
  },
  {
    id: 'assistant',
    label: 'AI Assistant',
    icon: 'sparkles',
    panelComponent: AssistantPanel,
    available: true,
    description: 'AI-powered coding assistance',
  },
  {
    id: 'settings',
    label: 'Settings',
    icon: 'settings',
    panelComponent: SettingsPanel,
    available: true,
    description: 'Configure Zaroxi settings',
  },
];

// Helper to get activity item by ID
export function getActivityItem(id: PanelId): ActivityItem | undefined {
  return ACTIVITY_REGISTRY.find(item => item.id === id);
}

// Get all available activity items
export function getAvailableActivities(): ActivityItem[] {
  return ACTIVITY_REGISTRY.filter(item => item.available);
}
