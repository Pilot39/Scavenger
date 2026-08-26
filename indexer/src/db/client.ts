import { Pool, PoolClient, PoolConfig } from 'pg';
import { logger } from '../utils/logger';

let pool: Pool | null = null;

/**
 * Build pool configuration from environment variables.
 *
 * Supported env vars (all optional, sensible defaults provided):
 *   DATABASE_URL              – connection string (required by pg)
 *   DB_MAX_CONNECTIONS        – pool max size              (default: 10)
 *   DB_MIN_CONNECTIONS        – pool min size / idle slots (default: 2)
 *   DB_IDLE_TIMEOUT_MS        – ms before idle client is closed (default: 30000)
 *   DB_CONNECTION_TIMEOUT_MS  – ms to wait for a free client  (default: 5000)
 *   DB_STATEMENT_TIMEOUT_MS   – ms before a query is cancelled (default: 30000)
 */
function buildPoolConfig(): PoolConfig {
  return {
    connectionString: process.env.DATABASE_URL,
    max: parseInt(process.env.DB_MAX_CONNECTIONS ?? '10', 10),
    min: parseInt(process.env.DB_MIN_CONNECTIONS ?? '2', 10),
    idleTimeoutMillis: parseInt(process.env.DB_IDLE_TIMEOUT_MS ?? '30000', 10),
    connectionTimeoutMillis: parseInt(process.env.DB_CONNECTION_TIMEOUT_MS ?? '5000', 10),
    statement_timeout: parseInt(process.env.DB_STATEMENT_TIMEOUT_MS ?? '30000', 10),
  };
}

export function getPool(): Pool {
  if (!pool) {
    const config = buildPoolConfig();
    pool = new Pool(config);

    pool.on('error', (err) => {
      logger.error('Unexpected DB pool error', { error: String(err) });
    });

    pool.on('connect', () => {
      logger.debug('New DB client connected');
    });

    logger.info('DB connection pool created', {
      max: config.max,
      min: config.min,
      idleTimeoutMs: config.idleTimeoutMillis,
      connectionTimeoutMs: config.connectionTimeoutMillis,
      statementTimeoutMs: config.statement_timeout,
    });
  }
  return pool;
}

export async function withClient<T>(fn: (client: PoolClient) => Promise<T>): Promise<T> {
  const client = await getPool().connect();
  try {
    return await fn(client);
  } finally {
    client.release();
  }
}

export async function withTransaction<T>(fn: (client: PoolClient) => Promise<T>): Promise<T> {
  return withClient(async (client) => {
    await client.query('BEGIN');
    try {
      const result = await fn(client);
      await client.query('COMMIT');
      return result;
    } catch (err) {
      await client.query('ROLLBACK');
      throw err;
    }
  });
}

export async function closePool(): Promise<void> {
  if (pool) {
    logger.info('Closing DB connection pool');
    await pool.end();
    pool = null;
  }
}
