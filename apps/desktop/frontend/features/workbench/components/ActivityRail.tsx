import { Icon } from '@/components/ui/Icon';
import { cn } from '@/lib/utils';
import { useWorkbenchStore } from '../store/workbenchStore';
import { getAvailableActivities } from '../config/activityRegistry';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/Tooltip';
import { LAYOUT } from '../config/layoutConstants';

interface ActivityRailProps {
  className?: string;
}

export function ActivityRail({ className }: ActivityRailProps) {
  const { 
    activeLeftPanel, 
    activeRightPanel, 
    isLeftPanelVisible, 
    isRightPanelVisible,
    togglePanel 
  } = useWorkbenchStore();
  
  const activities = getAvailableActivities();

  // Separate activities by position
  const topActivities = activities.filter(activity => activity.position !== 'bottom');
  const bottomActivities = activities.filter(activity => activity.position === 'bottom');

  return (
    <TooltipProvider delayDuration={300}>
      <div className={cn('flex flex-col items-center py-4 bg-activity-rail h-full min-h-0 max-h-full', className)} style={{ width: LAYOUT.activityRailWidth, borderRight: '1px solid var(--color-divider-subtle)' }}>
        {/* Top activity items */}
        <div className="flex flex-col items-center space-y-3">
          {topActivities.map((activity) => {
            const isActive = activity.side === 'left' 
              ? activeLeftPanel === activity.id && isLeftPanelVisible
              : activeRightPanel === activity.id && isRightPanelVisible;
            
            return (
              <Tooltip key={activity.id}>
                <TooltipTrigger asChild>
                  <button
                    onClick={() => togglePanel(activity.id)}
                    className={cn(
                      'relative w-9 h-9 flex items-center justify-center rounded-md transition-all duration-150',
                      isActive
                        ? 'bg-accent text-on-accent shadow-sm'
                        : 'text-primary hover:bg-hover-bg hover:text-accent active:scale-95'
                    )}
                    aria-label={activity.label}
                  >
                    <Icon 
                      name={activity.icon} 
                      size={16} 
                      className={isActive ? '' : 'opacity-80 hover:opacity-100'}
                    />
                    {activity.badge !== undefined && activity.badge > 0 && (
                      <span className="absolute -top-1 -right-1 w-4 h-4 text-xs flex items-center justify-center rounded-full bg-error text-on-accent font-medium border border-activity-rail">
                        {activity.badge > 9 ? '9+' : activity.badge}
                      </span>
                    )}
                  </button>
                </TooltipTrigger>
                <TooltipContent side="right" className="border bg-panel shadow-lg">
                  <div className="text-sm font-semibold text-primary">{activity.label}</div>
                  {activity.description && (
                    <div className="text-xs text-primary/90 mt-0.5">{activity.description}</div>
                  )}
                  {activity.shortcut && (
                    <div className="text-xs font-mono mt-1 text-accent">{activity.shortcut}</div>
                  )}
                </TooltipContent>
              </Tooltip>
            );
          })}
        </div>
        
        {/* Spacer - This pushes the bottom items down */}
        <div className="flex-1 min-h-6" />
        
        {/* Bottom activity items (Settings) */}
        <div className="flex flex-col items-center space-y-3">
          {bottomActivities.map((activity) => {
            const isActive = activity.side === 'left' 
              ? activeLeftPanel === activity.id && isLeftPanelVisible
              : activeRightPanel === activity.id && isRightPanelVisible;
            
            return (
              <Tooltip key={activity.id}>
                <TooltipTrigger asChild>
                  <button
                    onClick={() => togglePanel(activity.id)}
                    className={cn(
                      'relative w-9 h-9 flex items-center justify-center rounded-md transition-all duration-150',
                      isActive
                        ? 'bg-accent text-on-accent shadow-sm'
                        : 'text-primary hover:bg-hover-bg hover:text-accent active:scale-95'
                    )}
                    aria-label={activity.label}
                  >
                    <Icon 
                      name={activity.icon} 
                      size={16} 
                      className={isActive ? '' : 'opacity-80 hover:opacity-100'}
                    />
                    {activity.badge !== undefined && activity.badge > 0 && (
                      <span className="absolute -top-1 -right-1 w-4 h-4 text-xs flex items-center justify-center rounded-full bg-error text-on-accent font-medium border border-activity-rail">
                        {activity.badge > 9 ? '9+' : activity.badge}
                      </span>
                    )}
                  </button>
                </TooltipTrigger>
                <TooltipContent side="right" className="border bg-panel shadow-lg">
                  <div className="text-sm font-semibold text-primary">{activity.label}</div>
                  {activity.description && (
                    <div className="text-xs text-primary/90 mt-0.5">{activity.description}</div>
                  )}
                  {activity.shortcut && (
                    <div className="text-xs font-mono mt-1 text-accent">{activity.shortcut}</div>
                  )}
                </TooltipContent>
              </Tooltip>
            );
          })}
        </div>
      </div>
    </TooltipProvider>
  );
}
