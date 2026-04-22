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
        side === 'left' ? 'border-r border-divider' : 'border-l border-divider',
        className
      )}
      style={{ width: panelWidth }}
    >
      <div className="flex-shrink-0 border-b border-divider px-4 py-3 bg-panel-header">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <h3 className="font-semibold text-sm text-primary">{activityItem.label}</h3>
            {activityItem.badge !== undefined && activityItem.badge > 0 && (
              <span className="px-1.5 py-0.5 text-xs rounded-full bg-accent text-on-accent">
                {activityItem.badge}
              </span>
            )}
          </div>
          {activityItem.shortcut && (
            <span className="text-xs text-muted font-mono">
              {activityItem.shortcut}
            </span>
          )}
        </div>
        {activityItem.description && (
          <p className="text-xs text-muted mt-1">{activityItem.description}</p>
        )}
      </div>
      
      <div className="flex-1 overflow-auto bg-panel">
        <Suspense fallback={
          <div className="p-4">
            <div className="space-y-2">
              <div className="h-4 bg-muted rounded animate-pulse w-3/4"></div>
              <div className="h-4 bg-muted rounded animate-pulse w-1/2"></div>
              <div className="h-4 bg-muted rounded animate-pulse w-5/6"></div>
            </div>
          </div>
        }>
          <PanelComponent />
        </Suspense>
      </div>
    </div>
  );
}
