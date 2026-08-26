import { useQuery } from '@tanstack/react-query'
import { ScavengerClient } from '@/api/client'
import { WasteType } from '@/api/types'
import { useContract } from '@/context/ContractContext'
import { getNetworkPassphrase } from '@/lib/stellar'
import { cacheKeys } from '@/lib/cacheKeys'

function useClient() {
  const { config } = useContract()
  return new ScavengerClient({
    contractId: config.contractId,
    rpcUrl: config.rpcUrl,
    networkPassphrase: getNetworkPassphrase(config.network),
  })
}

export function useParticipant(address: string | undefined) {
  const client = useClient()
  return useQuery({
    queryKey: cacheKeys.participant(address ?? ''),
    queryFn: () => client.getParticipant(address!),
    enabled: !!address,
    staleTime: 60_000,
  })
}

export function useParticipantStats(address: string | undefined) {
  const client = useClient()
  return useQuery({
    queryKey: cacheKeys.participantStats(address ?? ''),
    queryFn: () => client.getStats(address!),
    enabled: !!address,
    staleTime: 60_000,
  })
}

export function useWaste(id: bigint | undefined) {
  const client = useClient()
  return useQuery({
    queryKey: cacheKeys.waste(id?.toString() ?? ''),
    queryFn: () => client.getWaste(id!),
    enabled: id !== undefined,
    staleTime: 30_000,
  })
}

export function useMetrics() {
  const client = useClient()
  return useQuery({
    queryKey: cacheKeys.metrics(),
    queryFn: () => client.getMetrics(),
    staleTime: 2 * 60_000,
  })
}

export function useActiveIncentives() {
  const client = useClient()
  return useQuery({
    queryKey: cacheKeys.activeIncentives(),
    queryFn: () => client.getActiveIncentives(),
    staleTime: 5 * 60_000,
  })
}

export function useIncentives(wasteType: WasteType | undefined) {
  const client = useClient()
  return useQuery({
    queryKey: cacheKeys.incentives(wasteType),
    queryFn: () => client.getIncentives(wasteType!),
    enabled: wasteType !== undefined,
    staleTime: 5 * 60_000,
  })
}

export function useSupplyChainStats() {
  const client = useClient()
  return useQuery({
    queryKey: cacheKeys.supplyChainStats(),
    queryFn: () => client.getSupplyChainStats(),
    staleTime: 2 * 60_000,
  })
}
