# Changelog

All notable changes to Scavngr are documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Stellar RPC client module** (#907) — Centralised Horizon/Soroban access with retry and exponential backoff
  - `backend/src/rpc/client.rs` — `StellarRpcClient` with `get_account`, `get_latest_ledger`, `submit_transaction`, `get_ledger_entries`, `simulate_transaction`
  - `backend/src/rpc/mod.rs` — Module entry point; re-exports all public types
  - `RetryConfig` with full-jitter exponential backoff, configurable max attempts and delay cap
  - All Stellar env vars (`STELLAR_HORIZON_URL`, `SOROBAN_RPC_URL`, `CONTRACT_ID`, etc.) read via `StellarRpcConfig::from_env()`
  - Client registered as `Arc<StellarRpcClient>` app data; handlers can inject it via `web::Data`
- **Per-endpoint cache TTL configuration** (#908) — Hot reads now expire at the appropriate rate
  - `backend/src/cache/ttl.rs` — `CacheTtl` enum with named presets: `WasteItem` (30 s), `WasteList` / `ParticipantItem` / `ParticipantList` (60 s), `ContractStats` (2 min), `ContractInfo` (5 min)
  - `cache_keys` helpers centralise cache key strings to prevent typos
  - `CacheInvalidationManager` wired as `Arc` app data; `invalidate_waste_cache` and `invalidate_all_cache` now apply structured invalidation strategies
  - New `GET /api/v1/cache/metrics` endpoint exposes hit/miss/eviction counters and current cache size
- **Per-route rate limiting** (#909) — Public read-heavy routes now have stricter limits
  - `RouteRateLimitConfig` — associates a URL path prefix with a `RateLimitTier`
  - `RateLimitLayer` builder for ergonomic middleware composition
  - `/api/v1/contracts/` and `/api/v1/search` throttled to Anonymous tier (30 RPM / 200 RPH)
  - All 429 responses now include `Retry-After` header (seconds until window resets)
  - Per-route + per-IP sliding window ensures limits are independent across route groups

### Changed
- **Cache layer** (#908) — `contracts.rs` handlers use `set_with_ttl` with typed `CacheTtl` constants instead of the default TTL
- **Rate limiting** (#909) — `RateLimitMiddlewareService` requires `S: Clone` (standard actix-web pattern); 429 body is now JSON with `error`, `message`, and `retry_after_seconds` fields

### Removed
- **Legacy analytics API endpoints** (#906) — `/analytics/participant/{id}`, `/analytics/global`, `/analytics/metrics/{name}` were never registered in `main.rs` and had no consumers; `api/analytics.rs` content cleared and module removed from `api/mod.rs`
- **Unused service exports** (#906) — `RecommendationEngine`, `NFTManager`, `ChainAbstraction` removed from `services/mod.rs` public re-exports (source files retained for future use)

### Added
- **Feature Flag System** (#786) — Gradual rollout and A/B testing infrastructure
  - `frontend/src/lib/featureFlags.ts` — Flag definitions, evaluation logic, rollout by user hash, TTL-aware overrides, analytics tracking
  - `frontend/src/hooks/useFeatureFlags.ts` — React hooks (`useFlag`, `useFeatureFlag`, `useFeatureFlags`)
  - `frontend/src/pages/FeatureFlagsPage.tsx` — Admin UI with toggle controls and event log
  - `frontend/src/lib/featureFlags.test.ts` — Unit tests for all flag operations
  - Route: `/feature-flags`
- **Platform Health Dashboard** (#788) — Real-time service status and incident tracking
  - `frontend/src/lib/healthMonitoring.ts` — Service definitions, status helpers, health history, incident model
  - `frontend/src/pages/PlatformHealthDashboardPage.tsx` — Dashboard with grouped services, incident history, auto-refresh
  - Route: `/health`
- **Performance SLAs** (#789) — Defined SLA targets with compliance monitoring
  - `frontend/src/lib/performanceSLAs.ts` — 11 SLA targets across availability, latency, web vitals, error rate, and throughput
  - `frontend/src/pages/PerformanceSLAsPage.tsx` — SLA dashboard with compliance bars, category filtering, violation procedures
  - `docs/PERFORMANCE_SLA_GUIDE.md` — Full SLA documentation with metrics, targets, and response playbooks
  - Route: `/slas`

---

## [0.11.0] — 2026-06-20

### Added
- Batch upload page for bulk waste submission via CSV (#785)
- Compliance reports page with downloadable PDF exports (#784)
- Test reports page with CI integration summary (#783)

### Changed
- Performance monitoring page now includes Web Vitals history chart (#782)
- Notification center supports push notification opt-in (#781)

### Fixed
- WasteVerificationDashboard pagination reset on filter change (#780)
- IncentiveMarketplace infinite scroll edge case with empty result sets (#779)

---

## [0.10.0] — 2026-06-06

### Added
- Gamification page with achievements, leaderboard, and challenges (#778)
- Offline mode with service-worker caching strategy (#777)
- Waste history page with timeline visualisation (#776)

### Changed
- Supply chain tracker upgraded to interactive SVG map (#775)
- Analytics page includes carbon impact estimation widget (#774)

### Fixed
- Auth token refresh race condition on simultaneous tab sessions (#773)
- Mobile layout overflow in RecyclerDashboard stats grid (#772)

---

## [0.9.0] — 2026-05-23

### Added
- Predictive analytics page with demand forecasting (#771)
- Waste comparison tool for cross-participant benchmarking (#770)
- Messaging/chat page for participant communication (#769)
- Route planner page for collection route optimisation (#768)

### Changed
- Wallet context now supports Freighter and LOBSTR wallet adapters (#767)
- Admin dashboard includes participant management table (#766)

### Security
- Added CSRF protection middleware to all mutating backend routes (#765)
- Session tokens now use HttpOnly + Secure + SameSite=Strict cookies (#764)

---

## [0.8.0] — 2026-05-09

### Added
- Waste certification page with verifiable credential generation (#763)
- Waste marketplace page for listing and purchasing waste lots (#762)
- Recycling guide page with material-type documentation (#761)
- Performance monitoring page with Core Web Vitals tracking (#760)

### Changed
- Smart contract `distribute_rewards` now supports multi-leg supply chains (#759)
- Indexer query optimiser reduces average query time by ~40% (#758)

### Fixed
- Contract `get_incentives` sort order instability for equal-reward entries (#757)
- Frontend `.env` validation now provides actionable error messages (#756)

---

## [0.7.0] — 2026-04-25

### Added
- Participant search page with geo-proximity filtering (#755)
- Waste statistics page with per-type breakdowns (#754)
- Reward tracking page with token vesting timeline (#753)
- Waste verification dashboard with bulk approve/reject (#752)
- Material transfer page with QR-code handoff flow (#751)

### Changed
- AppShell sidebar now collapses on mobile (#750)
- Role-based route protection added to all dashboard routes (#749)

---

## [0.6.0] — 2026-04-11

### Added
- Waste map page with Mapbox integration (#748)
- Community page with activity feed and participant profiles (#747)
- Analytics page with Recharts visualisations (#746)

### Changed
- RecyclerDashboard summary cards use new StatCard design-system component (#745)

### Deprecated
- Legacy `get_waste` v1 contract function — use `get_material` going forward (#744)

---

## [0.5.0] — 2026-03-28

### Added
- Soroban smart contract: `split_waste`, `merge_wastes`, `batch_deactivate_waste` functions (#743)
- Soroban smart contract: reputation system and participant scoring (#742)
- Soroban smart contract: waste tags and max-weight validation (#741)
- Backend: audit log service with tamper-evident event chain (#740)
- Backend: export service (CSV / JSON / PDF) for participant data (#739)

### Security
- RustSec audit run; all advisories resolved (#738)
- Dependency pinning enforced in all `package.json` and `Cargo.toml` files (#737)

---

## [0.4.0] — 2026-03-14

### Added
- Incentive marketplace: manufacturer incentive tiers and scheduling (#736)
- Soroban contract: `get_incentives_by_rewarder` query (#735)
- Indexer: Redis caching layer with TTL-based invalidation (#734)
- Indexer: rate limiting middleware (#733)
- k8s: horizontal pod autoscaler for backend and indexer deployments (#732)

### Changed
- Frontend migrated from Create React App to Vite (#731)
- Contract event emitters refactored into `events.rs` module (#730)

### Fixed
- `distribute_rewards` failed silently when charity address was not set (#729)
- Indexer dropped events during Stellar network reconnect (#728)

---

## [0.3.0] — 2026-02-28

### Added
- Soroban contract: waste confirmation and reset workflow (#557, #558)
- Soroban contract: transfer history with per-waste chain (#559, #560)
- Backend: WebSocket real-time event stream for frontend (#561)
- Frontend: supply chain tracker with transfer timeline (#562)
- Terraform: ECS, RDS, VPC, ALB, ECR, cost optimisation modules (#563)

### Changed
- `ParticipantRole` serialisation now uses string representation for readability (#564)
- Docker compose stack unified for local dev with hot-reload (#565)

### Fixed
- `transfer_waste` did not validate lat/lon range for intermediate nodes (#566)
- Frontend routing sent unauthenticated users to a blank page instead of `/login` (#567)

---

## [0.2.0] — 2026-02-14

### Added
- Soroban contract: incentive system (`create_incentive`, `update_incentive`, `distribute_rewards`) (#550–#554)
- Backend: Rust/Axum API server with contract proxy endpoints (#549)
- Indexer: TypeScript event indexer with PostgreSQL persistence (#548)
- Firebase Authentication integration with Stellar key linking (#547)
- Frontend: IncentivesMarketplacePage and ManufacturerDashboardPage (#546)

### Changed
- Contract storage migrated from `Persistent` to `Temporary` for waste entries to reduce ledger fees (#545)

### Security
- Input validation module (`validation.rs`) added to contract; all public functions gated (#544)

---

## [0.1.0] — 2026-01-31

### Added
- Initial Soroban smart contract with participant registration, waste submission, and role-based access (#541–#543)
- `ParticipantRole` enum: Recycler, Collector, Manufacturer (#541)
- On-chain global metrics and per-participant recycling stats (#542)
- Basic React/Vite/TypeScript frontend scaffold with Tailwind CSS (#540)
- GitHub Actions CI: Rust quality checks, frontend checks, security audit (#539)
- `docs/` directory: Architecture, API Reference, Deployment Runbook, User Guide (#538)

---

## Changelog Guidelines

### Format

Entries follow **Keep a Changelog** conventions:

- **Added** — New features
- **Changed** — Changes to existing behaviour
- **Deprecated** — Features to be removed in a future release
- **Removed** — Features removed in this release
- **Fixed** — Bug fixes
- **Security** — Security vulnerability fixes or hardening

### Issue References

Every entry should reference the closing GitHub issue, e.g. `(#786)`.

### Release Process

1. Before merging to `main`, move items from `[Unreleased]` to a new versioned section.
2. Use [Semantic Versioning](https://semver.org/): `MAJOR.MINOR.PATCH`
   - `PATCH` — Backward-compatible bug fixes
   - `MINOR` — Backward-compatible new features
   - `MAJOR` — Breaking changes
3. Tag the release commit: `git tag -a v0.12.0 -m "v0.12.0"`
4. Push tag to trigger the release workflow: `git push origin v0.12.0`
5. GitHub Actions generates the GitHub Release with auto-generated release notes.

### Automation

`scripts/generate-changelog-entry.sh` can pre-populate an entry from merged PRs since the last tag:

```bash
./scripts/generate-changelog-entry.sh --since v0.11.0
```

[Unreleased]: https://github.com/Xoulomon/Scavenger/compare/v0.11.0...HEAD
[0.11.0]: https://github.com/Xoulomon/Scavenger/compare/v0.10.0...v0.11.0
[0.10.0]: https://github.com/Xoulomon/Scavenger/compare/v0.9.0...v0.10.0
[0.9.0]: https://github.com/Xoulomon/Scavenger/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/Xoulomon/Scavenger/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/Xoulomon/Scavenger/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/Xoulomon/Scavenger/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/Xoulomon/Scavenger/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/Xoulomon/Scavenger/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/Xoulomon/Scavenger/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/Xoulomon/Scavenger/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/Xoulomon/Scavenger/releases/tag/v0.1.0
