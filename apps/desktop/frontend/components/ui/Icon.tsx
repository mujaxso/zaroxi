import { cn } from '@/lib/utils';

interface IconProps {
  name: string;
  size?: number;
  className?: string;
}

// Simple icon component - in production, you'd use Lucide or similar
export function Icon({ name, size = 16, className }: IconProps) {
  // This is a placeholder - you should install a proper icon library
  return (
    <div 
      className={cn('flex items-center justify-center', className)}
      style={{ width: size, height: size }}
    >
      <span className="text-xs font-mono">{name.charAt(0).toUpperCase()}</span>
    </div>
  );
}
