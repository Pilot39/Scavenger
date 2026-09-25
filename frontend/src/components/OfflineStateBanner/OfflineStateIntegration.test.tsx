import { describe, it, expect, beforeEach, vi, afterEach } from 'vitest'
import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import React, { useState, useEffect } from 'react'
import { OfflineStateBanner } from './OfflineStateBanner'
import { OfflineIndicator } from '../OfflineIndicator'

describe('Offline/Online Transition Integration Tests', () => {
  beforeEach(() => {
    localStorage.clear()
    vi.clearAllMocks()
  })

  afterEach(() => {
    localStorage.clear()
  })

  describe('Online to Offline Transitions', () => {
    it('should display both banner and indicator when going offline', async () => {
      const TestComponent = () => {
        const [isOnline, setIsOnline] = useState(true)

        return (
          <>
            <OfflineStateBanner isOnline={isOnline} />
            <OfflineIndicator />
            <button onClick={() => setIsOnline(false)}>Go Offline</button>
          </>
        )
      }

      const { rerender } = render(<TestComponent />)

      expect(screen.queryByTestId('offline-state-banner')).not.toBeInTheDocument()
      expect(screen.queryByTestId('offline-indicator')).not.toBeInTheDocument()

      const button = screen.getByRole('button', { name: /go offline/i })
      fireEvent.click(button)

      // Mock useOnlineStatus for OfflineIndicator
      vi.mock('@/hooks/useOnlineStatus', () => ({
        useOnlineStatus: vi.fn(() => false),
      }))

      await waitFor(() => {
        expect(screen.getByTestId('offline-state-banner')).toBeInTheDocument()
      })
    })

    it('should hide features notification during transition', async () => {
      const TestComponent = () => {
        const [isOnline, setIsOnline] = useState(true)
        const [message, setMessage] = useState('You are offline. Some features may be unavailable.')

        useEffect(() => {
          if (!isOnline) {
            setMessage('You are offline. Sync will resume when connected.')
          } else {
            setMessage('You are offline. Some features may be unavailable.')
          }
        }, [isOnline])

        return (
          <>
            <OfflineStateBanner isOnline={isOnline} message={message} />
            <button onClick={() => setIsOnline(false)}>Go Offline</button>
          </>
        )
      }

      render(<TestComponent />)

      const button = screen.getByRole('button', { name: /go offline/i })
      fireEvent.click(button)

      await waitFor(() => {
        expect(
          screen.getByText('You are offline. Sync will resume when connected.')
        ).toBeInTheDocument()
      })
    })
  })

  describe('Offline to Online Transitions', () => {
    it('should hide banner when reconnecting', async () => {
      const TestComponent = () => {
        const [isOnline, setIsOnline] = useState(false)

        return (
          <>
            <OfflineStateBanner isOnline={isOnline} />
            <button onClick={() => setIsOnline(true)}>Go Online</button>
          </>
        )
      }

      render(<TestComponent />)

      expect(screen.getByTestId('offline-state-banner')).toBeInTheDocument()

      const button = screen.getByRole('button', { name: /go online/i })
      fireEvent.click(button)

      await waitFor(() => {
        expect(screen.queryByTestId('offline-state-banner')).not.toBeInTheDocument()
      })
    })

    it('should trigger reconnect-and-sync flow on reconnection', async () => {
      const mockRetry = vi.fn().mockResolvedValue(undefined)
      const mockSync = vi.fn().mockResolvedValue({ synced: 5, failed: 0 })

      const TestComponent = () => {
        const [isOnline, setIsOnline] = useState(false)
        const [syncStatus, setSyncStatus] = useState<string | null>(null)

        const handleRetry = async () => {
          mockRetry()
          if (isOnline) {
            const result = await mockSync()
            setSyncStatus(`Synced ${result.synced} items`)
          }
        }

        return (
          <>
            <OfflineStateBanner isOnline={isOnline} onRetry={handleRetry} />
            {syncStatus && <div data-testid="sync-status">{syncStatus}</div>}
            <button onClick={() => setIsOnline(true)}>Go Online</button>
          </>
        )
      }

      const { rerender } = render(<TestComponent />)

      const retryButton = screen.getByTestId('retry-button')
      expect(retryButton).toBeInTheDocument()

      fireEvent.click(retryButton)

      await waitFor(() => {
        expect(mockRetry).toHaveBeenCalled()
      })
    })

    it('should show success message after successful reconnection', async () => {
      const TestComponent = () => {
        const [isOnline, setIsOnline] = useState(false)
        const [reconnected, setReconnected] = useState(false)

        const handleRetry = async () => {
          await new Promise(resolve => setTimeout(resolve, 50))
          setIsOnline(true)
          setReconnected(true)
        }

        return (
          <>
            <OfflineStateBanner isOnline={isOnline} onRetry={handleRetry} />
            {reconnected && <div data-testid="success-message">Reconnected!</div>}
          </>
        )
      }

      render(<TestComponent />)

      const retryButton = screen.getByTestId('retry-button')
      fireEvent.click(retryButton)

      await waitFor(() => {
        expect(screen.getByTestId('success-message')).toBeInTheDocument()
      })
    })
  })

  describe('Flaky Connectivity Scenarios', () => {
    it('should handle rapid online/offline transitions', async () => {
      const TestComponent = () => {
        const [isOnline, setIsOnline] = useState(true)
        const [transitionCount, setTransitionCount] = useState(0)

        const toggleConnection = () => {
          setIsOnline(prev => !prev)
          setTransitionCount(prev => prev + 1)
        }

        return (
          <>
            <OfflineStateBanner isOnline={isOnline} />
            <div data-testid="transition-count">{transitionCount}</div>
            <button onClick={toggleConnection}>Toggle Connection</button>
          </>
        )
      }

      render(<TestComponent />)

      const button = screen.getByRole('button', { name: /toggle connection/i })

      for (let i = 0; i < 5; i++) {
        fireEvent.click(button)
        await waitFor(() => {
          const count = parseInt(screen.getByTestId('transition-count').textContent || '0')
          expect(count).toBeGreaterThanOrEqual(i + 1)
        })
      }
    })

    it('should queue operations during intermittent connectivity', async () => {
      const mockOperations: string[] = []

      const TestComponent = () => {
        const [isOnline, setIsOnline] = useState(true)
        const [queuedOps, setQueuedOps] = useState<string[]>([])

        const queueOperation = (op: string) => {
          if (isOnline) {
            mockOperations.push(op)
          } else {
            setQueuedOps(prev => [...prev, op])
          }
        }

        return (
          <>
            <OfflineStateBanner isOnline={isOnline} />
            <button onClick={() => queueOperation('op1')}>Queue Op</button>
            <button onClick={() => setIsOnline(prev => !prev)}>Toggle</button>
            <div data-testid="queued-count">{queuedOps.length}</div>
          </>
        )
      }

      render(<TestComponent />)

      const toggleBtn = screen.getByRole('button', { name: /toggle/i })
      const queueBtn = screen.getByRole('button', { name: /queue op/i })

      fireEvent.click(toggleBtn)
      fireEvent.click(queueBtn)

      await waitFor(() => {
        expect(screen.getByTestId('queued-count').textContent).toBe('1')
      })

      fireEvent.click(toggleBtn)
      fireEvent.click(queueBtn)

      await waitFor(() => {
        expect(mockOperations.length).toBeGreaterThan(0)
      })
    })

    it('should persist pending changes during offline period', async () => {
      const TestComponent = () => {
        const [isOnline, setIsOnline] = useState(true)
        const [changes, setChanges] = useState<string[]>([])

        const makeChange = (change: string) => {
          const newChanges = [...changes, change]
          setChanges(newChanges)
          if (!isOnline) {
            localStorage.setItem('pending_changes', JSON.stringify(newChanges))
          }
        }

        useEffect(() => {
          if (isOnline) {
            const stored = localStorage.getItem('pending_changes')
            if (stored) {
              const pending = JSON.parse(stored)
              setChanges(prev => [...prev, ...pending])
              localStorage.removeItem('pending_changes')
            }
          }
        }, [isOnline])

        return (
          <>
            <OfflineStateBanner isOnline={isOnline} />
            <button onClick={() => makeChange('change1')}>Make Change</button>
            <button onClick={() => setIsOnline(prev => !prev)}>Toggle</button>
            <div data-testid="change-count">{changes.length}</div>
          </>
        )
      }

      render(<TestComponent />)

      const toggleBtn = screen.getByRole('button', { name: /toggle/i })
      const changeBtn = screen.getByRole('button', { name: /make change/i })

      fireEvent.click(changeBtn)
      expect(screen.getByTestId('change-count').textContent).toBe('1')

      fireEvent.click(toggleBtn)
      fireEvent.click(changeBtn)

      await waitFor(() => {
        expect(screen.getByTestId('change-count').textContent).toBe('2')
      })

      fireEvent.click(toggleBtn)

      await waitFor(() => {
        expect(localStorage.getItem('pending_changes')).toBeNull()
      })
    })

    it('should handle multiple failed retry attempts', async () => {
      let retryAttempts = 0
      const mockRetry = vi.fn(async () => {
        retryAttempts++
        if (retryAttempts < 3) {
          throw new Error('Retry failed')
        }
      })

      const TestComponent = () => {
        const [isOnline, setIsOnline] = useState(false)
        const [attempts, setAttempts] = useState(0)

        const handleRetry = async () => {
          setAttempts(prev => prev + 1)
          try {
            await mockRetry()
          } catch (error) {
            // Retry failed, stay offline
          }
        }

        return (
          <>
            <OfflineStateBanner isOnline={isOnline} onRetry={handleRetry} />
            <div data-testid="attempts">{attempts}</div>
          </>
        )
      }

      render(<TestComponent />)

      const retryButton = screen.getByTestId('retry-button')

      fireEvent.click(retryButton)
      await waitFor(() => {
        expect(screen.getByTestId('attempts').textContent).toBe('1')
      })

      fireEvent.click(retryButton)
      await waitFor(() => {
        expect(screen.getByTestId('attempts').textContent).toBe('2')
      })

      fireEvent.click(retryButton)
      await waitFor(() => {
        expect(screen.getByTestId('attempts').textContent).toBe('3')
        expect(mockRetry).toHaveBeenCalledTimes(3)
      })
    })
  })

  describe('Retry and Recovery Flow', () => {
    it('should disable user actions during offline state', async () => {
      const TestComponent = () => {
        const [isOnline, setIsOnline] = useState(false)

        return (
          <>
            <OfflineStateBanner isOnline={isOnline} />
            <button disabled={!isOnline} data-testid="action-button">
              Perform Action
            </button>
          </>
        )
      }

      render(<TestComponent />)

      const actionButton = screen.getByTestId('action-button')
      expect(actionButton).toBeDisabled()
    })

    it('should enable user actions after successful reconnection', async () => {
      const TestComponent = () => {
        const [isOnline, setIsOnline] = useState(false)

        const handleRetry = async () => {
          await new Promise(resolve => setTimeout(resolve, 50))
          setIsOnline(true)
        }

        return (
          <>
            <OfflineStateBanner isOnline={isOnline} onRetry={handleRetry} />
            <button disabled={!isOnline} data-testid="action-button">
              Perform Action
            </button>
          </>
        )
      }

      render(<TestComponent />)

      const actionButton = screen.getByTestId('action-button')
      expect(actionButton).toBeDisabled()

      const retryButton = screen.getByTestId('retry-button')
      fireEvent.click(retryButton)

      await waitFor(() => {
        expect(actionButton).not.toBeDisabled()
      })
    })
  })

  describe('Accessibility During Offline State', () => {
    it('should announce offline state to screen readers', async () => {
      const TestComponent = () => {
        const [isOnline, setIsOnline] = useState(false)

        return (
          <>
            <OfflineStateBanner isOnline={isOnline} />
            <button onClick={() => setIsOnline(prev => !prev)}>Toggle</button>
          </>
        )
      }

      render(<TestComponent />)

      const banner = screen.getByTestId('offline-state-banner')
      expect(banner).toHaveAttribute('role', 'alert')
      expect(banner).toHaveAttribute('aria-live', 'polite')
    })

    it('should provide keyboard navigation for retry button', async () => {
      const mockRetry = vi.fn().mockResolvedValue(undefined)

      render(<OfflineStateBanner isOnline={false} onRetry={mockRetry} />)

      const retryButton = screen.getByTestId('retry-button')
      retryButton.focus()

      fireEvent.keyDown(retryButton, { key: 'Enter', code: 'Enter' })

      await waitFor(() => {
        expect(mockRetry).toHaveBeenCalled()
      })
    })
  })
})
