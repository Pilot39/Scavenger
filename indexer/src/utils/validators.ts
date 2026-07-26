/**
 * Common validation utilities
 */

export function isValidAddress(address: string): boolean {
  return typeof address === 'string' && address.length > 0 && /^[A-Z0-9]+$/.test(address);
}

export function isValidWasteType(wasteType: string): boolean {
  const validTypes = ['Paper', 'Plastic', 'Metal', 'Glass', 'Organic', 'Electronic'];
  return validTypes.includes(wasteType);
}

export function isValidRole(role: string): boolean {
  const validRoles = ['Recycler', 'Collector', 'Manufacturer'];
  return validRoles.includes(role);
}

export function isValidCoordinate(value: unknown): boolean {
  const num = Number(value);
  return !isNaN(num) && isFinite(num);
}

export function isValidWeight(weight: unknown): boolean {
  const num = Number(weight);
  return num > 0 && isFinite(num);
}

export function isValidLimit(limit: unknown, max = 1000): boolean {
  const num = Number(limit);
  return Number.isInteger(num) && num > 0 && num <= max;
}
