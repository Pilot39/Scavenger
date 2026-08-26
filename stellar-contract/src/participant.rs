//! Participant module (issue #759)
//!
//! Encapsulates all helpers and constants related to participant management so
//! that `lib.rs` can delegate validation, lookup, and update logic here instead
//! of inlining it across thousands of lines.
//!
//! # Responsibilities
//! - Participant registration / deregistration helpers
//! - Role-based permission checks
//! - Reputation score helpers
//! - Location validation

use soroban_sdk::{Address, Env, Symbol};

use crate::errors::Error;
use crate::types::{CertificationLevel, Participant, ParticipantRole, ParticipantTier};

// Storage key prefix for participant records (matches lib.rs usage).
pub const PARTICIPANT_PREFIX: &str = "PART";

/// Returns `true` if `address` is currently registered and active.
pub fn is_registered(env: &Env, address: &Address) -> bool {
    let key = participant_key(env, address);
    if let Some(p) = env
        .storage()
        .persistent()
        .get::<(Symbol, Address), Participant>(&key)
    {
        p.is_registered
    } else {
        false
    }
}

/// Loads a participant or returns `Error::NotRegistered`.
pub fn require_participant(env: &Env, address: &Address) -> Result<Participant, Error> {
    let key = participant_key(env, address);
    env.storage()
        .persistent()
        .get::<(Symbol, Address), Participant>(&key)
        .filter(|p| p.is_registered)
        .ok_or(Error::NotRegistered)
}

/// Persists updated participant data.
pub fn save_participant(env: &Env, participant: &Participant) {
    let key = participant_key(env, &participant.address);
    env.storage().persistent().set(&key, participant);
}

/// Validates GPS coordinates (microdegrees).
/// Latitude ∈ [-90_000_000, +90_000_000]
/// Longitude ∈ [-180_000_000, +180_000_000]
pub fn validate_coordinates(lat: i128, lon: i128) -> Result<(), Error> {
    if lat < -90_000_000 || lat > 90_000_000 || lon < -180_000_000 || lon > 180_000_000 {
        return Err(Error::InvalidCoordinates);
    }
    Ok(())
}

/// Derives the `CertificationLevel` from total waste processed (in grams).
pub fn certification_from_weight(total_grams: u128) -> CertificationLevel {
    let kg = total_grams / 1_000;
    match kg {
        0..=999 => CertificationLevel::Beginner,
        1_000..=9_999 => CertificationLevel::Intermediate,
        10_000..=99_999 => CertificationLevel::Advanced,
        _ => CertificationLevel::Expert,
    }
}

/// Derives the `ParticipantTier` from total reward tokens earned.
pub fn tier_from_tokens(total_tokens: u128) -> ParticipantTier {
    match total_tokens {
        0..=999 => ParticipantTier::Bronze,
        1_000..=9_999 => ParticipantTier::Silver,
        10_000..=99_999 => ParticipantTier::Gold,
        _ => ParticipantTier::Platinum,
    }
}

/// Checks that `role` is allowed to submit waste.
/// Recyclers and Collectors may submit; Manufacturers may not.
pub fn can_submit_waste(role: ParticipantRole) -> bool {
    matches!(role, ParticipantRole::Recycler | ParticipantRole::Collector)
}

// ── Private helpers ───────────────────────────────────────────────────────────

/// Constructs the persistent-storage key for a participant.
/// Mirrors the key scheme used in `lib.rs`: `("PART", address)`.
fn participant_key(env: &Env, address: &Address) -> (Symbol, Address) {
    (Symbol::new(env, PARTICIPANT_PREFIX), address.clone())
}
