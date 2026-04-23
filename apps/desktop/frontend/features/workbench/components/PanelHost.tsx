import { Suspense, useRef, useEffect, useState, useCallback } from 'react';
import { useWorkbenchStore } from '../store/workbenchStore';
import { getActivityItem } from '../config/activityRegistry';
import { cn } from '@/lib/utils';
import { Icon } from '@/components/ui/Icon';
import { LAYOUT } from '../config/layoutConstants';
import { useLayoutMode } from '@/hooks/useLayoutMode';

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
  
  const layoutMode = useLayoutMode();
  const [isResizing, setIsResizing] = useState(false);
  const panelRef = useRef<HTMLDivElement>(null);
  const startXRef = useRef(0);
  const startWidthRef = useRef(0);

  // Compute responsive width bounds based on current layout mode
  const isNarrow = layoutMode === 'narrow';
  const minPanelWidth = isNarrow
    ? (side === 'left' ? LAYOUT.panelLeft.minNarrowWidth : LAYOUT.panelRight.minNarrowWidth)
    : (side === 'left' ? LAYOUT.panelLeft.minWidth : LAYOUT.panelRight.minWidth);
  const maxPanelWidth = isNarrow
    ? (side === 'left' ? LAYOUT.panelLeft.maxNarrowWidth : LAYOUT.panelRight.maxNarrowWidth)
    : (side === 'left' ? LAYOUT.panelLeft.maxWidth : LAYOUT.panelRight.maxWidth);
  const factor = side === 'left' ? 0.25 : 0.22;

  // Clamp panel width when layout mode changes (e.g., window resize)
  useEffect(() => {
    const clamped = Math.max(minPanelWidth, Math.min(maxPanelWidth, panelWidth));
    if (clamped !== panelWidth) {
      if (side === 'left') {
        setLeftPanelWidth(clamped);
      } else {
        setRightPanelWidth(clamped);
      }
    }
    // We intentionally only react to layoutMode and the min/max values.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [layoutMode, minPanelWidth, maxPanelWidth]);

  const handleMouseDown = useCallback((e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsResizing(true);
    startXRef.current = e.clientX;
    startWidthRef.current = panelWidth;
    
    // Add event listeners for mouse move and up
    const handleMouseMove = (moveEvent: MouseEvent) => {
      moveEvent.preventDefault();
      const delta = side === 'left' 
        ? moveEvent.clientX - startXRef.current
        : startXRef.current - moveEvent.clientX;
      
      const minW = side === 'left' ? LAYOUT.panelLeft.minWidth : LAYOUT.panelRight.minWidth;
      const maxW = side === 'left' ? LAYOUT.panelLeft.maxWidth : LAYOUT.panelRight.maxWidth;
      const newWidth = Math.max(minW, Math.min(maxW, startWidthRef.current + delta));
      
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
      document.body.style.cursor = '';
      document.body.style.userSelect = '';
    };
    
    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', handleMouseUp);
    document.body.style.cursor = 'col-resize';
    document.body.style.userSelect = 'none';
  }, [panelWidth, side, setLeftPanelWidth, setRightPanelWidth]);

  // Clean up event listeners when resizing state changes
  useEffect(() => {
    if (!isResizing) {
      return;
    }
    
    const handleMouseUp = () => {
      setIsResizing(false);
      document.body.style.cursor = '';
      document.body.style.userSelect = '';
    };
    
    document.addEventListener('mouseup', handleMouseUp);
    
    return () => {
      document.removeEventListener('mouseup', handleMouseUp);
    };
  }, [isResizing]);
  
  // Clean up event listeners on unmount
  useEffect(() => {
    return () => {
      document.removeEventListener('mousemove', () => {});
      document.removeEventListener('mouseup', () => {});
      document.body.style.cursor = '';
      document.body.style.userSelect = '';
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
          'h-full bg-panel overflow-hidden flex flex-col relative min-h-0',
          side === 'left' ? 'border-r' : 'border-l',
          className
        )}
        style={{
          flex: side === 'right' ? '0 0 auto' : '0 1 auto',
          width: 'auto',
          flexBasis: panelWidth,
          minWidth: `${minPanelWidth}px`,
          maxWidth: `min(${maxPanelWidth}px, ${side === 'right' ? 0.35 : factor} * 100vw)`,
          order: side === 'right' ? 2 : 0,
        }}
      >
        {/* Resize handle */}
        <div 
          className={cn(
            'absolute top-0 bottom-0 w-2 cursor-col-resize z-50 resize-handle',
            side === 'left' ? 'right-0' : 'left-0',
            'hover:bg-accent active:bg-accent transition-colors',
            isResizing && 'bg-accent'
          )}
          style={{
            transform: side === 'left' ? 'translateX(1px)' : 'translateX(-1px)',
          }}
          onMouseDown={handleMouseDown}
        />
        
        <div className="border-b border-divider px-4 py-2 flex items-center justify-between bg-activity-rail h-9">
          <div className="flex items-center space-x-3">
            <div className="flex items-center space-x-2">
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
        
        <div className="flex-1 overflow-auto bg-panel h-full max-h-full min-h-0 w-full">
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
        <div className="fixed inset-0 z-40 cursor-col-resize no-select" />
      )}
    </>
  );
}
