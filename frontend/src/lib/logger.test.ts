import { describe, it, expect, beforeEach } from 'vitest';
import { logger } from './logger';

describe('Logger', () => {
  beforeEach(() => {
    logger.clearLogs();
    logger.setLevel('debug');
  });

  it('should log debug messages', () => {
    logger.debug('Debug message', { key: 'value' });
    const logs = logger.getLogs('debug');
    expect(logs.length).toBeGreaterThan(0);
    expect(logs[logs.length - 1].level).toBe('debug');
  });

  it('should log info messages', () => {
    logger.info('Info message');
    const logs = logger.getLogs('info');
    expect(logs.length).toBeGreaterThan(0);
  });

  it('should log warn messages', () => {
    logger.warn('Warning message', { warning: true });
    const logs = logger.getLogs('warn');
    expect(logs.length).toBeGreaterThan(0);
  });

  it('should log error messages', () => {
    const error = new Error('Test error');
    logger.error('Error occurred', error, { errorCode: 500 });
    const logs = logger.getLogs('error');
    expect(logs.length).toBeGreaterThan(0);
    expect(logs[logs.length - 1].error).toBe(error);
  });

  it('should respect log level filtering', () => {
    logger.setLevel('warn');
    logger.debug('Debug');
    logger.info('Info');
    logger.warn('Warn');
    const logs = logger.getLogs();
    expect(logs.every((log) => log.level !== 'debug' && log.level !== 'info')).toBe(true);
  });

  it('should clear logs', () => {
    logger.info('Message');
    expect(logger.getLogs().length).toBeGreaterThan(0);
    logger.clearLogs();
    expect(logger.getLogs().length).toBe(0);
  });

  it('should include context in log entries', () => {
    const context = { userId: '123', action: 'submit' };
    logger.info('User action', context);
    const logs = logger.getLogs();
    expect(logs[logs.length - 1].context).toEqual(context);
  });
});
