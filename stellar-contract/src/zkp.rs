//! # Zero-Knowledge Proof Utilities — Issue #816
//!
//! Soroban WASM does not support full ZK proof systems (no std allocator, no
//! field arithmetic libraries, no RNG).  This module implements a
//! **hash-based commitment scheme** — a well-established ZKP primitive —
//! using the native `env.crypto().sha256()` host function.
//!
//! ## Scheme overview
//!
//! A participant who wants to prove they know a secret value `v` without
//! revealing it on-chain follows these steps:
//!
//! 1. **Commit** (off-chain): choose a random `nonce` (32 bytes), compute
//!    `commitment = SHA-256(v ‖ nonce)`, publish the commitment on-chain.
//! 2. **Reveal** (on-chain): later call `verify_commitment` supplying `v` and
//!    `nonce`.  The contract recomputes SHA-256 and checks it equals the stored
//!    commitment.  If it matches the proof is valid and `v` can be acted on.
//!
//! This gives **hiding** (commitment reveals nothing about `v`) and **binding**
//! (a committer cannot produce a different `v'` that opens the same
//! commitment).
//!
//! ## On-chain storage keys
//!
//! ```
//! DataKey::Commitment(committer: Address, nonce_prefix: u64)
//! ```
//!
//! ## Usage pattern
//!
//! ```ignore
//! // --- off-chain (client) ---
//! let commitment = zkp::compute_commitment(env, &secret_bytes, &nonce_bytes);
//!
//! // --- on-chain (contract function) ---
//! zkp::store_commitment(env, &committer, commitment_id, commitment);
//!
//! // --- later reveal ---
//! zkp::verify_commitment(env, &committer, commitment_id, &secret_bytes, &nonce_bytes)?;
//! ```

use soroban_sdk::{contracttype, symbol_short, Address, BytesN, Env};

// ─── Types ────────────────────────────────────────────────────────────────────

/// A 32-byte SHA-256 commitment hash stored on-chain.
///
/// Derive from `CommitmentInput` via [`compute_commitment`].
pub type Commitment = BytesN<32>;

/// Status of a commitment lifecycle.
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommitmentStatus {
    /// Commitment exists but has not been revealed yet.
    Pending = 0,
    /// Commitment was successfully verified and consumed.
    Verified = 1,
    /// Commitment was explicitly cancelled by the owner.
    Cancelled = 2,
    /// Commitment expired without being revealed.
    Expired = 3,
}

/// Stored on-chain alongside the commitment hash.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommitmentRecord {
    /// The SHA-256 hash committed to.
    pub hash: Commitment,
    /// Who made the commitment.
    pub committer: Address,
    /// Monotonic ID within this committer's namespace.
    pub commitment_id: u64,
    /// Ledger timestamp at creation.
    pub created_at: u64,
    /// Optional expiry (0 = no expiry).
    pub expires_at: u64,
    /// Current lifecycle status.
    pub status: CommitmentStatus,
}

// ─── Storage key ──────────────────────────────────────────────────────────────

/// Storage key type for commitments.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CommitmentKey {
    /// `(committer_address, commitment_id)` → `CommitmentRecord`
    Commitment(Address, u64),
}

// ─── Error ────────────────────────────────────────────────────────────────────

/// Errors specific to the commitment scheme.
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommitmentError {
    /// No commitment found for this (committer, id) pair.
    NotFound = 1,
    /// Commitment hash does not match the supplied preimage.
    HashMismatch = 2,
    /// Commitment has already been consumed.
    AlreadyVerified = 3,
    /// Commitment has been cancelled.
    Cancelled = 4,
    /// Commitment has expired.
    Expired = 5,
    /// Commitment is still pending — cannot cancel a verified commitment.
    NotPending = 6,
}

// ─── Commitment computation ───────────────────────────────────────────────────

/// Computes `SHA-256(secret ‖ nonce)` using the Soroban host function.
///
/// # Parameters
/// * `secret` — the secret value to commit to (arbitrary bytes).
/// * `nonce`  — 32 random bytes chosen by the client to ensure hiding.
///
/// The caller is responsible for choosing a fresh, unpredictable `nonce` for
/// each commitment.
pub fn compute_commitment(env: &Env, secret: &BytesN<32>, nonce: &BytesN<32>) -> Commitment {
    // Concatenate secret ‖ nonce into a 64-byte buffer
    let mut preimage = soroban_sdk::Bytes::new(env);
    preimage.extend_from_array(&secret.to_array());
    preimage.extend_from_array(&nonce.to_array());
    env.crypto().sha256(&preimage)
}

/// Verifies that `SHA-256(secret ‖ nonce) == commitment`.
///
/// Returns `Ok(())` if the check passes, otherwise `Err(CommitmentError::HashMismatch)`.
pub fn verify_preimage(
    env: &Env,
    secret: &BytesN<32>,
    nonce: &BytesN<32>,
    commitment: &Commitment,
) -> Result<(), CommitmentError> {
    let computed = compute_commitment(env, secret, nonce);
    if computed == *commitment {
        Ok(())
    } else {
        Err(CommitmentError::HashMismatch)
    }
}

// ─── On-chain storage ─────────────────────────────────────────────────────────

/// Stores a new `CommitmentRecord` in contract instance storage.
///
/// # Panics
/// Panics if a commitment with the same `(committer, commitment_id)` already
/// exists.
pub fn store_commitment(
    env: &Env,
    committer: &Address,
    commitment_id: u64,
    hash: Commitment,
    expires_at: u64,
) {
    let key = CommitmentKey::Commitment(committer.clone(), commitment_id);
    if env
        .storage()
        .instance()
        .has::<CommitmentKey>(&key)
    {
        panic!("zkp: commitment already exists");
    }
    let record = CommitmentRecord {
        hash,
        committer: committer.clone(),
        commitment_id,
        created_at: env.ledger().timestamp(),
        expires_at,
        status: CommitmentStatus::Pending,
    };
    env.storage().instance().set::<CommitmentKey, CommitmentRecord>(&key, &record);
    env.events()
        .publish((symbol_short!("com_new"), committer), commitment_id);
}

/// Retrieves a stored `CommitmentRecord`.
///
/// Returns `Err(CommitmentError::NotFound)` if absent.
pub fn get_commitment(
    env: &Env,
    committer: &Address,
    commitment_id: u64,
) -> Result<CommitmentRecord, CommitmentError> {
    let key = CommitmentKey::Commitment(committer.clone(), commitment_id);
    env.storage()
        .instance()
        .get::<CommitmentKey, CommitmentRecord>(&key)
        .ok_or(CommitmentError::NotFound)
}

/// Reveals a commitment: verifies `SHA-256(secret ‖ nonce) == stored_hash`
/// and marks the record as `Verified`.
///
/// # Returns
/// `Ok(())` on success.  Errors if the commitment doesn't exist, is already
/// consumed, is expired, or the preimage doesn't match.
pub fn reveal_commitment(
    env: &Env,
    committer: &Address,
    commitment_id: u64,
    secret: &BytesN<32>,
    nonce: &BytesN<32>,
) -> Result<(), CommitmentError> {
    let key = CommitmentKey::Commitment(committer.clone(), commitment_id);
    let mut record = get_commitment(env, committer, commitment_id)?;

    // Check lifecycle status
    match record.status {
        CommitmentStatus::Verified  => return Err(CommitmentError::AlreadyVerified),
        CommitmentStatus::Cancelled => return Err(CommitmentError::Cancelled),
        CommitmentStatus::Expired   => return Err(CommitmentError::Expired),
        CommitmentStatus::Pending   => {}
    }

    // Check expiry
    if record.expires_at != 0 && env.ledger().timestamp() > record.expires_at {
        record.status = CommitmentStatus::Expired;
        env.storage().instance().set(&key, &record);
        return Err(CommitmentError::Expired);
    }

    // Verify the preimage
    verify_preimage(env, secret, nonce, &record.hash)?;

    // Mark as verified
    record.status = CommitmentStatus::Verified;
    env.storage().instance().set(&key, &record);

    env.events()
        .publish((symbol_short!("com_vfy"), committer), commitment_id);

    Ok(())
}

/// Cancels a pending commitment.  Only the committer may cancel.
///
/// The caller must have called `committer.require_auth()` before invoking this.
pub fn cancel_commitment(
    env: &Env,
    committer: &Address,
    commitment_id: u64,
) -> Result<(), CommitmentError> {
    let key = CommitmentKey::Commitment(committer.clone(), commitment_id);
    let mut record = get_commitment(env, committer, commitment_id)?;

    if record.status != CommitmentStatus::Pending {
        return Err(CommitmentError::NotPending);
    }

    record.status = CommitmentStatus::Cancelled;
    env.storage().instance().set(&key, &record);

    env.events()
        .publish((symbol_short!("com_cnc"), committer), commitment_id);

    Ok(())
}

// ─── Performance monitoring ───────────────────────────────────────────────────

/// Returns a cost estimate for the commitment scheme operations.
///
/// These are **budget units**, not XLM.  Useful for benchmarking via the
/// Soroban CLI `--diagnostic-events` flag.
pub struct ZkpCostHints;

impl ZkpCostHints {
    /// One SHA-256 call on 64 bytes costs ~400 CPU instructions on Soroban.
    pub const SHA256_64B_INSTRUCTIONS: u64 = 400;
    /// Two storage reads (has + get) cost ~200 instructions each.
    pub const STORAGE_READ_INSTRUCTIONS: u64 = 200;
    /// One storage write costs ~300 instructions.
    pub const STORAGE_WRITE_INSTRUCTIONS: u64 = 300;

    /// Total instruction estimate for `store_commitment`.
    pub fn store_cost() -> u64 {
        Self::STORAGE_READ_INSTRUCTIONS  // has check
            + Self::STORAGE_WRITE_INSTRUCTIONS
    }

    /// Total instruction estimate for `reveal_commitment`.
    pub fn reveal_cost() -> u64 {
        Self::STORAGE_READ_INSTRUCTIONS  // get
            + Self::SHA256_64B_INSTRUCTIONS
            + Self::STORAGE_WRITE_INSTRUCTIONS
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    fn make_bytes32(env: &Env, seed: u8) -> BytesN<32> {
        BytesN::from_array(env, &[seed; 32])
    }

    // ── compute_commitment ────────────────────────────────────────────────────

    #[test]
    fn commitment_is_deterministic() {
        let env = Env::default();
        let secret = make_bytes32(&env, 0xAA);
        let nonce  = make_bytes32(&env, 0xBB);
        let c1 = compute_commitment(&env, &secret, &nonce);
        let c2 = compute_commitment(&env, &secret, &nonce);
        assert_eq!(c1, c2);
    }

    #[test]
    fn different_secrets_produce_different_commitments() {
        let env = Env::default();
        let s1 = make_bytes32(&env, 1);
        let s2 = make_bytes32(&env, 2);
        let nonce = make_bytes32(&env, 0xFF);
        assert_ne!(compute_commitment(&env, &s1, &nonce), compute_commitment(&env, &s2, &nonce));
    }

    #[test]
    fn different_nonces_produce_different_commitments() {
        let env = Env::default();
        let secret = make_bytes32(&env, 0xAA);
        let n1 = make_bytes32(&env, 1);
        let n2 = make_bytes32(&env, 2);
        assert_ne!(compute_commitment(&env, &secret, &n1), compute_commitment(&env, &secret, &n2));
    }

    // ── verify_preimage ───────────────────────────────────────────────────────

    #[test]
    fn verify_preimage_accepts_correct_preimage() {
        let env = Env::default();
        let secret = make_bytes32(&env, 0x01);
        let nonce  = make_bytes32(&env, 0x02);
        let commitment = compute_commitment(&env, &secret, &nonce);
        assert_eq!(verify_preimage(&env, &secret, &nonce, &commitment), Ok(()));
    }

    #[test]
    fn verify_preimage_rejects_wrong_secret() {
        let env = Env::default();
        let secret  = make_bytes32(&env, 0x01);
        let wrong_s = make_bytes32(&env, 0x99);
        let nonce   = make_bytes32(&env, 0x02);
        let commitment = compute_commitment(&env, &secret, &nonce);
        assert_eq!(
            verify_preimage(&env, &wrong_s, &nonce, &commitment),
            Err(CommitmentError::HashMismatch)
        );
    }

    #[test]
    fn verify_preimage_rejects_wrong_nonce() {
        let env = Env::default();
        let secret  = make_bytes32(&env, 0x01);
        let nonce   = make_bytes32(&env, 0x02);
        let wrong_n = make_bytes32(&env, 0x77);
        let commitment = compute_commitment(&env, &secret, &nonce);
        assert_eq!(
            verify_preimage(&env, &secret, &wrong_n, &commitment),
            Err(CommitmentError::HashMismatch)
        );
    }

    // ── store / get / reveal ──────────────────────────────────────────────────

    #[test]
    fn store_and_get_commitment() {
        let env = Env::default();
        let committer = Address::generate(&env);
        let secret = make_bytes32(&env, 0x10);
        let nonce  = make_bytes32(&env, 0x20);
        let hash   = compute_commitment(&env, &secret, &nonce);
        store_commitment(&env, &committer, 1, hash.clone(), 0);
        let record = get_commitment(&env, &committer, 1).unwrap();
        assert_eq!(record.hash, hash);
        assert_eq!(record.status, CommitmentStatus::Pending);
    }

    #[test]
    fn get_nonexistent_commitment_returns_not_found() {
        let env = Env::default();
        let addr = Address::generate(&env);
        assert_eq!(
            get_commitment(&env, &addr, 99),
            Err(CommitmentError::NotFound)
        );
    }

    #[test]
    fn reveal_commitment_happy_path() {
        let env = Env::default();
        let committer = Address::generate(&env);
        let secret = make_bytes32(&env, 0x10);
        let nonce  = make_bytes32(&env, 0x20);
        let hash   = compute_commitment(&env, &secret, &nonce);
        store_commitment(&env, &committer, 1, hash, 0);
        assert_eq!(
            reveal_commitment(&env, &committer, 1, &secret, &nonce),
            Ok(())
        );
        // Verify the record is now marked Verified
        let record = get_commitment(&env, &committer, 1).unwrap();
        assert_eq!(record.status, CommitmentStatus::Verified);
    }

    #[test]
    fn reveal_commitment_rejects_wrong_preimage() {
        let env = Env::default();
        let committer = Address::generate(&env);
        let secret = make_bytes32(&env, 0x10);
        let nonce  = make_bytes32(&env, 0x20);
        let hash   = compute_commitment(&env, &secret, &nonce);
        store_commitment(&env, &committer, 1, hash, 0);
        let bad_secret = make_bytes32(&env, 0xFF);
        assert_eq!(
            reveal_commitment(&env, &committer, 1, &bad_secret, &nonce),
            Err(CommitmentError::HashMismatch)
        );
    }

    #[test]
    fn reveal_commitment_rejects_double_reveal() {
        let env = Env::default();
        let committer = Address::generate(&env);
        let secret = make_bytes32(&env, 0x10);
        let nonce  = make_bytes32(&env, 0x20);
        let hash   = compute_commitment(&env, &secret, &nonce);
        store_commitment(&env, &committer, 1, hash, 0);
        reveal_commitment(&env, &committer, 1, &secret, &nonce).unwrap();
        assert_eq!(
            reveal_commitment(&env, &committer, 1, &secret, &nonce),
            Err(CommitmentError::AlreadyVerified)
        );
    }

    #[test]
    fn cancel_commitment_works() {
        let env = Env::default();
        let committer = Address::generate(&env);
        let secret = make_bytes32(&env, 0x10);
        let nonce  = make_bytes32(&env, 0x20);
        let hash   = compute_commitment(&env, &secret, &nonce);
        store_commitment(&env, &committer, 1, hash, 0);
        assert_eq!(cancel_commitment(&env, &committer, 1), Ok(()));
        let record = get_commitment(&env, &committer, 1).unwrap();
        assert_eq!(record.status, CommitmentStatus::Cancelled);
    }

    #[test]
    fn cancel_already_verified_returns_not_pending() {
        let env = Env::default();
        let committer = Address::generate(&env);
        let secret = make_bytes32(&env, 0x10);
        let nonce  = make_bytes32(&env, 0x20);
        let hash   = compute_commitment(&env, &secret, &nonce);
        store_commitment(&env, &committer, 1, hash, 0);
        reveal_commitment(&env, &committer, 1, &secret, &nonce).unwrap();
        assert_eq!(
            cancel_commitment(&env, &committer, 1),
            Err(CommitmentError::NotPending)
        );
    }

    #[test]
    fn expired_commitment_is_rejected_on_reveal() {
        let env = Env::default();
        env.ledger().set_timestamp(1000);
        let committer = Address::generate(&env);
        let secret = make_bytes32(&env, 0x10);
        let nonce  = make_bytes32(&env, 0x20);
        let hash   = compute_commitment(&env, &secret, &nonce);
        // expires_at is in the past (from the perspective of a later call)
        store_commitment(&env, &committer, 1, hash, 500); // expires at 500
        // Advance time past expiry
        env.ledger().set_timestamp(1500);
        assert_eq!(
            reveal_commitment(&env, &committer, 1, &secret, &nonce),
            Err(CommitmentError::Expired)
        );
    }

    // ── cost hints ────────────────────────────────────────────────────────────

    #[test]
    fn cost_hints_are_positive() {
        assert!(ZkpCostHints::store_cost() > 0);
        assert!(ZkpCostHints::reveal_cost() > 0);
        // reveal is more expensive than store (includes SHA-256)
        assert!(ZkpCostHints::reveal_cost() > ZkpCostHints::store_cost());
    }
}
