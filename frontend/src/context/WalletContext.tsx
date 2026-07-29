/**
 * context/WalletContext.tsx
 *
 * Compatibility shim — delegates to the wallet store slice.
 * Existing components that call useWallet() continue to work unchanged.
 *
 * The actual state and dispatch live in StoreProvider (store/index.tsx).
 * Freighter API calls are handled via lib/wallet.ts service.
 */
import React, { useEffect, type ReactNode } from 'react'
import { isBrowser } from '@stellar/freighter-api'
import { useWalletStore } from '@/store'
import {
  checkWalletInstalled,
  getWalletPublicKey,
  connectWallet,
} from '@/lib/wallet'

// ── Type kept for import compatibility ───────────────────────────
export interface WalletContextType {
  address: string | null
  isConnected: boolean
  isInstalled: boolean
  connect: () => Promise<void>
  disconnect: () => void
  isLoading: boolean
  error: string | null
}

/**
 * WalletProvider — initialises the Freighter extension connection and
 * writes results into the wallet store slice.
 */
export const WalletProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const { dispatch } = useWalletStore()

  useEffect(() => {
    if (!isBrowser) {
      dispatch({ type: 'WALLET_READY' })
      return
    }
    ;(async () => {
      try {
        const isInstalled = await checkWalletInstalled()
        dispatch({ type: 'WALLET_INSTALLED', payload: isInstalled })
        if (isInstalled) {
          const addr = await getWalletPublicKey()
          if (addr) dispatch({ type: 'WALLET_CONNECTED', payload: addr })
          else dispatch({ type: 'WALLET_READY' })
        } else {
          dispatch({ type: 'WALLET_READY' })
        }
      } catch {
        dispatch({ type: 'WALLET_INSTALLED', payload: false })
        dispatch({ type: 'WALLET_READY' })
      }
    })()
  }, [dispatch])

  return <>{children}</>
}

/**
 * useWallet — thin adapter over the wallet store slice.
 * Returns the same shape as the original WalletContext.
 */
// eslint-disable-next-line react-refresh/only-export-components
export function useWallet(): WalletContextType {
  const { state, dispatch } = useWalletStore()

  const connect = async () => {
    dispatch({ type: 'WALLET_LOADING' })
    if (!state.isInstalled) {
      dispatch({
        type: 'WALLET_ERROR',
        payload: 'Freighter extension is not installed. Please install it from freighter.app.',
      })
      return
    }
    try {
      const addr = await connectWallet()
      dispatch({ type: 'WALLET_CONNECTED', payload: addr })
    } catch (err: unknown) {
      const msg = err instanceof Error ? err.message : String(err)
      dispatch({
        type: 'WALLET_ERROR',
        payload: msg,
      })
    }
  }

  const disconnect = () => {
    dispatch({ type: 'WALLET_DISCONNECTED' })
  }

  return {
    address: state.address,
    isConnected: state.isConnected,
    isInstalled: state.isInstalled,
    isLoading: state.isLoading,
    error: state.error,
    connect,
    disconnect,
  }
}
