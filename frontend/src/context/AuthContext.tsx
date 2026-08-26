import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react'
import { useWallet } from '@/context/WalletContext'

interface Profile {
  role?: string
  name?: string
}

interface User extends Profile {
  address: string
}

interface AuthContextType {
  user: User | null
  isAuthenticated: boolean
  login: (profile: Profile) => void
  logout: () => void
  isLoading: boolean
}

const PROFILE_STORAGE_KEY = 'scavngr_profile'

const AuthContext = createContext<AuthContextType | undefined>(undefined)

export const AuthProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const { address, isLoading: walletLoading, disconnect } = useWallet()
  const [profile, setProfile] = useState<Profile | null>(null)

  // Profile is only meaningful for the currently connected wallet address.
  useEffect(() => {
    if (!address) {
      setProfile(null)
      return
    }
    const stored = localStorage.getItem(PROFILE_STORAGE_KEY)
    if (stored) {
      const parsed = JSON.parse(stored) as Profile & { address: string }
      setProfile(parsed.address === address ? { role: parsed.role, name: parsed.name } : null)
    }
  }, [address])

  const login = (newProfile: Profile) => {
    if (!address) return
    setProfile(newProfile)
    localStorage.setItem(PROFILE_STORAGE_KEY, JSON.stringify({ address, ...newProfile }))
  }

  const logout = () => {
    setProfile(null)
    localStorage.removeItem(PROFILE_STORAGE_KEY)
    disconnect()
  }

  const user = address ? { address, ...profile } : null

  return (
    <AuthContext.Provider
      value={{ user, isAuthenticated: !!address && !!profile, login, logout, isLoading: walletLoading }}
    >
      {children}
    </AuthContext.Provider>
  )
}

// eslint-disable-next-line react-refresh/only-export-components
export const useAuth = () => {
  const context = useContext(AuthContext)
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider')
  }
  return context
}
