import { Suspense, useRef, useEffect, useState, useCallback } from 'react';
import { useWorkbenchStore } from '../store/workbenchStore';
import { getActivityItem } from '../config/activityRegistry';
import { cn } from '@/lib/utils';
import { Icon } from '@/components/ui/Icon';

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
    rightPanelWidth,
    setLeftPanelWidth,
    setRightPanelWidth
  } = useWorkbenchStore();
  
  const activePanel = side === 'left' ? activeLeftPanel : activeRightPanel;
  const isVisible = side === 'left' ? isLeftPanelVisible : isRightPanelVisible;
  const panelWidth = side === 'left' ? leftPanelWidth : rightPanelWidth;
  
  const [isResizing, setIsResizing] = useState(false);
  const panelRef = useRef<HTMLDivElement>(null);
  const startXRef = useRef(0);
  const startWidthRef = useRef(0);

  const handleMouseDown = useCallback((e: React.MouseEvent) => {
    e.preventDefault();
    setIsResizing(true);
    startXRef.current = e.clientX;
    startWidthRef.current = panelWidth;
    
    // Add event listeners for mouse move and up
    const handleMouseMove = (moveEvent: MouseEvent) => {
      if (!isResizing) return;
      
      const delta = side === 'left' 
        ? moveEvent.clientX - startXRef.current
        : startXRef.current - moveEvent.clientX;
      
      const newWidth = Math.max(200, Math.min(600, startWidthRef.current + delta));
      
      if (side === 'left') {
        setLeftPanelWidth(newWidth);
      } else {
        setRightPanelWidth(newWidth);
      }
    };
    
    const handleMouseUp = () => {
      setIsResizing(false);
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
    
    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', handleMouseUp);
  }, [isResizing, panelWidth, side, setLeftPanelWidth, setRightPanelWidth]);

  // Clean up event listeners on unmount
  useEffect(() => {
    return () => {
      document.removeEventListener('mousemove', () => {});
      document.removeEventListener('mouseup', () => {});
    };
  }, []);

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
    <>
      <div 
        ref={panelRef}
        className={cn(
          'h-full bg-panel overflow-hidden flex flex-col relative',
          side === 'left' ? 'border-r' : 'border-l',
          'min-w-[200px] max-w-[600px]',
          className
        )}
        style={{ width: panelWidth }}
      >
        {/* Resize handle */}
        <div 
          className={cn(
            'absolute top-0 bottom-0 w-1 cursor-col-resize z-10',
            side === 'left' ? 'right-0' : 'left-0',
            'hover:bg-accent/50 active:bg-accent transition-colors',
            isResizing && 'bg-accent'
          )}
          onMouseDown={handleMouseDown}
        />
        
        <div className="border-b border-divider px-4 py-2 flex items-center justify-between bg-activity-rail h-9">
          <div className="flex items-center space-x-3">
            <div className="flex items-center space-x-2">
              <Icon name={activityItem.icon} size={14} className="text-primary" />
              <span className="text-sm font-semibold text-primary leading-none">{activityItem.label}</span>
              {activityItem.badge !== undefined && activityItem.badge > 0 && (
                <span className="px-1 py-0.5 text-xs rounded-full bg-accent text-on-accent font-medium leading-none">
                  {activityItem.badge}
                </span>
              )}
            </div>
            <span className="text-xs text-primary/80 font-mono truncate max-w-md leading-none">
              &nbsp;
            </span>
          </div>
          <div className="flex items-center space-x-2">
            {activityItem.shortcut ? (
              <span className="text-xs text-primary/80 font-mono leading-none">
                {activityItem.shortcut}
              </span>
            ) : (
              <div className="w-0"></div>
            )}
          </div>
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
      {/* Overlay during resizing */}
      {isResizing && (
        <div className="fixed inset-0 z-50 cursor-col-resize no-select" />
      )}
    </>
  );
}
