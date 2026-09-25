import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import VerificationBadge from './index';

describe('VerificationBadge', () => {
  it('renders verified badge with basic level', () => {
    render(<VerificationBadge isVerified={true} verificationLevel="basic" />);
    expect(screen.getByText('Verified')).toBeInTheDocument();
  });

  it('renders verified badge with advanced level', () => {
    render(<VerificationBadge isVerified={true} verificationLevel="advanced" />);
    expect(screen.getByText('Advanced Verified')).toBeInTheDocument();
  });

  it('renders verified badge with premium level', () => {
    render(<VerificationBadge isVerified={true} verificationLevel="premium" />);
    expect(screen.getByText('Premium Verified')).toBeInTheDocument();
  });

  it('renders unverified badge', () => {
    render(<VerificationBadge isVerified={false} />);
    expect(screen.getByText('Unverified')).toBeInTheDocument();
  });

  it('applies animation class when animated prop is true', () => {
    const { container } = render(
      <VerificationBadge isVerified={true} animated={true} />
    );
    expect(container.firstChild).toHaveClass('animate-pulse');
  });

  it('does not apply animation class when animated prop is false', () => {
    const { container } = render(
      <VerificationBadge isVerified={true} animated={false} />
    );
    expect(container.firstChild).not.toHaveClass('animate-pulse');
  });
});
