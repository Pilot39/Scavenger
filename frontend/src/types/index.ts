// API Response Types
export interface ApiResponse<T> {
  data: T;
  status: number;
  message?: string;
}

export interface ApiError {
  code: string;
  message: string;
  details?: Record<string, unknown>;
}

// Participant Types
export interface Participant {
  address: string;
  role: ParticipantRole;
  name: string;
  latitude: number;
  longitude: number;
  registeredAt: number;
}

export enum ParticipantRole {
  Recycler = 0,
  Collector = 1,
  Manufacturer = 2,
}

// Waste Types
export interface Waste {
  id: string;
  type: WasteType;
  weight: number;
  owner: string;
  latitude: number;
  longitude: number;
  status: WasteStatus;
  createdAt: number;
}

export enum WasteType {
  Plastic = 'plastic',
  Paper = 'paper',
  Metal = 'metal',
  Glass = 'glass',
  Organic = 'organic',
}

export enum WasteStatus {
  Submitted = 'submitted',
  Verified = 'verified',
  Transferred = 'transferred',
  Deactivated = 'deactivated',
}

// Incentive Types
export interface Incentive {
  id: string;
  rewarder: string;
  wasteType: WasteType;
  rewardPoints: number;
  budget: number;
  active: boolean;
  createdAt: number;
}

// Contract Configuration
export interface ContractConfig {
  contractId: string;
  network: 'TESTNET' | 'MAINNET' | 'FUTURENET' | 'STANDALONE';
  rpcUrl: string;
}

// Form Types
export interface RegistrationFormData {
  name: string;
  role: ParticipantRole;
  latitude: number;
  longitude: number;
}

export interface WasteSubmissionFormData {
  type: WasteType;
  weight: number;
  latitude: number;
  longitude: number;
}

// Query Result Types
export interface QueryResult<T> {
  data: T | null;
  isLoading: boolean;
  error: ApiError | null;
}
