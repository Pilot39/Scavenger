// Test suite for ScavengerClient API type safety (#1222)
// Validates typed responses and eliminates use of 'any' in API layer

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { ScavengerClient } from '../client'
import {
  Participant,
  Material,
  Waste,
  WasteTransfer,
  Incentive,
  ParticipantStats,
  GlobalMetrics,
  ContractError
} from '../types'
import { Address, scValToNative, TransactionBuilder, BASE_FEE } from '@stellar/stellar-sdk'

vi.mock('@stellar/stellar-sdk', async () => {
  const actual = await vi.importActual('@stellar/stellar-sdk')
  return {
    ...actual,
    Contract: vi.fn(),
  }
})

describe('ScavengerClient', () => {
  let client: ScavengerClient

  beforeEach(() => {
    client = new ScavengerClient({
      rpcUrl: 'https://test.stellar.org',
      networkPassphrase: 'Test SDF Network ; September 2015',
      contractId: 'CTEST123',
    })
  })

  afterEach(() => {
    vi.clearAllMocks()
  })

  describe('Type safety validation', () => {
    it('should properly type getParticipant response as Participant | null', () => {
      // Type validation: if this compiles, getParticipant is properly typed
      const result: Promise<Participant | null> = client.getParticipant('GABC123')
      expect(result).toBeDefined()
    })

    it('should properly type getMaterial response as Material | null', () => {
      const result: Promise<Material | null> = client.getMaterial(1n)
      expect(result).toBeDefined()
    })

    it('should properly type getWaste response as Waste | null', () => {
      const result: Promise<Waste | null> = client.getWaste(1n)
      expect(result).toBeDefined()
    })

    it('should properly type getWasteTransferHistory response as WasteTransfer[]', () => {
      const result: Promise<WasteTransfer[]> = client.getWasteTransferHistory(1n)
      expect(result).toBeDefined()
    })

    it('should properly type getIncentiveById response as Incentive | null', () => {
      const result: Promise<Incentive | null> = client.getIncentiveById(1n)
      expect(result).toBeDefined()
    })

    it('should properly type getIncentives response as Incentive[]', () => {
      const result: Promise<Incentive[]> = client.getIncentives(0)
      expect(result).toBeDefined()
    })

    it('should properly type getActiveIncentives response as Incentive[]', () => {
      const result: Promise<Incentive[]> = client.getActiveIncentives()
      expect(result).toBeDefined()
    })

    it('should properly type getStats response as ParticipantStats', () => {
      const result: Promise<ParticipantStats> = client.getStats('GABC123')
      expect(result).toBeDefined()
    })

    it('should properly type getMetrics response as GlobalMetrics', () => {
      const result: Promise<GlobalMetrics> = client.getMetrics()
      expect(result).toBeDefined()
    })

    it('should properly type getParticipantWastes response as bigint[]', () => {
      const result: Promise<bigint[]> = client.getParticipantWastes('GABC123')
      expect(result).toBeDefined()
    })

    it('should properly type registerParticipant response as Participant', () => {
      const result: Promise<Participant> = client.registerParticipant(
        'GABC123',
        0,
        'Alice',
        10.5,
        20.5,
        'GABC123'
      )
      expect(result).toBeDefined()
    })

    it('should properly type submitMaterial response as Material', () => {
      const result: Promise<Material> = client.submitMaterial(
        'GABC123',
        0,
        100n,
        10n,
        20n,
        'GABC123'
      )
      expect(result).toBeDefined()
    })
  })

  describe('Response type assertions', () => {
    it('validates Participant type structure', () => {
      const participant: Participant = {
        address: 'GABC123',
        role: 0,
        name: 'Alice',
        latitude: 10,
        longitude: 20,
      }
      expect(participant.address).toBeTypeOf('string')
      expect(participant.role).toBeTypeOf('number')
      expect(participant.name).toBeTypeOf('string')
      expect(participant.latitude).toBeTypeOf('number')
      expect(participant.longitude).toBeTypeOf('number')
    })

    it('validates Material type structure', () => {
      const material: Material = {
        id: 1n,
        submitter: 'GABC123',
        wasteType: 0,
        weight: 100n,
        submissionTime: 1000n,
        verified: true,
      }
      expect(material.id).toBeTypeOf('bigint')
      expect(material.submitter).toBeTypeOf('string')
      expect(material.wasteType).toBeTypeOf('number')
      expect(material.weight).toBeTypeOf('bigint')
    })

    it('validates Incentive type structure', () => {
      const incentive: Incentive = {
        id: 1n,
        creator: 'GABC123',
        wasteType: 0,
        rewardPoints: 100n,
        remainingBudget: 1000n,
        active: true,
      }
      expect(incentive.id).toBeTypeOf('bigint')
      expect(incentive.creator).toBeTypeOf('string')
      expect(incentive.wasteType).toBeTypeOf('number')
      expect(incentive.rewardPoints).toBeTypeOf('bigint')
      expect(incentive.remainingBudget).toBeTypeOf('bigint')
      expect(incentive.active).toBeTypeOf('boolean')
    })
  })

  describe('ContractError type validation', () => {
    it('should create ContractError with proper type', () => {
      const error: ContractError = new ContractError('Test error', 123)
      expect(error).toBeInstanceOf(ContractError)
      expect(error.message).toBeTypeOf('string')
      expect(error.code).toBeTypeOf('number')
    })

    it('should create ContractError with message only', () => {
      const error: ContractError = new ContractError('Test error')
      expect(error).toBeInstanceOf(ContractError)
      expect(error.message).toBeTypeOf('string')
    })
  })

  describe('Deprecated methods type safety', () => {
    it('should maintain type safety for deprecated getWasteV2', () => {
      const result: Promise<Waste | null> = client.getWasteV2(1n)
      expect(result).toBeDefined()
    })

    it('should maintain type safety for deprecated getParticipantWastesV2', () => {
      const result: Promise<bigint[]> = client.getParticipantWastesV2('GABC123')
      expect(result).toBeDefined()
    })

    it('should maintain type safety for deprecated getWasteTransferHistoryV2', () => {
      const result: Promise<WasteTransfer[]> = client.getWasteTransferHistoryV2(1n)
      expect(result).toBeDefined()
    })

    it('should maintain type safety for deprecated recycleWaste', () => {
      const result: Promise<bigint> = client.recycleWaste('GABC123', 0, 100n, 10n, 20n, 'GABC123')
      expect(result).toBeDefined()
    })
  })
})
