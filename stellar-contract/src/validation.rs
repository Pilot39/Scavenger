use soroban_sdk::Address;

pub fn validate_positive_amount(amount: i128, field_name: &str) {
    if amount <= 0 {
        panic!("{} must be greater than zero", field_name);
    }
}

pub fn validate_positive_u128(amount: u128, field_name: &str) {
    if amount == 0 {
        panic!("{} must be greater than zero", field_name);
    }
}

/// Validates a waste/material weight: must be non-zero and within `max`.
pub fn validate_weight(weight: u128, max: u128) {
    validate_positive_u128(weight, "Waste weight");
    if weight > max {
        panic!("Waste weight exceeds maximum allowed");
    }
}

/// Validates that two reward-distribution percentages don't sum past 100.
pub fn validate_percentage_sum(collector_percentage: u32, owner_percentage: u32) {
    if collector_percentage + owner_percentage > 100 {
        panic!("Total percentages cannot exceed 100");
    }
}

pub fn validate_coordinates(latitude: i128, longitude: i128) {
    const MAX_LAT: i128 = 90_000_000;
    const MAX_LON: i128 = 180_000_000;

    if latitude < -MAX_LAT || latitude > MAX_LAT {
        panic!("Latitude must be between -90 and +90 degrees");
    }

    if longitude < -MAX_LON || longitude > MAX_LON {
        panic!("Longitude must be between -180 and +180 degrees");
    }
}

pub fn validate_addresses_different(addr1: &Address, addr2: &Address, context: &str) {
    if addr1 == addr2 {
        panic!("{}: addresses must be different", context);
    }
}
