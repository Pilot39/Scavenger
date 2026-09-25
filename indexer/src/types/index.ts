/**
 * Type definitions for the indexer
 */

export interface Participant {
  address: string;
  role: 'Recycler' | 'Collector' | 'Manufacturer';
  name: string;
  latitude: string;
  longitude: string;
  registeredAt: Date;
}

export interface Waste {
  id: string;
  recyclerAddress: string;
  wasteType: string;
  weight: string;
  isConfirmed: boolean;
  isActive: boolean;
  registeredAt: Date;
}

export interface WasteTransfer {
  wasteId: string;
  fromAddress: string;
  toAddress: string;
  transferredAt: Date;
}

export interface TokenReward {
  recipientAddress: string;
  amount: string;
  wasteId: string;
  rewardedAt: Date;
}

export interface QueryOptions {
  limit?: number;
  offset?: number;
  orderBy?: string;
  orderDirection?: 'ASC' | 'DESC';
}

export interface PaginatedResult<T> {
  data: T[];
  total: number;
  limit: number;
  offset: number;
}
