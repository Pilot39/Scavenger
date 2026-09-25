# Development Environment — #484

> **Start with the [Developer Onboarding Guide](./DEVELOPER_ONBOARDING.md#development-environment-setup)**,
> which is the canonical setup path for all components. This document is the
> Docker-specific reference: service URLs, hot reload, seed data, container shells,
> and port overrides.

## Prerequisites

- Docker Desktop ≥ 24 (or Docker Engine + Compose plugin v2)
- `docker compose version` should show `v2.x`

## Quick start

```bash
# 1. Clone and enter the repo
git clone https://github.com/Xoulomon/Scavenger.git
cd Scavenger

# 2. (Optional) copy and fill in Firebase credentials
cp frontend/.env.example .env
# Edit .env — only Firebase vars are needed for local dev;
# Stellar vars are pre-configured for the local standalone network.

# 3. Start all services
docker compose up -d

# 4. Watch logs
docker compose logs -f
```

## Services

| Service | URL | Description |
|---|---|---|
| Stellar standalone | http://localhost:8000 | Horizon API + Soroban RPC |
| Frontend | http://localhost:5173 | Vite dev server with HMR |
| Backend | http://localhost:8080 | Rust/Actix-web API |
| Indexer | http://localhost:3001 | Stellar event indexer |
| PostgreSQL | localhost:5432 | Database (user: `scavngr`, pass: `scavngr_dev`) |
| Redis | localhost:6379 | Cache / job queue |

## Deploy the contract locally

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

# Set the returned contract ID in your .env
echo "CONTRACT_ID=<returned-id>" >> .env

# Restart services to pick up the new CONTRACT_ID
docker compose up -d --force-recreate indexer frontend
```

## Hot-reload

- **Frontend** — Vite HMR is active. Edit files under `frontend/src/` and the browser updates instantly.
- **Backend** — `cargo-watch` restarts the server on any change under `backend/src/`.
- **Indexer** — `ts-node` watches `indexer/src/` via `npm run dev`.

## Useful commands

```bash
# Stop everything (keep volumes)
docker compose stop

# Destroy everything including volumes (fresh start)
docker compose down -v

# Run backend tests
docker compose exec backend cargo test

# Run indexer tests
docker compose exec indexer npm test

# Open a psql shell
docker compose exec postgres psql -U scavngr scavngr

# Flush Redis
docker compose exec redis redis-cli FLUSHALL
```

## Seed data

`scripts/seed.sql` is mounted into the Postgres container and runs automatically on first start. It creates the `sync_status` table used by the indexer. Add additional seed rows there as needed.

## Troubleshooting

**Stellar container takes too long to start** — it needs ~30 s to initialise. The indexer and frontend will wait via `depends_on` health checks.

**`CONTRACT_ID` is empty** — the frontend and indexer will start but contract calls will fail. Deploy the contract first (see above) and restart those services.

**Port conflicts** — if any port is already in use, override it in a `docker-compose.override.yml`:

```yaml
services:
  frontend:
    ports:
      - "5174:5173"
```
