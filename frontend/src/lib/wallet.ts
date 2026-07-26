import {
  isConnected as freighterIsConnected,
  requestAccess,
  getPublicKey as freighterGetPublicKey,
  signTransaction as freighterSignTransaction,
  isBrowser,
} from '@stellar/freighter-api'

export interface WalletConnectionState {
  address: string | null
  isConnected: boolean
  isInstalled: boolean
  isLoading: boolean
  error: string | null
}

export const initialWalletState: WalletConnectionState = {
  address: null,
  isConnected: false,
  isInstalled: false,
  isLoading: false,
  error: null,
}

export async function checkWalletInstalled(): Promise<boolean> {
  if (!isBrowser) return false
  try {
    return await freighterIsConnected()
  } catch {
    return false
  }
}

export async function getWalletPublicKey(): Promise<string | null> {
  try {
    return await freighterGetPublicKey()
  } catch {
    return null
  }
}

export async function connectWallet(): Promise<string> {
  try {
    const address = await requestAccess()
    return address
  } catch (err: unknown) {
    const message = err instanceof Error ? err.message : String(err)
    throw new Error(
      message.includes('User declined')
        ? 'Connection rejected by user'
        : 'Failed to connect wallet'
    )
  }
}

export async function signTransactionXDR(
  transactionXDR: string,
  networkPassphrase: string
): Promise<string> {
  try {
    const signResult = await freighterSignTransaction(transactionXDR, {
      networkPassphrase,
    })
    return typeof signResult === 'string'
      ? signResult
      : (signResult as { signedTxXdr: string }).signedTxXdr
  } catch (err: unknown) {
    const message = err instanceof Error ? err.message : String(err)
    throw new Error(`Failed to sign transaction: ${message}`)
  }
}
