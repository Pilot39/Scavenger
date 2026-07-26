-- Migration: 003_query_indexes (issue #761)
-- Adds composite and covering indexes identified as missing during query analysis.

-- Covering index for paginated event listing (avoids heap fetch for common columns)
CREATE INDEX IF NOT EXISTS idx_raw_events_ledger_type
  ON raw_events (ledger_sequence DESC, id DESC, event_type, contract_id, created_at);

-- Composite index for per-recycler active-waste queries (N+1 fix)
CREATE INDEX IF NOT EXISTS idx_wastes_recycler_active_registered
  ON wastes (recycler_address, is_active, registered_at DESC);

-- Composite index for transfer history ordered by time (used in supply-chain view)
CREATE INDEX IF NOT EXISTS idx_waste_transfers_waste_time
  ON waste_transfers (waste_id, transferred_at ASC);

-- Partial index: only active participants (most queries filter is_active = true)
CREATE INDEX IF NOT EXISTS idx_participants_active
  ON participants (address) WHERE is_active = true;

-- Partial index: only active wastes
CREATE INDEX IF NOT EXISTS idx_wastes_active_only
  ON wastes (recycler_address, registered_at DESC) WHERE is_active = true;

-- Index for token reward aggregation by recipient
CREATE INDEX IF NOT EXISTS idx_token_rewards_recipient_amount
  ON token_rewards (recipient_address, amount);

-- Index for date-range event queries
CREATE INDEX IF NOT EXISTS idx_raw_events_created_at
  ON raw_events (created_at, event_type);
