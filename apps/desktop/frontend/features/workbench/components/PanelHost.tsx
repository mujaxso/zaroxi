import { Suspense } from 'react';
import { useWorkbenchStore } from '../store/workbenchStore';
import { getActivityItem } from '../config/activityRegistry';
import { cn } from '@/lib/utils';

interface PanelHostProps {
  className?: string;
  side?: 'left' | 'right';
}

export function PanelHost({ className, side = 'left' }: PanelHostProps) {
  const { 
    activeLeftPanel, 
    activeRightPanel, 
    isLeftPanelVisible, 
    isRightPanelVisible,
    leftPanelWidth,
    rightPanelWidth 
  } = useWorkbenchStore();
  
  const activePanel = side === 'left' ? activeLeftPanel : activeRightPanel;
  const isVisible = side === 'left' ? isLeftPanelVisible : isRightPanelVisible;
  const panelWidth = side === 'left' ? leftPanelWidth : rightPanelWidth;
  
  if (!isVisible || !activePanel) {
    return null;
  }

  const activityItem = getActivityItem(activePanel);
  if (!activityItem) {
    console.warn(`No activity item found for panel ID: ${activePanel}`);
    return null;
  }

  const PanelComponent = activityItem.panelComponent;

  return (
    <div 
      className={cn(
        'h-full bg-panel overflow-hidden flex flex-col',
        side === 'left' ? 'border-r' : 'border-l',
        'min-w-[200px] max-w-[400px]',
        className
      )}
      style={{ width: panelWidth }}
    >
      <div className="flex-shrink-0 border-b border-divider px-4 py-2 flex items-center justify-between bg-activity-rail">
        <div className="flex items-center space-x-1.5">
          <h3 className="font-semibold text-sm text-primary leading-none">{activityItem.label}</h3>
          {activityItem.badge !== undefined && activityItem.badge > 0 && (
            <span className="px-1 py-0.5 text-xs rounded-full bg-accent text-on-accent font-medium leading-none">
              {activityItem.badge}
            </span>
          )}
        </div>
        {activityItem.shortcut && (
          <span className="text-xs text-primary/80 font-mono leading-none">
            {activityItem.shortcut}
          </span>
        )}
      </div>
      
      <div className="flex-1 overflow-auto bg-panel">
        <Suspense fallback={
          <div className="p-3">
            <div className="space-y-1.5">
              <div className="h-3 bg-muted rounded animate-pulse w-3/4"></div>
              <div className="h-3 bg-muted rounded animate-pulse w-1/2"></div>
              <div className="h-3 bg-muted rounded animate-pulse w-5/6"></div>
            </div>
          </div>
        }>
          <PanelComponent />
        </Suspense>
      </div>
    </div>
  );
}
