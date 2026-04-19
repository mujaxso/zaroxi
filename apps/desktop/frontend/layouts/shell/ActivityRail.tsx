import { Icon } from '@/components/ui/Icon';
import { cn } from '@/lib/utils';

interface ActivityRailProps {
  activePanel: string;
  onPanelChange: (panel: 'explorer' | 'search' | 'git' | 'debug') => void;
  onAssistantToggle: () => void;
}

export function ActivityRail({ activePanel, onPanelChange, onAssistantToggle }: ActivityRailProps) {
  const items = [
    { id: 'explorer', icon: 'folder', label: 'Explorer' },
    { id: 'search', icon: 'search', label: 'Search' },
    { id: 'git', icon: 'git-branch', label: 'Git' },
    { id: 'debug', icon: 'bug', label: 'Debug' },
  ] as const;

  return (
    <div className="w-12 flex flex-col items-center py-4 border-r border-border bg-sidebar">
      {/* Main activity items */}
      <div className="flex flex-col items-center space-y-4">
        {items.map((item) => (
          <button
            key={item.id}
            onClick={() => onPanelChange(item.id)}
            className={cn(
              'w-10 h-10 flex items-center justify-center rounded-lg transition-colors',
              activePanel === item.id
                ? 'bg-accent text-accent-foreground'
                : 'hover:bg-muted text-muted-foreground'
            )}
            title={item.label}
          >
            <Icon name={item.icon} size={20} />
          </button>
        ))}
      </div>
      
      {/* Spacer */}
      <div className="flex-1" />
      
      {/* Bottom items */}
      <div className="flex flex-col items-center space-y-4">
        <button
          onClick={onAssistantToggle}
          className="w-10 h-10 flex items-center justify-center rounded-lg hover:bg-muted text-muted-foreground"
          title="AI Assistant"
        >
          <Icon name="sparkles" size={20} />
        </button>
        
        <button
          className="w-10 h-10 flex items-center justify-center rounded-lg hover:bg-muted text-muted-foreground"
          title="Settings"
        >
          <Icon name="settings" size={20} />
        </button>
      </div>
    </div>
  );
}
