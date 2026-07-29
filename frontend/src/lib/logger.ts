import { config } from '../config/app';

export type LogLevel = 'debug' | 'info' | 'warn' | 'error';

interface LogEntry {
  timestamp: string;
  level: LogLevel;
  message: string;
  context?: Record<string, unknown>;
  error?: Error;
}

const LOG_LEVELS: Record<LogLevel, number> = {
  debug: 0,
  info: 1,
  warn: 2,
  error: 3,
};

class Logger {
  private level: LogLevel;
  private enableConsole: boolean;
  private logs: LogEntry[] = [];

  constructor(level: LogLevel = 'info', enableConsole: boolean = true) {
    this.level = level;
    this.enableConsole = enableConsole;
  }

  private shouldLog(level: LogLevel): boolean {
    return LOG_LEVELS[level] >= LOG_LEVELS[this.level];
  }

  private formatEntry(entry: LogEntry): string {
    const { timestamp, level, message, context } = entry;
    const contextStr = context ? ` ${JSON.stringify(context)}` : '';
    return `[${timestamp}] ${level.toUpperCase()}: ${message}${contextStr}`;
  }

  private createEntry(level: LogLevel, message: string, context?: Record<string, unknown>, error?: Error): LogEntry {
    return {
      timestamp: new Date().toISOString(),
      level,
      message,
      context,
      error,
    };
  }

  debug(message: string, context?: Record<string, unknown>): void {
    if (this.shouldLog('debug')) {
      const entry = this.createEntry('debug', message, context);
      this.logs.push(entry);
    }
  }

  info(message: string, context?: Record<string, unknown>): void {
    if (this.shouldLog('info')) {
      const entry = this.createEntry('info', message, context);
      this.logs.push(entry);
    }
  }

  warn(message: string, context?: Record<string, unknown>): void {
    if (this.shouldLog('warn')) {
      const entry = this.createEntry('warn', message, context);
      this.logs.push(entry);
      if (this.enableConsole) console.warn(this.formatEntry(entry));
    }
  }

  error(message: string, error?: Error, context?: Record<string, unknown>): void {
    if (this.shouldLog('error')) {
      const entry = this.createEntry('error', message, context, error);
      this.logs.push(entry);
      if (this.enableConsole) {
        console.error(this.formatEntry(entry));
        if (error) console.error(error);
      }
    }
  }

  getLogs(level?: LogLevel): LogEntry[] {
    if (!level) return this.logs;
    return this.logs.filter((log) => LOG_LEVELS[log.level] >= LOG_LEVELS[level]);
  }

  clearLogs(): void {
    this.logs = [];
  }

  setLevel(level: LogLevel): void {
    this.level = level;
  }
}

export const logger = new Logger(config.logging.level, config.logging.enableConsole);
