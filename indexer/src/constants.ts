/**
 * Application constants
 */

export const WASTE_TYPES = ['Paper', 'Plastic', 'Metal', 'Glass', 'Organic', 'Electronic'] as const;
export const PARTICIPANT_ROLES = ['Recycler', 'Collector', 'Manufacturer'] as const;

export const QUERY_LIMITS = {
  DEFAULT: 20,
  MAX: 1000,
  MIN: 1,
} as const;

export const PERFORMANCE = {
  SLOW_QUERY_THRESHOLD_MS: 100,
  CONNECTION_TIMEOUT_MS: 5000,
  QUERY_TIMEOUT_MS: 30000,
} as const;

export const PAGINATION = {
  DEFAULT_PAGE_SIZE: 20,
  MAX_PAGE_SIZE: 100,
} as const;
