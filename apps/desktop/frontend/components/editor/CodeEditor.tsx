import { useEffect, useRef, useState, useCallback, useMemo } from 'react';
import { cn } from '@/lib/utils';

interface CodeEditorProps {
  initialValue: string;
  onChange: (value: string) => void;
  language?: string;
  readOnly?: boolean;
  className?: string;
  /** Total number of lines in the document (for virtual scrolling) */
  totalLines?: number;
  /** Callback to request more lines when scrolling */
  onRequestLines?: (startLine: number, count: number) => Promise<{ lines: { index: number; text: string }[] }>;
  /** Whether the document is in large file mode */
  largeFileMode?: boolean;
}

const LINE_HEIGHT = 22; // pixels
const OVERSCAN_LINES = 10; // lines to render above/below viewport

export function CodeEditor({
  initialValue,
  onChange,
  language = 'plaintext',
  readOnly = false,
  className,
  totalLines,
  onRequestLines,
  largeFileMode = false,
}: CodeEditorProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const [value, setValue] = useState(initialValue);
  const [scrollTop, setScrollTop] = useState(0);
  const [containerHeight, setContainerHeight] = useState(600);
  const [visibleLines, setVisibleLines] = useState<{ index: number; text: string }[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [scrollVersion, setScrollVersion] = useState(0);
  const onRequestLinesRef = useRef(onRequestLines);
  onRequestLinesRef.current = onRequestLines;

  // Compute the actual first visible line (without overscan) for transform offsets
  const actualFirstVisibleLine = useMemo(() => {
    return Math.max(0, Math.floor(scrollTop / LINE_HEIGHT));
  }, [scrollTop]);

  // Compute visible range – stable reference to avoid infinite effect loops
  const visibleRangeKey = useMemo(() => {
    const startLine = Math.max(0, Math.floor(scrollTop / LINE_HEIGHT) - OVERSCAN_LINES);
    const endLine = Math.min(
      totalLines ?? value.split('\n').length,
      Math.ceil((scrollTop + containerHeight) / LINE_HEIGHT) + OVERSCAN_LINES
    );
    return `${startLine}-${endLine}`;
  }, [scrollTop, containerHeight, totalLines, value]);

  // Compute visible lines for small files (no onRequestLines) directly from value
  const localVisibleLines = useMemo(() => {
    if (onRequestLines) return null; // not used for large files
    const lines = value.split('\n');
    const startLine = Math.max(0, Math.floor(scrollTop / LINE_HEIGHT) - OVERSCAN_LINES);
    const endLine = Math.min(
      lines.length,
      Math.ceil((scrollTop + containerHeight) / LINE_HEIGHT) + OVERSCAN_LINES
    );
    const result: { index: number; text: string }[] = [];
    for (let i = startLine; i < endLine; i++) {
      result.push({ index: i, text: lines[i] ?? '' });
    }
    return result;
  }, [value, scrollTop, containerHeight, onRequestLines]);

  // Fetch visible lines when range changes OR when scrollVersion increments
  useEffect(() => {
    const fetchFn = onRequestLinesRef.current;
    if (fetchFn) {
      const [startStr, endStr] = visibleRangeKey.split('-');
      const startLine = Number(startStr);
      const endLine = Number(endStr);
      
      const fetchLines = async () => {
        setIsLoading(true);
        try {
          const result = await fetchFn(startLine, endLine - startLine);
          setVisibleLines(result.lines);
        } catch (error) {
          console.error('Failed to fetch visible lines:', error);
        } finally {
          setIsLoading(false);
        }
      };
      
      fetchLines();
    }
  }, [visibleRangeKey, scrollVersion]);

  // Handle scroll – also bump scrollVersion to force re-fetch even if range key doesn't change
  const handleScroll = useCallback((e: React.UIEvent<HTMLDivElement>) => {
    setScrollTop(e.currentTarget.scrollTop);
    setScrollVersion((v) => v + 1);
  }, []);

  // Observe container size
  useEffect(() => {
    const container = containerRef.current;
    if (!container) return;
    
    const observer = new ResizeObserver((entries) => {
      for (const entry of entries) {
        setContainerHeight(entry.contentRect.height);
      }
    });
    
    observer.observe(container);
    return () => observer.disconnect();
  }, []);

  // Sync initial value only on mount (not on every render)
  const initialValueRef = useRef(initialValue);
  useEffect(() => {
    if (initialValueRef.current !== initialValue) {
      initialValueRef.current = initialValue;
      setValue(initialValue);
    }
  }, [initialValue]);

  const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    // Ignore changes when read‑only
    if (readOnly) return;
    const newValue = e.target.value;
    setValue(newValue);
    onChange(newValue);
  };

  // Compute total height for scroll container
  const totalHeight = useMemo(() => {
    if (totalLines) {
      return totalLines * LINE_HEIGHT;
    }
    // Fallback: estimate from current content
    return Math.max(containerHeight, value.split('\n').length * LINE_HEIGHT);
  }, [totalLines, value, containerHeight]);

  // Compute line numbers for visible range
  const lineNumbers = useMemo(() => {
    const [startStr, endStr] = visibleRangeKey.split('-');
    const startLine = Number(startStr);
    const endLine = Number(endStr);
    const numbers: number[] = [];
    const maxLine = totalLines ?? value.split('\n').length;
    for (let i = startLine; i < endLine && i < maxLine; i++) {
      numbers.push(i + 1);
    }
    return numbers;
  }, [visibleRangeKey, totalLines, value]);

  // Determine whether we are in virtual‑scrolling mode (large file)
  const isVirtualScrolling = !!onRequestLines;

  // Render a read‑only line‑by‑line view for all files (both small and large)
  const renderVirtualLines = () => {
    // Determine which lines to render
    const linesToRender = isVirtualScrolling ? visibleLines : (localVisibleLines ?? []);
    
    // Compute offset based on the visible start line (without overscan)
    const offsetY = actualFirstVisibleLine * LINE_HEIGHT;
    
    return (
      <div
        className="absolute left-8 top-0 right-0 pr-4 font-mono"
        style={{
          fontSize: '14px',
          lineHeight: `${LINE_HEIGHT}px`,
          transform: `translateY(${offsetY}px)`,
        }}
      >
        {linesToRender.map((line) => (
          <div key={line.index} style={{ height: LINE_HEIGHT, lineHeight: `${LINE_HEIGHT}px` }} className="whitespace-pre overflow-hidden">
            {line.text || '\u00A0'}
          </div>
        ))}
      </div>
    );
  };

  return (
    <div className={cn('relative h-full', className)} ref={containerRef}>
      {largeFileMode && (
        <div className="absolute top-0 left-0 right-0 z-10 bg-yellow-100 dark:bg-yellow-900 text-yellow-800 dark:text-yellow-200 text-xs px-2 py-1 text-center">
          Large file mode — some features are disabled for performance
        </div>
      )}
      <div className="absolute inset-0 bg-editor code-editor-font">
        {/* Scrollable container */}
        <div
          className="absolute inset-0 overflow-auto"
          onScroll={handleScroll}
          style={{ paddingTop: largeFileMode ? '20px' : '0' }}
        >
          {/* Spacer for scroll height */}
          <div style={{ height: totalHeight, position: 'relative' }}>
            {/* Line numbers gutter */}
            <div
              className="absolute left-0 top-0 w-8 bg-editor border-r border-border flex flex-col items-center font-mono text-xs text-muted-foreground"
              style={{
                transform: `translateY(${actualFirstVisibleLine * LINE_HEIGHT}px)`,
              }}
            >
              {lineNumbers.map((num) => (
                <div key={num} style={{ height: LINE_HEIGHT, lineHeight: `${LINE_HEIGHT}px` }} className="py-0">
                  {num}
                </div>
              ))}
            </div>
            
            {/* For all files: read‑only line view */}
            {renderVirtualLines()}
          </div>
        </div>
        
        {/* Loading indicator */}
        {isLoading && (
          <div className="absolute bottom-2 right-2 bg-background/80 text-xs px-2 py-1 rounded">
            Loading...
          </div>
        )}
      </div>
    </div>
  );
}
