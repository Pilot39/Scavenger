# Developer Onboarding Guide

Welcome to the Scavngr codebase. This guide gets you from zero to a running local environment and your first pull request.

> **This is the canonical setup guide.** Setup steps for the contract, indexer,
> frontend, backend, and mobile app all live here. Component READMEs cover what is
> specific to that component and link back here for setup rather than repeating it.
> If you find setup instructions duplicated elsewhere, that is a bug — please replace
> them with a link to this guide.

---

## Table of Contents

1. [Development Environment Setup](#development-environment-setup)
   - [Prerequisites](#prerequisites)
   - [Path A: Docker (recommended)](#path-a-docker-recommended)
   - [Path B: Running components directly](#path-b-running-components-directly)
   - [Environment Variables](#environment-variables)
   - [Local Run Commands](#local-run-commands)
   - [Verifying Setup](#verifying-setup)
   - [Setup Troubleshooting](#setup-troubleshooting)
2. [Project Structure Overview](#project-structure-overview)
3. [Development Workflow](#development-workflow)
4. [Coding Standards](#coding-standards)
5. [Testing Guide](#testing-guide)
6. [Debugging Guide](#debugging-guide)
7. [Contribution Guidelines](#contribution-guidelines)
8. [Commit Message Standards](#commit-message-standards)
9. [Local Development Tips](#local-development-tips)

---

## Development Environment Setup

There are two ways to get running. **Path A (Docker)** brings up every service with
one command and is what most contributors should use. **Path B** runs components
directly on your machine, which you will want for the component you are actively
working on.

The two compose: a common setup is Docker for the dependencies (Stellar, Postgres,
Redis) and Path B for whichever component you are editing.

### Prerequisites

**Everyone needs:**

| Tool | Version | Install |
|---|---|---|
| Git | 2.40+ | https://git-scm.com |
| Docker Desktop | 24+ | https://docker.com — needs Compose v2 (`docker compose version`) |

**Additionally, by component:**

| Component | Tool | Version | Install |
|---|---|---|---|
| Contract | Rust | stable (1.70+) | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| Contract | wasm32 target | — | `rustup target add wasm32-unknown-unknown` |
| Contract | Soroban CLI | latest | `cargo install --locked soroban-cli --features opt` |
| Indexer | Node.js | 18+ | https://nodejs.org (LTS) |
| Indexer | PostgreSQL | 14+ | Provided by Docker; only needed natively for Path B |
| Frontend | Node.js | 18+ | https://nodejs.org (LTS) |
| Frontend | Freighter wallet | latest | https://freighter.app — browser extension, required for contract calls |
| Backend | Rust | stable (1.70+) | as above |
| Mobile | Node.js | 18+ | https://nodejs.org (LTS) |
| Mobile | React Native CLI | latest | `npm install -g react-native-cli` |
| Mobile | Xcode | latest | iOS builds only (macOS) |
| Mobile | Android Studio | latest | Android builds only |
| Perf tests | k6 | latest | https://k6.io — only for `performance/` |

The repo pins its Rust toolchain in `rust-toolchain.toml`, so `rustup` will select the
right version automatically the first time you build.

### Recommended VS Code Extensions

```json
{
  "recommendations": [
    "rust-lang.rust-analyzer",
    "tamasfe.even-better-toml",
    "bradlc.vscode-tailwindcss",
    "esbenp.prettier-vscode",
    "dbaeumer.vscode-eslint",
    "ms-azuretools.vscode-docker"
  ]
}
```

### Clone and fork

Do this once, whichever path you take:

```bash
# 1. Fork the repo on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/Scavenger.git
cd Scavenger

# 2. Add the upstream remote so you can sync later
git remote add upstream https://github.com/Xoulomon/Scavenger.git
```

### Path A: Docker (recommended)

Brings up Stellar standalone, Postgres, Redis, the backend, the indexer, and the
frontend together.

```bash
# 1. Create the root .env used by docker compose
cp frontend/.env.example .env
# Only the Firebase vars need real values for local dev; the compose file
# substitutes dev stubs for anything you leave unset, and overrides the
# Stellar vars to point at the local standalone network.

# 2. Start everything
docker compose up -d

# 3. Stellar needs ~30 s to initialise. Watch until all services are healthy:
docker compose ps
```

Once up:

| Service | URL | Notes |
|---|---|---|
| Stellar standalone | http://localhost:8000 | Horizon API + Soroban RPC + friendbot |
| Frontend | http://localhost:5173 | Vite dev server with HMR |
| Backend | http://localhost:8080 | Rust / Actix-web API |
| Indexer | http://localhost:3001 | Event indexer + REST API |
| PostgreSQL | localhost:5432 | user `scavngr`, password `scavngr_dev`, db `scavngr` |
| Redis | localhost:6379 | Cache / job queue |

**Then deploy the contract**, which nothing else can do for you:

```bash
# Generate a keypair and fund it from the local friendbot
soroban keys generate local-deployer --network standalone
curl "http://localhost:8000/friendbot?addr=$(soroban keys address local-deployer)"

# Build and deploy
cargo build --target wasm32-unknown-unknown --release
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/stellar_scavngr_contract.optimized.wasm \
  --source local-deployer \
  --network standalone

# Record the returned contract ID and restart the services that consume it
echo "CONTRACT_ID=<returned-id>" >> .env
docker compose up -d --force-recreate indexer frontend
```

Until `CONTRACT_ID` is set, the frontend and indexer start but every contract call
fails. This is the single most common "my setup is broken" cause.

For Docker-specific details — hot reload, seed data, port overrides, per-container
shells — see [DEV_ENVIRONMENT.md](./DEV_ENVIRONMENT.md).

### Path B: Running components directly

Each block below assumes you have cloned the repo and are at its root. If you are not
running the full Docker stack, start at least its dependencies first:

```bash
docker compose up -d stellar postgres redis
```

#### Contract (`stellar-contract/`)

```bash
# Build (native, for tests)
cargo build --release

# Build WASM
cargo build --target wasm32-unknown-unknown --release

# Optimise the WASM artifact
soroban contract optimize \
  --wasm target/wasm32-unknown-unknown/release/stellar_scavngr_contract.wasm

# Test
cargo test
```

Deploying to testnet instead of standalone:

```bash
soroban keys generate testnet-deployer
curl "https://friendbot.stellar.org?addr=$(soroban keys address testnet-deployer)"
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/stellar_scavngr_contract.optimized.wasm \
  --source testnet-deployer \
  --network testnet
```

#### Indexer (`indexer/`)

```bash
cd indexer
npm install
cp .env.example .env
# Set CONTRACT_ID to the ID you deployed above. DATABASE_URL must point at a
# reachable Postgres; the Docker one is
#   postgresql://scavngr:scavngr_dev@localhost:5432/scavngr

npm run migrate   # apply SQL migrations (also runs automatically on start)
npm run dev       # ts-node, watches src/
```

The indexer refuses to start without `STELLAR_RPC_URL` and `CONTRACT_ID`.

#### Frontend (`frontend/`)

```bash
cd frontend
npm install
cp .env.example .env
# Set VITE_CONTRACT_ID, VITE_NETWORK, VITE_RPC_URL

npm run dev       # http://localhost:5173
```

The app validates `VITE_CONTRACT_ID`, `VITE_NETWORK`, and `VITE_RPC_URL` at startup and
fails loudly if any is missing or malformed. Install the
[Freighter](https://freighter.app) extension before trying any action that signs a
transaction.

#### Backend (`backend/`)

```bash
cd backend
cargo run
# Reads the root .env; see the Environment Variables section below.
```

#### Mobile (`mobile/`)

```bash
cd mobile
npm install
cp .env.example .env 2>/dev/null || cat > .env <<'ENVEOF'
REACT_APP_API_URL=http://localhost:8080
REACT_APP_CONTRACT_ID=your_contract_id
REACT_APP_NETWORK=testnet
ENVEOF

npm start                 # Metro bundler
npm run ios               # iOS simulator (macOS only)
npm run android           # Android emulator
```

### Environment Variables

There are **three** env files, and they are not interchangeable. Copy each from its
own template.

| File | Template | Consumed by |
|---|---|---|
| `.env` (repo root) | `frontend/.env.example` | `docker compose`, backend |
| `frontend/.env` | `frontend/.env.example` | Vite dev server (Path B only) |
| `indexer/.env` | `indexer/.env.example` | Indexer process (Path B only) |
| `mobile/.env` | — (see the mobile block above) | React Native app |

> The root `.env` is seeded from `frontend/.env.example` because compose passes the
> `VITE_*` values through to the frontend container. That is why the same template
> serves two files.

**Frontend — `VITE_*`** (`frontend/.env.example`):

| Variable | Required | Description |
|---|---|---|
| `VITE_CONTRACT_ID` | ✅ | Deployed Soroban contract ID |
| `VITE_NETWORK` | ✅ | `TESTNET`, `MAINNET`, `FUTURENET`, or `STANDALONE` |
| `VITE_RPC_URL` | ✅ | Soroban RPC endpoint |
| `VITE_FIREBASE_API_KEY` | ✅ | Firebase project API key |
| `VITE_FIREBASE_AUTH_DOMAIN` | ✅ | Firebase auth domain |
| `VITE_FIREBASE_PROJECT_ID` | ✅ | Firebase project ID |
| `VITE_FIREBASE_STORAGE_BUCKET` | ✅ | Firebase storage bucket |
| `VITE_FIREBASE_MESSAGING_SENDER_ID` | ✅ | Firebase messaging sender ID |
| `VITE_FIREBASE_APP_ID` | ✅ | Firebase app ID |
| `VITE_FIREBASE_MEASUREMENT_ID` | ✅ | Firebase measurement ID |
| `VITE_PINATA_JWT` | — | Pinata IPFS token; omit to use the dev stub |

Under Docker, every `VITE_FIREBASE_*` value falls back to `dev-stub`, so you can bring
the stack up before you have a Firebase project.

**Indexer** (`indexer/.env.example`):

| Variable | Required | Default | Description |
|---|---|---|---|
| `DATABASE_URL` | ✅ | `postgresql://localhost/scavenger` | Postgres connection string |
| `STELLAR_RPC_URL` | ✅ | — | Soroban RPC endpoint; startup fails without it |
| `CONTRACT_ID` | ✅ | — | Contract to index; startup fails without it |
| `NETWORK_PASSPHRASE` | ✅ | — | Must match the network |
| `START_LEDGER` | — | `0` | Ledger to begin indexing from |
| `POLL_INTERVAL_MS` | — | `5000` | RPC poll interval |
| `API_HOST` | — | `0.0.0.0` | REST API bind address |
| `API_PORT` | — | `3001` | REST API port |
| `DB_MAX_CONNECTIONS` | — | `20` | Connection pool size |
| `LOG_LEVEL` / `LOG_FORMAT` | — | `info` / `json` | Logging |

**Backend** (root `.env.example`) — server, security, email, storage, search,
rate limiting, and archival settings. `CSRF_SECRET` and `ALLOWED_ORIGINS` must be set
for anything beyond local development. See `.env.example` for the annotated list.

**Never commit a populated `.env`.** All four paths are gitignored; keep it that way.

### Local Run Commands

| Component | Install | Run | Test | Lint / Format |
|---|---|---|---|---|
| Contract | — | `soroban contract deploy …` | `cargo test` | `cargo fmt`, `cargo clippy` |
| Indexer | `cd indexer && npm install` | `npm run dev` | `npm test` | — |
| Frontend | `cd frontend && npm install` | `npm run dev` | `npm test` | `npm run lint`, `npm run format` |
| Backend | — | `cd backend && cargo run` | `cargo test` | `cargo fmt`, `cargo clippy` |
| Mobile | `cd mobile && npm install` | `npm start` | `npm test` | `npm run lint` |
| Full stack | — | `docker compose up -d` | — | — |

Component-specific test suites (integration, security, performance, E2E) have their
own READMEs — they assume this environment is already up:

- [`integration-tests/README.md`](../integration-tests/README.md)
- [`security-tests/README.md`](../security-tests/README.md)
- [`performance/README.md`](../performance/README.md)
- [`docs/E2E_TESTING.md`](./E2E_TESTING.md)

### Verifying Setup

Run these in order. Each should succeed before you move to the next.

```bash
# 1. Contract compiles and its tests pass
cargo build --target wasm32-unknown-unknown --release
cargo test

# 2. Docker stack is healthy (Path A)
docker compose ps          # every service "healthy" or "running"
curl -sf http://localhost:8000/friendbot?addr=test | head -c 1   # Stellar up
curl -sf http://localhost:8080/health                            # Backend
curl -sf http://localhost:3001/health                            # Indexer

# 3. Frontend serves
curl -sf http://localhost:5173 >/dev/null && echo "frontend ok"

# 4. Contract is deployed and reachable
soroban contract invoke --id "$CONTRACT_ID" --network standalone -- get_metrics
```

You have a working environment when step 4 returns metrics rather than an error.

### Setup Troubleshooting

| Symptom | Cause | Fix |
|---|---|---|
| Contract calls fail; UI loads fine | `CONTRACT_ID` unset or stale | Deploy the contract, put the ID in `.env`, `docker compose up -d --force-recreate indexer frontend` |
| Indexer exits immediately on start | `STELLAR_RPC_URL` or `CONTRACT_ID` missing | Both are required; the process throws at startup by design |
| Stellar container unhealthy for ~30 s | Normal — it initialises slowly | Wait. Other services gate on it via `depends_on` health checks |
| Port already in use | Something else owns 5173/8000/8080/3001/5432/6379 | Override the port in a `docker-compose.override.yml` (example in [DEV_ENVIRONMENT.md](./DEV_ENVIRONMENT.md)) |
| `wasm32-unknown-unknown` target not found | Target not installed | `rustup target add wasm32-unknown-unknown` |
| Frontend throws on startup about env vars | A required `VITE_*` is missing or malformed | Validation is deliberate; fill in `frontend/.env` |
| "Freighter is not installed" | Extension missing | Install from https://freighter.app and reload |
| Postgres connection refused from a native indexer | Using the Docker-internal hostname | Use `localhost:5432` from the host, `postgres:5432` from inside a container |
| Stale state after schema changes | Old volumes | `docker compose down -v` then `docker compose up -d` (destroys all local data) |

---

## Project Structure Overview

```
Scavenger/
├── stellar-contract/          # Soroban smart contract (Rust)
│   ├── src/
│   │   ├── lib.rs             # ScavengerContract — all public functions
│   │   ├── types.rs           # Waste, Participant, Incentive, etc.
│   │   ├── errors.rs          # Error enum
│   │   ├── events.rs          # Event emitters
│   │   ├── validation.rs      # Reusable validation utilities
│   │   └── search.rs          # Search/query helpers
│   └── tests/                 # Integration tests (one file per feature)
│
├── frontend/                  # React + Vite + TypeScript + Tailwind
│   └── src/
│       ├── api/               # Contract client and Horizon API calls
│       ├── components/        # Reusable UI components
│       ├── context/           # React context (wallet, theme)
│       ├── hooks/             # Custom React hooks
│       ├── pages/             # Page-level components
│       └── types/             # TypeScript type definitions
│
├── backend/                   # Rust/Actix-web API server
│   └── src/                   # REST endpoints, indexer bridge
│
├── indexer/                   # TypeScript Stellar event indexer
│   └── src/
│       ├── handlers/          # Event handlers per contract event
│       ├── db/                # Database queries (PostgreSQL)
│       └── stellar/           # Horizon API client
│
├── docs/                      # All documentation
├── scripts/                   # Deployment and maintenance scripts
├── config/                    # Prometheus, alertmanager, etc.
├── k8s/                       # Kubernetes manifests
└── docker-compose.yml         # Local dev stack
```

### Key Files to Read First

| File | Why |
|---|---|
| `stellar-contract/src/lib.rs` | All contract functions — start here |
| `stellar-contract/src/types.rs` | Data structures (`Waste`, `Participant`, `Incentive`) |
| `stellar-contract/src/validation.rs` | Shared validation utilities |
| `frontend/src/api/` | How the frontend talks to the contract |
| `docs/ARCHITECTURE.md` | System design and data flow |
| `docs/API_DOCUMENTATION.md` | All public contract functions documented |

---

## Development Workflow

### Branch Strategy

```
main                    ← stable, protected, CI required
  └── feature/issue-NNN-short-description   ← your work
  └── fix/issue-NNN-short-description
  └── docs/issue-NNN-short-description
  └── refactor/issue-NNN-short-description
```

### Standard Workflow

```bash
# 1. Sync with upstream before starting
git fetch upstream
git checkout main
git merge upstream/main

# 2. Create feature branch
git checkout -b feature/issue-123-add-batch-verification

# 3. Make changes, run tests frequently
cargo test                          # contract tests
cd frontend && npm test -- --run    # frontend tests

# 4. Commit in logical chunks (see commit message standards below)
git add -p                          # stage selectively
git commit -m "feat(#123): add batch verification function"

# 5. Push and open PR
git push origin feature/issue-123-add-batch-verification
# Open PR on GitHub targeting main
```

### Before Opening a PR

```bash
# Contract: format, lint, test
cargo fmt
cargo clippy -- -D warnings
cargo test

# Frontend: format, lint, build
cd frontend
npm run lint
npm run build
npm test -- --run
```

---

## Coding Standards

### Rust (Contract)

**Formatting** — enforced by `cargo fmt` (rustfmt.toml in root).

**Linting** — enforced by `cargo clippy -- -D warnings`. Zero warnings policy.

**Patterns to follow:**

```rust
// ✅ Use validation utilities from validation.rs
use crate::validation::{validate_weight, validate_coordinates};
validate_weight(waste.weight, "waste weight");

// ✅ Document all public functions
/// Registers a new participant in the supply chain.
///
/// # Arguments
/// * `address` — Stellar address (must authorize).
/// * `role`    — [`ParticipantRole`] variant.
///
/// # Errors
/// Panics `"Participant already registered"` if already registered.
pub fn register_participant(env: Env, address: Address, ...) { ... }

// ✅ Batch storage reads at the top of a function
let inst = env.storage().instance();
let status = inst.get(&KEY_STATUS).unwrap();
let admin  = inst.get(&KEY_ADMIN).unwrap();

// ❌ Avoid repeated storage reads mid-function
let status = env.storage().instance().get(&KEY_STATUS).unwrap();
// ... other code ...
let status2 = env.storage().instance().get(&KEY_STATUS).unwrap(); // redundant
```

**Error handling** — use `panic!()` for contract-level errors (Soroban converts to contract errors). Include descriptive messages.

### TypeScript (Frontend)

**Formatting** — Prettier with `.prettierrc` config.

**Linting** — ESLint with `.eslintrc.cjs`.

**Patterns:**

```typescript
// ✅ Typed contract calls
const result = await contractClient.register_participant({
  address: walletAddress,
  role: ParticipantRole.Recycler,
  name: 'My Org',
  latitude: BigInt(52_520_000),
  longitude: BigInt(13_405_000),
});

// ✅ Handle loading and error states
const { data, isLoading, error } = useContractQuery('get_participant', address);

// ✅ Use existing hooks — don't re-implement wallet logic
const { walletAddress, isConnected } = useWallet();

// ❌ Don't hardcode network URLs
const RPC_URL = import.meta.env.VITE_RPC_URL; // ✅
const RPC_URL = 'https://soroban-testnet.stellar.org'; // ❌
```

---

## Testing Guide

### Contract Tests

Tests live in `stellar-contract/tests/`. One file per feature area.

```bash
# Run all tests
cargo test

# Run a specific test file
cargo test --test waste_registration_flow_test

# Run with output (useful for debugging)
cargo test -- --nocapture

# Run a specific test function
cargo test test_register_participant_success
```

**Writing a new test:**

```rust
// stellar-contract/tests/my_feature_test.rs
#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env};
use stellar_scavngr_contract::ScavengerContractClient;

fn setup() -> (Env, Address, ScavengerContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let id = env.register_contract(None, stellar_scavngr_contract::ScavengerContract);
    let client = ScavengerContractClient::new(&env, &id);
    let admin = Address::generate(&env);
    client.initialize_admin(&admin);
    (env, admin, client)
}

#[test]
fn test_my_feature_happy_path() {
    let (env, admin, client) = setup();
    // ... test body
}

#[test]
#[should_panic(expected = "specific error message")]
fn test_my_feature_error_case() {
    let (env, _, client) = setup();
    // ... trigger error
}
```

### Frontend Tests

```bash
cd frontend

# Run all tests once
npm test -- --run

# Watch mode
npm test

# Coverage report
npm run test:coverage
```

---

## Debugging Guide

### Contract Debugging

```bash
# Run with verbose output
RUST_LOG=debug cargo test -- --nocapture

# Inspect contract storage
soroban contract read \
  --id "$CONTRACT_ID" \
  --network standalone

# Simulate a call (no state change)
soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source local-deployer \
  --network standalone \
  --simulate \
  -- get_metrics
```

### Frontend Debugging

```bash
# Check what contract ID is loaded
console.log(import.meta.env.VITE_CONTRACT_ID);

# Enable verbose Soroban SDK logging
localStorage.setItem('debug', 'soroban:*');
# Reload page

# Inspect raw XDR responses
soroban contract invoke ... --verbose 2>&1 | jq .
```

### Docker Debugging

```bash
# View all service logs
docker compose logs -f

# View specific service
docker compose logs -f stellar frontend

# Open shell in a container
docker compose exec backend bash
docker compose exec stellar bash

# Check service health
docker compose ps
```

---

## Contribution Guidelines

### Opening an Issue

Before starting work, ensure an issue exists. If not, create one describing:
- What you want to add/fix
- Why it's needed
- Acceptance criteria

### Pull Request Requirements

Every PR must:

- [ ] Reference the issue it closes (`Closes #NNN` in description)
- [ ] Have a clear title: `type(#NNN): short description`
- [ ] Pass all CI checks (`cargo test`, `cargo clippy`, `npm run build`)
- [ ] Include tests for new functionality
- [ ] Update relevant documentation
- [ ] Have no merge conflicts with `main`

### PR Size Guidelines

| Size | Lines Changed | Approach |
|---|---|---|
| Small | < 200 | Single PR, fast review |
| Medium | 200–500 | Split if possible |
| Large | > 500 | Must split into smaller PRs |

### Code Review Etiquette

- **Authors**: Respond to all comments before requesting re-review
- **Reviewers**: Approve or request changes within 48 hours
- **Blocking comments**: Use `BLOCKING:` prefix for must-fix issues
- **Suggestions**: Use `SUGGESTION:` prefix for optional improvements

---

## Commit Message Standards

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
type(#issue): short description (max 72 chars)

Optional longer explanation. Wrap at 80 chars.
Explain WHY, not WHAT (the diff shows what).

Closes #NNN
```

### Types

| Type | When to Use |
|---|---|
| `feat` | New feature or function |
| `fix` | Bug fix |
| `docs` | Documentation only |
| `refactor` | Code change that neither fixes a bug nor adds a feature |
| `test` | Adding or improving tests |
| `chore` | Build process, tooling, dependencies |
| `perf` | Performance improvement |

### Examples

```bash
# ✅ Good
git commit -m "feat(#45): add batch waste submission with 25-item limit"
git commit -m "fix(#89): correct latitude validation boundary (was exclusive, now inclusive)"
git commit -m "docs(#102): add troubleshooting section for wallet connection errors"
git commit -m "refactor(#757): extract validation utilities to shared module"

# ❌ Bad
git commit -m "fix stuff"
git commit -m "WIP"
git commit -m "update files"
```

---

## Local Development Tips

### Speed Up Contract Iteration

```bash
# Use cargo-watch for auto-rebuild
cargo install cargo-watch
cargo watch -x "test 2>&1 | tail -20"

# Skip WASM build during development (use rlib only)
cargo test --lib  # faster than full test suite
```

### Use Snapshots for Stable Tests

```bash
# Update all snapshots after intentional changes
cargo test -- --update-snapshots

# Review snapshot diffs before committing
git diff stellar-contract/test_snapshots/
```

### Environment Variable Shortcuts

```bash
# Add to ~/.bashrc or ~/.zshrc
alias scav-deploy='soroban contract deploy \
  --wasm stellar-contract/target/wasm32-unknown-unknown/release/stellar_scavngr_contract.optimized.wasm \
  --source local-deployer \
  --network standalone'

alias scav-metrics='soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source local-deployer \
  --network standalone \
  -- get_metrics'
```

### Common Pitfalls

| Pitfall | Solution |
|---|---|
| Tests fail with "admin not set" | Call `initialize_admin` in test setup |
| Contract not found after restart | Docker volumes cleared — redeploy |
| Frontend shows stale data | Hard-refresh (Ctrl+Shift+R) or clear localStorage |
| `cargo clippy` fails on new code | Run `cargo clippy --fix` for auto-fixes |
| WASM too large | Ensure `[profile.release]` has `opt-level = "z"` in Cargo.toml |

---

*Last updated: June 2026 | Questions? Open a GitHub Discussion.*
