-- Seed data for local development
-- Creates the indexer schema and inserts sample participants

CREATE TABLE IF NOT EXISTS sync_status (
    id SERIAL PRIMARY KEY,
    last_ledger BIGINT NOT NULL DEFAULT 0,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO sync_status (last_ledger) VALUES (0) ON CONFLICT DO NOTHING;
