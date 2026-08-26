import { describe, it, expect } from 'vitest';
import {
  AppError,
  ValidationError,
  DatabaseError,
  NotFoundError,
  NetworkError,
  ErrorCode,
  isAppError,
  handleError,
} from '../src/errors';

describe('Error Handling', () => {
  it('should create validation error', () => {
    const error = new ValidationError('Invalid input', { field: 'email' });
    expect(error.code).toBe(ErrorCode.VALIDATION_ERROR);
    expect(error.statusCode).toBe(400);
    expect(error.details).toEqual({ field: 'email' });
  });

  it('should create database error', () => {
    const error = new DatabaseError('Connection failed');
    expect(error.code).toBe(ErrorCode.DATABASE_ERROR);
    expect(error.statusCode).toBe(500);
  });

  it('should create not found error', () => {
    const error = new NotFoundError('Participant', 'ABC123');
    expect(error.code).toBe(ErrorCode.NOT_FOUND);
    expect(error.statusCode).toBe(404);
    expect(error.message).toContain('ABC123');
  });

  it('should create network error', () => {
    const error = new NetworkError('Connection timeout');
    expect(error.code).toBe(ErrorCode.NETWORK_ERROR);
    expect(error.statusCode).toBe(503);
  });

  it('should identify app errors', () => {
    const appError = new ValidationError('Test');
    const regularError = new Error('Test');

    expect(isAppError(appError)).toBe(true);
    expect(isAppError(regularError)).toBe(false);
  });

  it('should handle regular errors', () => {
    const error = new Error('Connection refused');
    const handled = handleError(error);

    expect(isAppError(handled)).toBe(true);
    expect(handled.code).toBe(ErrorCode.NETWORK_ERROR);
  });

  it('should handle unknown errors', () => {
    const handled = handleError('unknown');
    expect(handled.code).toBe(ErrorCode.INTERNAL_ERROR);
  });

  it('should serialize to JSON', () => {
    const error = new ValidationError('Test error', { field: 'name' });
    const json = error.toJSON();

    expect(json.code).toBe(ErrorCode.VALIDATION_ERROR);
    expect(json.statusCode).toBe(400);
    expect(json.message).toBe('Test error');
  });
});
