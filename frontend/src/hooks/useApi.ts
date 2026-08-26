/**
 * useApi — returns a ready-to-use ScavengerClient built from the current
 * ContractContext config and Stellar network settings.
 *
 * Usage:
 *   const api = useApi()
 *   const { data } = useQuery({ queryKey: ['participant', addr], queryFn: () => api.getParticipant(addr) })
 */
import { useMemo } from 'react'
import { ScavengerClient } from '@/api/client'
import { useContract } from '@/context/ContractContext'
import { networkConfig } from '@/lib/stellar'

export function useApi(): ScavengerClient {
  const { config } = useContract()
  return useMemo(
    () =>
      new ScavengerClient({
        rpcUrl: config.rpcUrl,
        networkPassphrase: networkConfig.networkPassphrase,
        contractId: config.contractId,
      }),
    [config.rpcUrl, config.contractId]
  )
}
