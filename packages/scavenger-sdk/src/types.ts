/** Role a participant can have in the Scavngr ecosystem. */
export enum Role {
  Recycler = 'Recycler',
  Collector = 'Collector',
  Manufacturer = 'Manufacturer',
}

/** Types of waste materials tracked by the contract. */
export enum WasteType {
  Paper = 0,
  PetPlastic = 1,
  Plastic = 2,
  Metal = 3,
  Glass = 4,
  Organic = 5,
  Electronic = 6,
}

/** Certification level for participants. */
export enum CertificationLevel {
  Beginner = 'Beginner',
  Intermediate = 'Intermediate',
  Advanced = 'Advanced',
  Expert = 'Expert',
}

/** Supported Stellar network presets. */
export enum Network {
  /** Local standalone network (Docker). */
  Standalone = 'standalone',
  /** SDF testnet. */
  Testnet = 'testnet',
  /** SDF futurenet. */
  Futurenet = 'futurenet',
  /** Stellar mainnet. */
  Mainnet = 'mainnet',
}

/** A registered participant on the Scavngr contract. */
export interface Participant {
  address: string
  role: Role
  name: string
  latitude: number
  longitude: number
  registered_at: number
}

/** An incentive (reward program) for a specific waste type. */
export interface Incentive {
  id: number
  rewarder: string
  waste_type: WasteType
  reward_points: number
  total_budget: number
  remaining_budget: number
  active: boolean
  created_at: number
}

/** A material submitted to the system. */
export interface Material {
  id: number
  waste_type: WasteType
  weight: number
  submitter: string
  current_owner: string
  submitted_at: number
  verified: boolean
  is_active: boolean
  is_confirmed: boolean
  confirmer: string
}

/** A tracked waste item. */
export interface Waste {
  waste_id: bigint
  waste_type: WasteType
  weight: bigint
  current_owner: string
  latitude: bigint
  longitude: bigint
  recycled_timestamp: number
  is_active: boolean
  is_confirmed: boolean
  confirmer: string
}

/** A waste transfer record. */
export interface WasteTransfer {
  waste_id: number
  from: string
  to: string
  transferred_at: number
}

/** Statistics for a specific participant. */
export interface ParticipantStats {
  address: string
  total_earned: bigint
  materials_submitted: number
  transfers_count: number
}

/** Global ecosystem metrics. */
export interface GlobalMetrics {
  total_wastes_count: number
  total_tokens_earned: bigint
}

/** Supply chain aggregated statistics. */
export interface SupplyChainStats {
  total_wastes: bigint
  total_weight: bigint
  total_tokens: bigint
}

/** Network configuration for connecting to a Stellar RPC endpoint. */
export interface NetworkConfig {
  /** Soroban RPC URL. */
  rpcUrl: string
  /** Network passphrase for transaction signing. */
  networkPassphrase: string
}

/** Options for initializing the SDK client. */
export interface ClientOptions {
  /** Soroban RPC URL. */
  rpcUrl: string
  /** Stellar network passphrase. */
  networkPassphrase: string
  /** Scavngr contract address (Contract ID). */
  contractId: string
  /** Optional timeout in milliseconds for transaction confirmation polling. */
  pollTimeoutMs?: number
  /** Optional interval in milliseconds between poll attempts. */
  pollIntervalMs?: number
}

/** Material batch submission payload. */
export interface MaterialBatchItem {
  wasteType: WasteType
  weight: bigint
}
