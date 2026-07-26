import { describe, it, expect, beforeEach, vi } from 'vitest'
import {
  evaluateFlag,
  isEnabled,
  setFlagOverride,
  clearFlagOverride,
  clearAllOverrides,
  getAllFlagValues,
  getAllFlagOverrides,
  getFlagAnalytics,
  clearFlagAnalytics,
  FLAGS,
} from '@/lib/featureFlags'

// Mock localStorage
const localStorageMock = (() => {
  let store: Record<string, string> = {}
  return {
    getItem: (key: string) => store[key] ?? null,
    setItem: (key: string, value: string) => { store[key] = value },
    removeItem: (key: string) => { delete store[key] },
    clear: () => { store = {} },
  }
})()

Object.defineProperty(globalThis, 'localStorage', { value: localStorageMock })
Object.defineProperty(globalThis, 'window', { value: { location: { hostname: 'localhost' } }, writable: true })

describe('featureFlags', () => {
  beforeEach(() => {
    localStorageMock.clear()
  })

  it('evaluates a flag with defaultValue=true', () => {
    expect(evaluateFlag('healthDashboard')).toBe(true)
  })

  it('evaluates a flag with defaultValue=false', () => {
    expect(evaluateFlag('batchWasteUpload')).toBe(false)
  })

  it('returns false for unknown flag', () => {
    expect(evaluateFlag('unknownFlag')).toBe(false)
  })

  it('isEnabled wraps evaluateFlag as boolean', () => {
    expect(isEnabled('healthDashboard')).toBe(true)
    expect(isEnabled('unknownFlag')).toBe(false)
  })

  it('setFlagOverride overrides default value', () => {
    setFlagOverride('batchWasteUpload', true)
    expect(isEnabled('batchWasteUpload')).toBe(true)
  })

  it('clearFlagOverride restores default', () => {
    setFlagOverride('batchWasteUpload', true)
    clearFlagOverride('batchWasteUpload')
    expect(isEnabled('batchWasteUpload')).toBe(false)
  })

  it('clearAllOverrides removes all overrides', () => {
    setFlagOverride('healthDashboard', false)
    setFlagOverride('batchWasteUpload', true)
    clearAllOverrides()
    expect(getAllFlagOverrides()).toHaveLength(0)
  })

  it('expired overrides are ignored', () => {
    setFlagOverride('batchWasteUpload', true, -1000) // already expired
    expect(isEnabled('batchWasteUpload')).toBe(false)
  })

  it('getAllFlagValues returns all flags', () => {
    const all = getAllFlagValues()
    expect(Object.keys(all)).toEqual(expect.arrayContaining(Object.keys(FLAGS)))
  })

  it('setFlagOverride records analytics event', () => {
    clearFlagAnalytics()
    setFlagOverride('newIncentiveUI', true)
    const events = getFlagAnalytics()
    expect(events.some((e) => e.flagKey === 'newIncentiveUI' && e.value === true)).toBe(true)
  })

  it('getAllFlagOverrides lists active overrides', () => {
    setFlagOverride('darkModeDefault', true)
    const overrides = getAllFlagOverrides()
    expect(overrides.some((o) => o.key === 'darkModeDefault')).toBe(true)
  })
})
