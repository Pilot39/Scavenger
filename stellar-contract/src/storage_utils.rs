//! Storage utility helpers for the Scavngr contract.

use soroban_sdk::Env;

/// Bump the instance storage TTL so it remains alive for at least another
/// `INSTANCE_LIFETIME_THRESHOLD` ledgers.
///
/// Call this at the start of every externally-invokable contract function to
/// prevent the instance storage from expiring during normal usage.
pub fn bump_instance(env: &Env) {
    // Keep instance alive for ~30 days (at 5 s/ledger, 30d ≈ 518_400 ledgers).
    const INSTANCE_LIFETIME_THRESHOLD: u32 = 518_400;
    const INSTANCE_BUMP_AMOUNT: u32 = 518_400;
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}
