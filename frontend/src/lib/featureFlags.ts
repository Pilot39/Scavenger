/**
 * Feature Flag System
 * Supports gradual rollouts, A/B testing, and environment-specific flags.
 * Flags are persisted in localStorage with remote override support.
 */

export type FlagValue = boolean | string | number
export type FlagEnvironment = 'development' | 'staging' | 'production' | 'all'

export interface FeatureFlag {
  key: string
  description: string
  defaultValue: FlagValue
  /** Percentage of users (0-100) who get this flag enabled */
  rolloutPercentage?: number
  environments?: FlagEnvironment[]
  /** Optional analytics tracking ID */
  analyticsId?: string
}

export interface FlagOverride {
  key: string
  value: FlagValue
  expiresAt?: number // unix timestamp
}

export interface FlagAnalyticsEvent {
  flagKey: string
  value: FlagValue
  timestamp: number
  userId?: string
}

// ─── Built-in flag definitions ───────────────────────────────────────────────

export const FLAGS: Record<string, FeatureFlag> = {
  healthDashboard: {
    key: 'healthDashboard',
    description: 'Platform health & status dashboard',
    defaultValue: true,
    environments: ['all'],
    analyticsId: 'flag_health_dashboard',
  },
  performanceSLAs: {
    key: 'performanceSLAs',
    description: 'Performance SLA monitoring and reporting',
    defaultValue: true,
    environments: ['all'],
    analyticsId: 'flag_performance_slas',
  },
  batchWasteUpload: {
    key: 'batchWasteUpload',
    description: 'Enable batch CSV upload for waste submissions',
    defaultValue: false,
    rolloutPercentage: 50,
    environments: ['development', 'staging'],
    analyticsId: 'flag_batch_upload',
  },
  newIncentiveUI: {
    key: 'newIncentiveUI',
    description: 'Redesigned incentives marketplace UI',
    defaultValue: false,
    rolloutPercentage: 25,
    environments: ['all'],
    analyticsId: 'flag_new_incentive_ui',
  },
  predictiveAnalytics: {
    key: 'predictiveAnalytics',
    description: 'AI-powered predictive analytics features',
    defaultValue: false,
    rolloutPercentage: 10,
    environments: ['production'],
    analyticsId: 'flag_predictive_analytics',
  },
  darkModeDefault: {
    key: 'darkModeDefault',
    description: 'Enable dark mode as default theme',
    defaultValue: false,
    rolloutPercentage: 0,
    environments: ['all'],
    analyticsId: 'flag_dark_mode_default',
  },
  carbonCreditIntegration: {
    key: 'carbonCreditIntegration',
    description: 'Carbon credit marketplace integration',
    defaultValue: false,
    rolloutPercentage: 5,
    environments: ['staging', 'production'],
    analyticsId: 'flag_carbon_credits',
  },
  multiLanguageSupport: {
    key: 'multiLanguageSupport',
    description: 'Full multi-language (i18n) support',
    defaultValue: true,
    environments: ['all'],
    analyticsId: 'flag_i18n',
  },
}

// ─── Storage key ─────────────────────────────────────────────────────────────

const OVERRIDES_KEY = 'scavngr_flag_overrides'
const ANALYTICS_KEY = 'scavngr_flag_analytics'

// ─── Environment detection ────────────────────────────────────────────────────

function getCurrentEnvironment(): FlagEnvironment {
  const host = typeof window !== 'undefined' ? window.location.hostname : ''
  if (host === 'localhost' || host === '127.0.0.1') return 'development'
  if (host.includes('staging')) return 'staging'
  return 'production'
}

// ─── Rollout evaluation ───────────────────────────────────────────────────────

function isInRollout(percentage: number, userId?: string): boolean {
  if (percentage >= 100) return true
  if (percentage <= 0) return false
  // Deterministic hash of userId for consistent rollout
  const seed = userId ?? localStorage.getItem('scavngr_user_id') ?? 'anonymous'
  let hash = 0
  for (let i = 0; i < seed.length; i++) {
    hash = (hash * 31 + seed.charCodeAt(i)) >>> 0
  }
  return (hash % 100) < percentage
}

// ─── Core evaluation ─────────────────────────────────────────────────────────

export function evaluateFlag(flagKey: string, userId?: string): FlagValue {
  const flag = FLAGS[flagKey]
  if (!flag) return false

  // Check environment
  const env = getCurrentEnvironment()
  if (flag.environments && !flag.environments.includes('all') && !flag.environments.includes(env)) {
    return false
  }

  // Check for user override
  const overrides = getOverrides()
  const override = overrides.find((o) => o.key === flagKey)
  if (override) {
    if (override.expiresAt && Date.now() > override.expiresAt) {
      // Expired override – remove it
      setOverrides(overrides.filter((o) => o.key !== flagKey))
    } else {
      return override.value
    }
  }

  // Rollout check
  if (flag.rolloutPercentage !== undefined) {
    return isInRollout(flag.rolloutPercentage, userId) ? flag.defaultValue : false
  }

  return flag.defaultValue
}

export function isEnabled(flagKey: string, userId?: string): boolean {
  return Boolean(evaluateFlag(flagKey, userId))
}

// ─── Override management ──────────────────────────────────────────────────────

function getOverrides(): FlagOverride[] {
  try {
    return JSON.parse(localStorage.getItem(OVERRIDES_KEY) ?? '[]')
  } catch {
    return []
  }
}

function setOverrides(overrides: FlagOverride[]): void {
  localStorage.setItem(OVERRIDES_KEY, JSON.stringify(overrides))
}

export function setFlagOverride(key: string, value: FlagValue, ttlMs?: number): void {
  const overrides = getOverrides().filter((o) => o.key !== key)
  overrides.push({ key, value, expiresAt: ttlMs ? Date.now() + ttlMs : undefined })
  setOverrides(overrides)
  trackFlagEvaluation(key, value)
}

export function clearFlagOverride(key: string): void {
  setOverrides(getOverrides().filter((o) => o.key !== key))
}

export function clearAllOverrides(): void {
  localStorage.removeItem(OVERRIDES_KEY)
}

export function getAllFlagOverrides(): FlagOverride[] {
  return getOverrides().filter((o) => !o.expiresAt || Date.now() <= o.expiresAt)
}

// ─── Analytics ────────────────────────────────────────────────────────────────

function trackFlagEvaluation(flagKey: string, value: FlagValue): void {
  try {
    const events: FlagAnalyticsEvent[] = JSON.parse(
      localStorage.getItem(ANALYTICS_KEY) ?? '[]'
    )
    events.push({ flagKey, value, timestamp: Date.now() })
    // Keep only the last 500 events
    localStorage.setItem(ANALYTICS_KEY, JSON.stringify(events.slice(-500)))
  } catch {
    // non-critical
  }
}

export function getFlagAnalytics(): FlagAnalyticsEvent[] {
  try {
    return JSON.parse(localStorage.getItem(ANALYTICS_KEY) ?? '[]')
  } catch {
    return []
  }
}

export function clearFlagAnalytics(): void {
  localStorage.removeItem(ANALYTICS_KEY)
}

// ─── Snapshot of all current values ──────────────────────────────────────────

export function getAllFlagValues(userId?: string): Record<string, FlagValue> {
  return Object.fromEntries(
    Object.keys(FLAGS).map((key) => [key, evaluateFlag(key, userId)])
  )
}
