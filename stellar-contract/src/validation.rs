pub fn validate_coordinates(latitude: i128, longitude: i128) {
    const MAX_LAT: i128 = 90_000_000;
    const MAX_LON: i128 = 180_000_000;
//! # Validation Utilities — Issue #757
//!
//! Common validation functions extracted into a single reusable module to
//! eliminate code duplication across contract functions.
//!
//! ## Usage
//!
//! ```ignore
//! use crate::validation::{
//!     validate_positive_amount, validate_weight, validate_coordinates,
//!     validate_percentage, validate_addresses_different,
//! };
//!
//! validate_weight(weight, "waste weight");
//! validate_coordinates(latitude, longitude);
//! validate_percentage(collector_pct, "collector_percentage");
//! ```

use soroban_sdk::{Address, Env, String, Vec};

// ── Constants ─────────────────────────────────────────────────────────────────

/// Maximum latitude in microdegrees (90°).
pub const MAX_LAT: i128 = 90_000_000;

/// Maximum longitude in microdegrees (180°).
pub const MAX_LON: i128 = 180_000_000;

/// Maximum waste weight per submission in grams (1 000 000 kg).
pub const MAX_WASTE_WEIGHT: u128 = 1_000_000_000;

/// Minimum waste weight per submission in grams (100 g).
pub const MIN_WASTE_WEIGHT: u128 = 100;

/// Maximum string length for participant display name.
pub const MAX_NAME_LEN: u32 = 64;

/// Maximum note/memo length on transfers.
pub const MAX_NOTE_LEN: u32 = 256;

/// Maximum number of tags per waste item.
pub const MAX_TAGS: u32 = 10;

/// Maximum tag length in characters.
pub const MAX_TAG_LEN: u32 = 32;

// ── Amount validators ─────────────────────────────────────────────────────────

/// Panics if `amount` is not positive (> 0).
///
/// # Parameters
/// * `amount`     — the value to check.
/// * `field_name` — name used in the panic message.
///
/// # Example
/// ```ignore
/// validate_positive_amount(reward_amount, "reward");
/// ```
#[allow(dead_code)]
pub fn validate_positive_amount(amount: i128, field_name: &str) {
    if amount <= 0 {
        panic!("{} must be positive", field_name);
    }
}

/// Panics if `amount` is not positive (u128 variant, rejects zero).
///
/// # Parameters
/// * `amount`     — the value to check.
/// * `field_name` — name used in the panic message.
#[allow(dead_code)]
pub fn validate_positive_u128(amount: u128, field_name: &str) {
    if amount == 0 {
        panic!("{} must be greater than zero", field_name);
    }
}

/// Validates a waste weight in grams.
///
/// Panics if weight is below `MIN_WASTE_WEIGHT` (100 g) or above
/// `MAX_WASTE_WEIGHT` (1 000 000 kg).
///
/// # Parameters
/// * `weight`     — weight in grams.
/// * `field_name` — name used in the panic message.
///
/// # Example
/// ```ignore
/// validate_weight(waste.weight, "waste weight");
/// ```
pub fn validate_weight(weight: u128, field_name: &str) {
    if weight < MIN_WASTE_WEIGHT {
        panic!("{} must be at least {} grams", field_name, MIN_WASTE_WEIGHT);
    }
    if weight > MAX_WASTE_WEIGHT {
        panic!(
            "{} must not exceed {} grams (1 000 000 kg)",
            field_name, MAX_WASTE_WEIGHT
        );
    }
}

/// Validates a non-negative i128 token amount (allows 0).
#[allow(dead_code)]
pub fn validate_non_negative(amount: i128, field_name: &str) {
    if amount < 0 {
        panic!("{} cannot be negative", field_name);
    }
}

// ── Percentage validators ─────────────────────────────────────────────────────

/// Panics if `percentage` is greater than 100.
///
/// # Parameters
/// * `percentage` — value to validate (0–100).
/// * `field_name` — name used in the panic message.
pub fn validate_percentage(percentage: u32, field_name: &str) {
    if percentage > 100 {
        panic!("{} must be <= 100", field_name);
    }
}

/// Validates that collector and owner percentages together do not exceed 100.
///
/// # Parameters
/// * `collector_pct` — collector percentage.
/// * `owner_pct`     — owner percentage.
///
/// # Panics
/// Panics with `"Total percentages cannot exceed 100"` if the sum exceeds 100.
pub fn validate_reward_percentages(collector_pct: u32, owner_pct: u32) {
    if collector_pct + owner_pct > 100 {
        panic!("Total percentages cannot exceed 100");
    }
}

/// Validates a basis-point value (0–10 000 = 0%–100%).
///
/// # Parameters
/// * `bps`        — basis points to validate.
/// * `field_name` — name used in the panic message.
#[allow(dead_code)]
pub fn validate_bps(bps: u32, field_name: &str) {
    if bps > 10_000 {
        panic!("{} must be <= 10 000 basis points", field_name);
    }
}

// ── Coordinate validators ─────────────────────────────────────────────────────

/// Validates WGS-84 coordinates stored as microdegrees.
///
/// * Latitude  must be in `[-90_000_000, 90_000_000]`  (i.e. −90° to +90°).
/// * Longitude must be in `[-180_000_000, 180_000_000]` (i.e. −180° to +180°).
///
/// # Panics
/// Panics with a descriptive message if either value is out of range.
///
/// # Example
/// ```ignore
/// validate_coordinates(52_520_000, 13_405_000); // Berlin, valid
/// validate_coordinates(91_000_000, 0);           // panics
/// ```
pub fn validate_coordinates(latitude: i128, longitude: i128) {
    if !(-MAX_LAT..=MAX_LAT).contains(&latitude) {
        panic!("Latitude must be between -90 and +90 degrees");
    }
    if !(-MAX_LON..=MAX_LON).contains(&longitude) {
        panic!("Longitude must be between -180 and +180 degrees");
    }
}

// ── Address validators ────────────────────────────────────────────────────────

/// Panics if `address` equals the current contract address.
///
/// Use this to prevent participants from registering as the contract itself.
///
/// # Parameters
/// * `env`     — Soroban environment.
/// * `address` — address to check.
pub fn validate_address_not_contract(env: &Env, address: &Address) {
    if address == &env.current_contract_address() {
        panic!("Address cannot be the contract itself");
    }
}

/// Panics if `addr1` equals `addr2`.
///
/// # Parameters
/// * `addr1`   — first address.
/// * `addr2`   — second address.
/// * `context` — description used in the panic message.
///
/// # Example
/// ```ignore
/// validate_addresses_different(&from, &to, "waste transfer");
/// ```
pub fn validate_addresses_different(addr1: &Address, addr2: &Address, context: &str) {
    if addr1 == addr2 {
        panic!("{}: addresses must be different", context);
    }
}

/// Panics if `address` is not present in `allowed`.
///
/// Use to verify a caller is in the admin list or member set.
///
/// # Parameters
/// * `address` — address to check.
/// * `allowed` — list of permitted addresses.
/// * `context` — description used in the panic message.
#[allow(dead_code)]
pub fn validate_address_in_list(address: &Address, allowed: &Vec<Address>, context: &str) {
    if !allowed.contains(address) {
        panic!("{}: address not authorised", context);
    }
}

// ── String validators ─────────────────────────────────────────────────────────

/// Panics if the Soroban `String` is empty.
///
/// # Parameters
/// * `s`          — string to check.
/// * `field_name` — name used in the panic message.
pub fn validate_string_not_empty(s: &String, field_name: &str) {
    if s.len() == 0 {
        panic!("{} must not be empty", field_name);
    }
}

/// Panics if the Soroban `String` exceeds `max_len` characters.
///
/// # Parameters
/// * `s`          — string to check.
/// * `max_len`    — maximum allowed length.
/// * `field_name` — name used in the panic message.
pub fn validate_string_max_len(s: &String, max_len: u32, field_name: &str) {
    if s.len() > max_len {
        panic!("{} must not exceed {} characters", field_name, max_len);
    }
}

/// Convenience: validates both non-empty and max length.
///
/// # Parameters
/// * `s`          — string to check.
/// * `max_len`    — maximum allowed length.
/// * `field_name` — name used in the panic message.
#[allow(dead_code)]
pub fn validate_string(s: &String, max_len: u32, field_name: &str) {
    validate_string_not_empty(s, field_name);
    validate_string_max_len(s, max_len, field_name);
}

// ── Timestamp validators ──────────────────────────────────────────────────────

/// Panics if `timestamp` is not strictly in the future.
///
/// # Parameters
/// * `timestamp`    — the timestamp to validate (Unix seconds).
/// * `current_time` — the current ledger timestamp.
/// * `field_name`   — name used in the panic message.
#[allow(dead_code)]
pub fn validate_future_timestamp(timestamp: u64, current_time: u64, field_name: &str) {
    if timestamp <= current_time {
        panic!("{} must be in the future", field_name);
    }
}

/// Panics if `end` is not strictly after `start`.
///
/// # Parameters
/// * `start` — start timestamp.
/// * `end`   — end timestamp.
#[allow(dead_code)]
pub fn validate_time_range(start: u64, end: u64) {
    if end <= start {
        panic!("End time must be after start time");
    }
}

// ── Collection validators ─────────────────────────────────────────────────────

/// Panics if a `Vec` is empty.
///
/// # Parameters
/// * `len`        — length of the collection.
/// * `field_name` — name used in the panic message.
#[allow(dead_code)]
pub fn validate_not_empty_collection(len: u32, field_name: &str) {
    if len == 0 {
        panic!("{} must not be empty", field_name);
    }
}

/// Panics if a `Vec` length exceeds `max`.
///
/// # Parameters
/// * `len`        — length of the collection.
/// * `max`        — maximum allowed length.
/// * `field_name` — name used in the panic message.
#[allow(dead_code)]
pub fn validate_max_collection_size(len: u32, max: u32, field_name: &str) {
    if len > max {
        panic!("{} must not exceed {} items", field_name, max);
    }
}

// ── Waste-specific composite validators ───────────────────────────────────────

/// Full validation for a waste submission.
///
/// Checks weight range and coordinates in a single call.
///
/// # Parameters
/// * `weight`    — weight in grams.
/// * `latitude`  — latitude in microdegrees.
/// * `longitude` — longitude in microdegrees.
///
/// # Example
/// ```ignore
/// validate_waste_submission(weight, latitude, longitude);
/// ```
pub fn validate_waste_submission(weight: u128, latitude: i128, longitude: i128) {
    validate_weight(weight, "waste weight");
    validate_coordinates(latitude, longitude);
}

/// Full validation for participant registration.
///
/// Checks that the address is not the contract itself and coordinates are valid.
///
/// # Parameters
/// * `env`       — Soroban environment.
/// * `address`   — registering address.
/// * `latitude`  — latitude in microdegrees.
/// * `longitude` — longitude in microdegrees.
pub fn validate_participant_registration(
    env: &Env,
    address: &Address,
    latitude: i128,
    longitude: i128,
) {
    validate_address_not_contract(env, address);
    validate_coordinates(latitude, longitude);
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    // ── Amount tests ──────────────────────────────────────────────────────────

    #[test]
    fn positive_amount_accepts_valid_value() {
        validate_positive_amount(1, "amount");
        validate_positive_amount(i128::MAX, "amount");
    }

    #[test]
    #[should_panic(expected = "amount must be positive")]
    fn positive_amount_rejects_zero() {
        validate_positive_amount(0, "amount");
    }

    #[test]
    #[should_panic(expected = "amount must be positive")]
    fn positive_amount_rejects_negative() {
        validate_positive_amount(-1, "amount");
    }

    #[test]
    fn positive_u128_accepts_valid_value() {
        validate_positive_u128(1, "value");
    }

    #[test]
    #[should_panic(expected = "value must be greater than zero")]
    fn positive_u128_rejects_zero() {
        validate_positive_u128(0, "value");
    }

    // ── Weight tests ──────────────────────────────────────────────────────────

    #[test]
    fn weight_accepts_valid_range() {
        validate_weight(MIN_WASTE_WEIGHT, "weight");
        validate_weight(1_000, "weight");
        validate_weight(MAX_WASTE_WEIGHT, "weight");
    }

    #[test]
    #[should_panic(expected = "must be at least")]
    fn weight_rejects_below_minimum() {
        validate_weight(99, "waste weight");
    }

    #[test]
    #[should_panic(expected = "must not exceed")]
    fn weight_rejects_above_maximum() {
        validate_weight(MAX_WASTE_WEIGHT + 1, "waste weight");
    }

    // ── Percentage tests ──────────────────────────────────────────────────────

    #[test]
    fn percentage_accepts_valid_values() {
        validate_percentage(0, "pct");
        validate_percentage(50, "pct");
        validate_percentage(100, "pct");
    }

    #[test]
    #[should_panic(expected = "pct must be <= 100")]
    fn percentage_rejects_over_100() {
        validate_percentage(101, "pct");
    }

    #[test]
    fn reward_percentages_accepts_valid_split() {
        validate_reward_percentages(30, 50); // 80 total — OK
        validate_reward_percentages(0, 100);
        validate_reward_percentages(50, 50);
    }

    #[test]
    #[should_panic(expected = "Total percentages cannot exceed 100")]
    fn reward_percentages_rejects_over_100() {
        validate_reward_percentages(60, 50);
    }

    // ── Coordinate tests ──────────────────────────────────────────────────────

    #[test]
    fn coordinates_accept_valid_values() {
        validate_coordinates(0, 0);
        validate_coordinates(MAX_LAT, MAX_LON);
        validate_coordinates(-MAX_LAT, -MAX_LON);
        validate_coordinates(52_520_000, 13_405_000); // Berlin
    }

    #[test]
    #[should_panic(expected = "Latitude must be between -90 and +90 degrees")]
    fn coordinates_reject_invalid_latitude() {
        validate_coordinates(91_000_000, 0);
    }

    #[test]
    #[should_panic(expected = "Longitude must be between -180 and +180 degrees")]
    fn coordinates_reject_invalid_longitude() {
        validate_coordinates(0, 181_000_000);
    }

    #[test]
    #[should_panic(expected = "Latitude must be between -90 and +90 degrees")]
    fn coordinates_reject_negative_lat_out_of_range() {
        validate_coordinates(-91_000_000, 0);
    }

    // ── Address tests ─────────────────────────────────────────────────────────

    #[test]
    fn addresses_different_accepts_different_addresses() {
        let env = Env::default();
        let a = soroban_sdk::Address::generate(&env);
        let b = soroban_sdk::Address::generate(&env);
        validate_addresses_different(&a, &b, "transfer");
    }

    #[test]
    #[should_panic(expected = "transfer: addresses must be different")]
    fn addresses_different_rejects_same_address() {
        let env = Env::default();
        let a = soroban_sdk::Address::generate(&env);
        validate_addresses_different(&a, &a, "transfer");
    }

    // ── Composite tests ───────────────────────────────────────────────────────

    #[test]
    fn waste_submission_validates_all_fields() {
        validate_waste_submission(1_000, 52_520_000, 13_405_000);
    }

    #[test]
    #[should_panic(expected = "must be at least")]
    fn waste_submission_rejects_low_weight() {
        validate_waste_submission(10, 0, 0);
    }

    #[test]
    #[should_panic(expected = "Latitude must be between")]
    fn waste_submission_rejects_bad_coordinates() {
        validate_waste_submission(1_000, 999_000_000, 0);
    }

    // ── Timestamp tests ───────────────────────────────────────────────────────

    #[test]
    fn future_timestamp_accepts_future_time() {
        validate_future_timestamp(1_000, 500);
    }

    #[test]
    #[should_panic(expected = "deadline must be in the future")]
    fn future_timestamp_rejects_past() {
        validate_future_timestamp(500, 1_000, "deadline");
    }

    #[test]
    #[should_panic(expected = "deadline must be in the future")]
    fn future_timestamp_rejects_equal() {
        validate_future_timestamp(1_000, 1_000, "deadline");
    }

    // ── String tests ──────────────────────────────────────────────────────────

    #[test]
    fn string_not_empty_accepts_non_empty() {
        let env = Env::default();
        let s = soroban_sdk::String::from_str(&env, "hello");
        validate_string_not_empty(&s, "name");
    }

    #[test]
    #[should_panic(expected = "name must not be empty")]
    fn string_not_empty_rejects_empty() {
        let env = Env::default();
        let s = soroban_sdk::String::from_str(&env, "");
        validate_string_not_empty(&s, "name");
    }
}
