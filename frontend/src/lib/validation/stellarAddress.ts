import { z } from 'zod'

/** Stellar public key: 56 chars, starts with G. */
export const stellarAddressSchema = z
  .string()
  .regex(/^G[A-Z2-7]{55}$/, 'Invalid Stellar address.')

export function isValidStellarAddress(address: string): boolean {
  return stellarAddressSchema.safeParse(address).success
}
