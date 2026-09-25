import React from 'react';
import { CheckCircle2, AlertCircle } from 'lucide-react';
import { cn } from '@/lib/cn';

interface VerificationBadgeProps {
  isVerified: boolean;
  verificationLevel?: 'basic' | 'advanced' | 'premium';
  animated?: boolean;
  className?: string;
}

export const VerificationBadge: React.FC<VerificationBadgeProps> = ({
  isVerified,
  verificationLevel = 'basic',
  animated = true,
  className,
}) => {
  const getLevelColor = () => {
    switch (verificationLevel) {
      case 'premium':
        return 'text-yellow-500';
      case 'advanced':
        return 'text-blue-500';
      case 'basic':
      default:
        return 'text-green-500';
    }
  };

  const getLevelLabel = () => {
    switch (verificationLevel) {
      case 'premium':
        return 'Premium Verified';
      case 'advanced':
        return 'Advanced Verified';
      case 'basic':
      default:
        return 'Verified';
    }
  };

  if (!isVerified) {
    return (
      <div className={cn('flex items-center gap-1', className)}>
        <AlertCircle className="w-4 h-4 text-gray-400" />
        <span className="text-xs text-gray-500">Unverified</span>
      </div>
    );
  }

  return (
    <div
      className={cn(
        'flex items-center gap-1',
        animated && 'animate-pulse',
        className
      )}
    >
      <CheckCircle2 className={cn('w-4 h-4', getLevelColor())} />
      <span className={cn('text-xs font-medium', getLevelColor())}>
        {getLevelLabel()}
      </span>
    </div>
  );
};

export default VerificationBadge;
