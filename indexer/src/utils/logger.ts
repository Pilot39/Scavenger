/**
 * Structured JSON logger for the Scavngr indexer.
 * Outputs JSON lines compatible with Filebeat / Logstash.
 */

export type LogLevel = 'debug' | 'info' | 'warn' | 'error';

const LEVELS: Record<LogLevel, number> = {
  debug: 10,
  info:  20,
  warn:  30,
  error: 40,
};

const MIN_LEVEL: LogLevel =
  (process.env.LOG_LEVEL as LogLevel | undefined) ?? 'info';

function shouldLog(level: LogLevel): boolean {
  return LEVELS[level] >= LEVELS[MIN_LEVEL];
}

function write(level: LogLevel, message: string, fields?: Record<string, unknown>): void {
  if (!shouldLog(level)) return;

  const entry = {
    timestamp: new Date().toISOString(),
    level: level.toUpperCase(),
    service: 'indexer',
    message,
    ...fields,
  };

  const line = JSON.stringify(entry);
  if (level === 'error') {
    process.stderr.write(line + '\n');
  } else {
    process.stdout.write(line + '\n');
  }
}

export const logger = {
  debug: (message: string, fields?: Record<string, unknown>) => write('debug', message, fields),
  info:  (message: string, fields?: Record<string, unknown>) => write('info',  message, fields),
  warn:  (message: string, fields?: Record<string, unknown>) => write('warn',  message, fields),
  error: (message: string, fields?: Record<string, unknown>) => write('error', message, fields),
};
