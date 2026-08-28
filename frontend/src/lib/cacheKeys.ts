/**
 * Centralized React Query key factory for cache invalidation and co-location.
 */
export const cacheKeys = {
  participant: (address: string) => ['participant', address] as const,
  participantWastes: (address: string) => ['participantWastes', address] as const,
  participantStats: (address: string) => ['stats', address] as const,
  waste: (id: string) => ['waste', id] as const,
  metrics: () => ['metrics'] as const,
  incentives: (wasteType?: string) => ['incentives', wasteType] as const,
  activeIncentives: () => ['incentives', 'active'] as const,
  supplyChainStats: () => ['supplyChainStats'] as const,
} as const
