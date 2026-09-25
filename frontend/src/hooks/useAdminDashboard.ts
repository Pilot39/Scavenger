import { useQuery } from '@tanstack/react-query'
import { ScavengerClient } from '@/api/client'
import { useContract } from '@/context/ContractContext'
import { networkConfig } from '@/lib/stellar'
import type { GlobalMetrics, Incentive } from '@/api/types'

function useAdminClient() {
  const { config } = useContract()
  return () =>
    new ScavengerClient({
      rpcUrl: config.rpcUrl,
      networkPassphrase: networkConfig.networkPassphrase,
      contractId: config.contractId,
    })
}

export function useAdminMetrics() {
  const makeClient = useAdminClient()
  return useQuery<GlobalMetrics>({
    queryKey: ['admin-metrics'],
    queryFn: async () => makeClient().getMetrics(),
    staleTime: 30_000,
  })
}

export function useAdminIncentives() {
  const makeClient = useAdminClient()
  return useQuery<Incentive[]>({
    queryKey: ['admin-incentives'],
    queryFn: async () => makeClient().getActiveIncentives(),
    staleTime: 30_000,
  })
}

export function useAdminWasteLookup(wasteId: bigint | null) {
  const makeClient = useAdminClient()
  return useQuery({
    queryKey: ['admin-waste', wasteId?.toString()],
    queryFn: async () => makeClient().getWaste(wasteId!),
    enabled: wasteId !== null,
  })
}
