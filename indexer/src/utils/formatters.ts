/**
 * Common data formatting utilities
 */

export function formatAddress(address: string): string {
  return address.substring(0, 6) + '...' + address.substring(address.length - 4);
}

export function formatWeight(weight: unknown): string {
  const num = Number(weight);
  return `${num.toFixed(2)} kg`;
}

export function formatDate(date: Date | string): string {
  const d = typeof date === 'string' ? new Date(date) : date;
  return d.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  });
}

export function formatTimestamp(date: Date | string): string {
  const d = typeof date === 'string' ? new Date(date) : date;
  return d.toLocaleString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  });
}

export function formatAmount(amount: unknown): string {
  const num = Number(amount);
  return num.toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 });
}

export function truncateString(str: string, length: number): string {
  return str.length > length ? str.substring(0, length) + '...' : str;
}
