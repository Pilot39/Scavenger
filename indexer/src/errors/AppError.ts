/**
 * Centralized error handling for the indexer
 */

export enum ErrorCode {
  VALIDATION_ERROR = 'VALIDATION_ERROR',
  DATABASE_ERROR = 'DATABASE_ERROR',
  NETWORK_ERROR = 'NETWORK_ERROR',
  NOT_FOUND = 'NOT_FOUND',
  UNAUTHORIZED = 'UNAUTHORIZED',
  INTERNAL_ERROR = 'INTERNAL_ERROR',
}

export class AppError extends Error {
  constructor(
    public code: ErrorCode,
    message: string,
    public statusCode: number = 500,
    public details?: Record<string, unknown>
  ) {
    super(message);
    this.name = 'AppError';
  }

  toJSON() {
    return {
      code: this.code,
      message: this.message,
      statusCode: this.statusCode,
      details: this.details,
    };
  }
}

export class ValidationError extends AppError {
  constructor(message: string, details?: Record<string, unknown>) {
    super(ErrorCode.VALIDATION_ERROR, message, 400, details);
    this.name = 'ValidationError';
  }
}

export class DatabaseError extends AppError {
  constructor(message: string, details?: Record<string, unknown>) {
    super(ErrorCode.DATABASE_ERROR, message, 500, details);
    this.name = 'DatabaseError';
  }
}

export class NotFoundError extends AppError {
  constructor(resource: string, id?: string) {
    const message = id ? `${resource} with id ${id} not found` : `${resource} not found`;
    super(ErrorCode.NOT_FOUND, message, 404);
    this.name = 'NotFoundError';
  }
}

export class NetworkError extends AppError {
  constructor(message: string, details?: Record<string, unknown>) {
    super(ErrorCode.NETWORK_ERROR, message, 503, details);
    this.name = 'NetworkError';
  }
}

export function isAppError(error: unknown): error is AppError {
  return error instanceof AppError;
}

export function handleError(error: unknown): AppError {
  if (isAppError(error)) {
    return error;
  }

  if (error instanceof Error) {
    if (error.message.includes('ECONNREFUSED')) {
      return new NetworkError('Failed to connect to database');
    }
    if (error.message.includes('timeout')) {
      return new NetworkError('Request timeout');
    }
    return new AppError(ErrorCode.INTERNAL_ERROR, error.message, 500);
  }

  return new AppError(ErrorCode.INTERNAL_ERROR, 'Unknown error occurred', 500);
}
