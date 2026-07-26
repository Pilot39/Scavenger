import { describe, it, expect } from 'vitest';
import {
  isValidAddress,
  isValidWasteType,
  isValidRole,
  isValidWeight,
  formatAddress,
  formatWeight,
  formatDate,
  truncateString,
} from '../src/utils';

describe('Utility Functions', () => {
  describe('Validators', () => {
    it('should validate addresses', () => {
      expect(isValidAddress('GBUQWP3BOUZX34LOCALBTJVYOUSBTCHAU4YX5LK4EVRCNRANEA2AR5G')).toBe(true);
      expect(isValidAddress('')).toBe(false);
      expect(isValidAddress('invalid-address')).toBe(false);
    });

    it('should validate waste types', () => {
      expect(isValidWasteType('Paper')).toBe(true);
      expect(isValidWasteType('Plastic')).toBe(true);
      expect(isValidWasteType('Invalid')).toBe(false);
    });

    it('should validate roles', () => {
      expect(isValidRole('Recycler')).toBe(true);
      expect(isValidRole('Collector')).toBe(true);
      expect(isValidRole('Manufacturer')).toBe(true);
      expect(isValidRole('Admin')).toBe(false);
    });

    it('should validate weight', () => {
      expect(isValidWeight(10)).toBe(true);
      expect(isValidWeight(0)).toBe(false);
      expect(isValidWeight(-5)).toBe(false);
    });
  });

  describe('Formatters', () => {
    it('should format address', () => {
      const addr = 'GBUQWP3BOUZX34LOCALBTJVYOUSBTCHAU4YX5LK4EVRCNRANEA2AR5G';
      const formatted = formatAddress(addr);
      expect(formatted).toMatch(/^GBU.*AR5G$/);
    });

    it('should format weight', () => {
      expect(formatWeight(10.5)).toBe('10.50 kg');
      expect(formatWeight('20')).toBe('20.00 kg');
    });

    it('should format date', () => {
      const date = new Date('2024-01-15');
      const formatted = formatDate(date);
      expect(formatted).toContain('Jan');
      expect(formatted).toContain('15');
    });

    it('should truncate strings', () => {
      expect(truncateString('Hello World', 5)).toBe('Hello...');
      expect(truncateString('Hi', 5)).toBe('Hi');
    });
  });
});
