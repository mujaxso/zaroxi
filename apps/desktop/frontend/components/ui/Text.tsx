import { cn } from '@/lib/utils';
import { ReactNode } from 'react';

interface TextProps {
  children: ReactNode;
  className?: string;
  variant?: 'body' | 'label' | 'caption' | 'code' | 'heading';
  size?: 'xs' | 'sm' | 'base' | 'lg' | 'xl';
  weight?: 'normal' | 'medium' | 'semibold' | 'bold';
  family?: 'sans' | 'mono' | 'icon';
  align?: 'left' | 'center' | 'right';
}

export function Text({
  children,
  className,
  variant = 'body',
  size = 'base',
  weight = 'normal',
  family = 'sans',
  align = 'left',
}: TextProps) {
  const sizeClasses = {
    xs: 'text-xs',
    sm: 'text-sm',
    base: 'text-base',
    lg: 'text-lg',
    xl: 'text-xl',
  };

  const weightClasses = {
    normal: 'font-normal',
    medium: 'font-medium',
    semibold: 'font-semibold',
    bold: 'font-bold',
  };

  const familyClasses = {
    sans: 'font-sans',
    mono: 'font-mono',
    icon: 'font-icon',
  };

  const alignClasses = {
    left: 'text-left',
    center: 'text-center',
    right: 'text-right',
  };

  return (
    <div className={cn(
      sizeClasses[size],
      weightClasses[weight],
      familyClasses[family],
      alignClasses[align],
      className
    )}>
      {children}
    </div>
  );
}
