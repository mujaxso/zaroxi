import { cn } from '@/lib/utils';
import { nerdFontIcons } from '@/lib/theme/nerd-font-icons';
import { useEffect, useRef, useState } from 'react';

// Export the IconName type for use in other files
export type IconName = keyof typeof nerdFontIcons;

interface IconProps {
  name: IconName;
  size?: number;
  className?: string;
  label?: string;
  debug?: boolean;
}

export function Icon({ name, size = 16, className, label, debug = false }: IconProps) {
  const iconGlyph = nerdFontIcons[name] || '?';
  const spanRef = useRef<HTMLSpanElement>(null);
  const [fontReady, setFontReady] = useState(false);

  useEffect(() => {
    // Check if the font is loaded and contains the glyph
    const checkFont = async () => {
      if (!spanRef.current) return;
      
      // Wait for fonts to be ready
      await document.fonts.ready;
      
      // Check if our specific font is loaded
      const isFontLoaded = document.fonts.check(`${size}px "JetBrainsMono Nerd Font"`) ||
                          document.fonts.check(`${size}px "JetBrainsMonoNL Nerd Font Mono"`) ||
                          document.fonts.check(`${size}px "JetBrainsMonoNL NFM"`);
      
      if (isFontLoaded) {
        setFontReady(true);
      } else {
        // Try to load the font if not loaded
        console.warn(`Font not loaded for icon ${name}, trying to detect...`);
        // Check if any font is loaded that can display the icon
        // We'll set fontReady to true anyway to try to display
        setFontReady(true);
      }
    };

    checkFont();
  }, [name, size]);

  return (
    <span 
      ref={spanRef}
      className={cn(
        'inline-flex items-center justify-center antialiased',
        'leading-none tracking-normal',
        'select-none',
        debug && 'outline outline-1 outline-red-500',
        className
      )}
      style={{ 
        fontSize: size,
        width: size,
        height: size,
        fontFamily: fontReady 
          ? '"JetBrainsMono Nerd Font", "JetBrainsMonoNL Nerd Font Mono", "JetBrainsMonoNL NFM", monospace'
          : 'monospace',
        fontVariantLigatures: 'normal',
        fontFeatureSettings: '"liga" 1, "calt" 1',
      }}
      role="img"
      aria-label={label || name}
      title={label || name}
      data-icon-name={name}
      data-font-ready={fontReady}
    >
      {iconGlyph}
    </span>
  );
}
