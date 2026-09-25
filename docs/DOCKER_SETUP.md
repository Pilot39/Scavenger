# Docker Setup

This project uses Docker and Docker Compose to run all services locally.

---

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) 24+
- [Docker Compose](https://docs.docker.com/compose/install/) v2.20+

---

## Services

| Service  | Image             | Local port | Description                       |
|----------|-------------------|------------|-----------------------------------|
| stellar  | stellar/quickstart | 8000       | Stellar standalone + Soroban RPC  |
| postgres | postgres:16-alpine | 5432       | Primary database                  |
| redis    | redis:7-alpine    | 6379       | Cache / session store             |
| backend  | ./backend         | 8080       | Rust / Actix-web API              |
| indexer  | ./indexer         | 3001       | Node.js blockchain indexer        |
| frontend | ./frontend        | 5173       | Vite / React UI                   |

---

## Quick Start

```bash
# Start all services (detached)
docker compose up -d

# View logs for a specific service
docker compose logs -f backend

# Stop all services
docker compose down

# Stop and remove volumes
docker compose down -v
```

---

## Environment Variables

Copy `.env.example` to `.env` and fill in the required values before starting:

```bash
cp .env.example .env
```

| Variable              | Required | Description                           |
|-----------------------|----------|---------------------------------------|
| `CONTRACT_ID`         | No       | Deployed Soroban contract address     |
| `VITE_FIREBASE_*`     | No       | Firebase config (dev stubs used by default) |

---

## Build Targets

Each service Dockerfile defines three build stages:

| Target    | Purpose                                   |
|-----------|-------------------------------------------|
| `dev`     | Hot-reload for local development          |
| `builder` | Compile / bundle production artefacts     |
| `runtime` | Minimal production image                  |

Use the `target` field in `docker-compose.yml` (already set to `dev`) or override at build time:

```bash
# Build a production image for the backend
docker build --target runtime -t scavngr-backend:prod ./backend
```

---

## Dockerfiles

### frontend — `frontend/Dockerfile`

| Stage     | Base image       | Notes                          |
|-----------|------------------|--------------------------------|
| dev       | node:20-alpine   | `npm run dev --host 0.0.0.0`   |
| builder   | node:20-alpine   | `npm run build` → `/app/dist`  |
| runtime   | nginx:alpine     | Serves static build artefacts  |

### backend — `backend/Dockerfile`

| Stage     | Base image          | Notes                                      |
|-----------|---------------------|--------------------------------------------|
| dev       | rust:1.78-slim      | `cargo-watch` for live reload              |
| builder   | rust:1.78-slim      | `cargo build --release`                    |
| runtime   | debian:bookworm-slim | Minimal binary-only image, port 8080      |

### indexer — `indexer/Dockerfile`

| Stage     | Base image      | Notes                                        |
|-----------|-----------------|----------------------------------------------|
| dev       | node:20-alpine  | `npm run dev`                                |
| builder   | node:20-alpine  | `npm run build` → `/app/dist`                |
| runtime   | node:20-alpine  | Production deps only, `node dist/index.js`   |

---

## Additional Compose Files

| File                            | Purpose                              |
|---------------------------------|--------------------------------------|
| `docker-compose.yml`            | Core dev environment                 |
| `docker-compose.monitoring.yml` | Prometheus + Grafana stack           |
| `docker-compose.logging.yml`    | Loki + Promtail log aggregation      |

Merge files with the `-f` flag:

```bash
# Core + monitoring
docker compose -f docker-compose.yml -f docker-compose.monitoring.yml up -d
```

---

## Healthchecks

All services define Docker healthchecks. Wait for healthy status before running scripts:

```bash
docker compose up -d
docker compose ps   # STATUS column shows (healthy) / (starting)
```

---

## Rebuilding After Code Changes

Volume mounts keep source code in sync during development for the frontend and indexer. The backend uses `cargo-watch` to recompile on change.

To force a full image rebuild:

```bash
docker compose build --no-cache <service>
docker compose up -d <service>
```
