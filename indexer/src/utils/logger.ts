import { StructuredLogger } from '../../shared/src/logger';
import type { LogLevel } from '../../shared/src/logger';

export const logger = new StructuredLogger(
  (process.env.LOG_LEVEL as LogLevel | undefined) ?? 'info',
  true,
);

export type { LogLevel };