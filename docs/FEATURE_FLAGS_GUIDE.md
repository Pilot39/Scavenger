# Feature Flag Guide

Scavngr uses a lightweight client-side feature flag system for gradual rollouts, A/B testing, and environment-specific features.

---

## Architecture

```
featureFlags.ts          ← Flag definitions, evaluation, overrides, analytics
  └── useFeatureFlags.ts ← React hooks
        └── FeatureFlagsPage.tsx ← Admin UI at /feature-flags
```

All flag state is stored in `localStorage`. There is no external flag service dependency — flags are defined in code and deployed with the app.

---

## Defining a Flag

Add your flag to the `FLAGS` object in `frontend/src/lib/featureFlags.ts`:

```ts
myNewFeature: {
  key: 'myNewFeature',
  description: 'Short description of what this flag controls',
  defaultValue: false,           // Value when no override is set
  rolloutPercentage: 25,         // Optional: % of users who see defaultValue=true
  environments: ['staging'],     // Optional: restrict to specific environments
  analyticsId: 'flag_my_feature', // Optional: analytics tracking key
},
```

**`environments`** accepts `'development' | 'staging' | 'production' | 'all'`.  
If omitted, the flag is active in all environments.

**`rolloutPercentage`** (0–100) is evaluated against a deterministic hash of the user ID, giving consistent results per user across sessions.

---

## Using Flags in Components

### Boolean flag check

```tsx
import { useFlag } from '@/hooks/useFeatureFlags'

function MyComponent() {
  const showNewUI = useFlag('newIncentiveUI')
  return showNewUI ? <NewUI /> : <LegacyUI />
}
```

### Flag value (string/number/boolean)

```tsx
import { useFeatureFlag } from '@/hooks/useFeatureFlags'

function MyComponent() {
  const theme = useFeatureFlag('themeVariant') // e.g. 'v2'
  return <div data-theme={String(theme)}>…</div>
}
```

### Imperative check (outside React)

```ts
import { isEnabled } from '@/lib/featureFlags'

if (isEnabled('carbonCreditIntegration')) {
  // …
}
```

---

## Flag Evaluation Order

1. **Environment check** — if the current environment is not in `environments`, the flag returns `false`
2. **Local override** — if a browser override exists (and has not expired), it wins
3. **Rollout** — if `rolloutPercentage` is set, evaluate the deterministic hash
4. **Default** — return `defaultValue`

---

## Local Overrides

The admin UI at `/feature-flags` lets any authenticated user toggle flags in their browser. Overrides are saved to `localStorage` under the key `scavngr_flag_overrides`.

Programmatically:

```ts
import { setFlagOverride, clearFlagOverride } from '@/lib/featureFlags'

setFlagOverride('batchWasteUpload', true)          // enable
setFlagOverride('batchWasteUpload', true, 3_600_000) // enable for 1 h then expire
clearFlagOverride('batchWasteUpload')              // remove override
```

---

## Analytics

Every override event is recorded to `localStorage` (key: `scavngr_flag_analytics`, last 500 events). The analytics panel in the `/feature-flags` UI shows recent events.

---

## Permanent Rollout Changes

Local overrides do not affect other users. To change a flag for everyone:

1. Update `defaultValue` or `rolloutPercentage` in `featureFlags.ts`
2. Commit and open a PR — CI will lint and type-check the change
3. Merge to `main` and deploy

---

## Current Flags

| Flag | Default | Rollout | Description |
|---|---|---|---|
| `healthDashboard` | `true` | 100% | Platform health & status dashboard |
| `performanceSLAs` | `true` | 100% | SLA monitoring and reporting |
| `batchWasteUpload` | `false` | 50% | CSV bulk waste upload |
| `newIncentiveUI` | `false` | 25% | Redesigned incentives marketplace |
| `predictiveAnalytics` | `false` | 10% | AI demand forecasting |
| `darkModeDefault` | `false` | 0% | Dark mode as default theme |
| `carbonCreditIntegration` | `false` | 5% | Carbon credit marketplace |
| `multiLanguageSupport` | `true` | 100% | Full i18n support |
