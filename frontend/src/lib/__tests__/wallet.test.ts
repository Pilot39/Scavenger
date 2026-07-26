import { describe, it, expect, vi, beforeEach } from 'vitest'
import * as walletService from '../wallet'
import * as freighterApi from '@stellar/freighter-api'

vi.mock('@stellar/freighter-api', () => ({
  isConnected: vi.fn(),
  requestAccess: vi.fn(),
  getPublicKey: vi.fn(),
  signTransaction: vi.fn(),
  isBrowser: true,
}))

describe('wallet service', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  describe('checkWalletInstalled', () => {
    it('should return true if wallet is installed', async () => {
      vi.mocked(freighterApi.isConnected).mockResolvedValue(true)
      const result = await walletService.checkWalletInstalled()
      expect(result).toBe(true)
    })

    it('should return false if wallet is not installed', async () => {
      vi.mocked(freighterApi.isConnected).mockRejectedValue(new Error('Not installed'))
      const result = await walletService.checkWalletInstalled()
      expect(result).toBe(false)
    })

    it('should return false on non-browser environment', async () => {
      Object.defineProperty(freighterApi, 'isBrowser', { value: false })
      const result = await walletService.checkWalletInstalled()
      expect(result).toBe(false)
    })
  })

  describe('getWalletPublicKey', () => {
    it('should return public key if available', async () => {
      const mockKey = 'GXYZABC123'
      vi.mocked(freighterApi.getPublicKey).mockResolvedValue(mockKey)
      const result = await walletService.getWalletPublicKey()
      expect(result).toBe(mockKey)
    })

    it('should return null if error occurs', async () => {
      vi.mocked(freighterApi.getPublicKey).mockRejectedValue(new Error('Failed'))
      const result = await walletService.getWalletPublicKey()
      expect(result).toBeNull()
    })
  })

  describe('connectWallet', () => {
    it('should return address on successful connection', async () => {
      const mockAddress = 'GXYZABC123'
      vi.mocked(freighterApi.requestAccess).mockResolvedValue(mockAddress)
      const result = await walletService.connectWallet()
      expect(result).toBe(mockAddress)
    })

    it('should throw error with user-declined message', async () => {
      vi.mocked(freighterApi.requestAccess).mockRejectedValue(
        new Error('User declined')
      )
      await expect(walletService.connectWallet()).rejects.toThrow(
        'Connection rejected by user'
      )
    })

    it('should throw error on connection failure', async () => {
      vi.mocked(freighterApi.requestAccess).mockRejectedValue(
        new Error('Network error')
      )
      await expect(walletService.connectWallet()).rejects.toThrow(
        'Failed to connect wallet'
      )
    })
  })

  describe('signTransactionXDR', () => {
    it('should sign transaction and return XDR string', async () => {
      const txXdr = 'tx_xdr_string'
      const passphrase = 'Test SDF Network ; September 2015'
      const signedXdr = 'signed_tx_xdr_string'

      vi.mocked(freighterApi.signTransaction).mockResolvedValue(signedXdr)
      const result = await walletService.signTransactionXDR(txXdr, passphrase)

      expect(result).toBe(signedXdr)
      expect(freighterApi.signTransaction).toHaveBeenCalledWith(txXdr, {
        networkPassphrase: passphrase,
      })
    })

    it('should handle signTransaction response object', async () => {
      const txXdr = 'tx_xdr_string'
      const passphrase = 'Test SDF Network ; September 2015'
      const signedXdr = 'signed_tx_xdr_string'

      vi.mocked(freighterApi.signTransaction).mockResolvedValue({
        signedTxXdr: signedXdr,
      })
      const result = await walletService.signTransactionXDR(txXdr, passphrase)

      expect(result).toBe(signedXdr)
    })

    it('should throw error on signing failure', async () => {
      vi.mocked(freighterApi.signTransaction).mockRejectedValue(
        new Error('Sign failed')
      )
      await expect(
        walletService.signTransactionXDR('tx', 'passphrase')
      ).rejects.toThrow('Failed to sign transaction: Sign failed')
    })
  })
})
