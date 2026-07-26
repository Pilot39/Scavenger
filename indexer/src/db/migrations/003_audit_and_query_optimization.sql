-- Migration: 003_audit_and_query_optimization
-- Adds audit_logs table and composite indexes for query performance

-- Audit log table for on-chain sensitive operations
CREATE TABLE IF NOT EXISTS audit_logs (
  id BIGSERIAL PRIMARY KEY,
  action TEXT NOT NULL,
  actor_address TEXT NOT NULL,
  target TEXT NOT NULL,
  details TEXT,
  ledger_sequence BIGINT,
  timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_audit_actor ON audit_logs(actor_address);
CREATE INDEX IF NOT EXISTS idx_audit_action ON audit_logs(action);
CREATE INDEX IF NOT EXISTS idx_audit_timestamp ON audit_logs(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_audit_actor_action ON audit_logs(actor_address, action);

-- Composite indexes for common query patterns
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_wastes_type_registered
  ON wastes(waste_type, registered_at DESC);

CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_wastes_recycler_active_time
  ON wastes(recycler_address, is_active, registered_at DESC);

CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_transfers_waste_time
  ON waste_transfers(waste_id, transferred_at DESC);

CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_rewards_recipient_time
  ON token_rewards(recipient_address, rewarded_at DESC);

-- Partial indexes for filtered queries
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_wastes_active_only
  ON wastes(registered_at DESC) WHERE is_active = true;

CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_wastes_confirmed_active
  ON wastes(recycler_address, registered_at DESC) WHERE is_confirmed = true AND is_active = true;

-- Composite index for raw_events range + type queries
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_raw_events_ledger_type
  ON raw_events(ledger_sequence DESC, event_type);
