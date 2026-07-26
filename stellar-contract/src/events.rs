use soroban_sdk::{symbol_short, Address, Env, Symbol};

use crate::types::{ParticipantRole, WasteType};

const WASTE_REGISTERED: Symbol = symbol_short!("recycled");
const DONATION_MADE: Symbol = symbol_short!("donated");
const WASTE_TRANSFERRED: Symbol = symbol_short!("transfer");
const WASTE_CONFIRMED: Symbol = symbol_short!("confirmed");
const PARTICIPANT_REGISTERED: Symbol = symbol_short!("reg");
const TOKENS_REWARDED: Symbol = symbol_short!("rewarded");
const PARTICIPANT_LOCATION_UPDATED: Symbol = symbol_short!("loc_upd");
const ADMIN_TRANSFERRED: Symbol = symbol_short!("adm_xfr");
const CONTRACT_PAUSED: Symbol = symbol_short!("paused");
const CONTRACT_UNPAUSED: Symbol = symbol_short!("unpaused");
const INCENTIVE_UPDATED: Symbol = symbol_short!("inc_upd");
const BULK_TRANSFER: Symbol = symbol_short!("bulk_xfr");
const WASTE_CONFIRMATION_RESET: Symbol = symbol_short!("reset");
const WASTE_DEACTIVATED: Symbol = symbol_short!("deactive");

/// Emit event when waste is registered
pub fn emit_waste_registered(
    env: &Env,
    waste_id: u128,
    recycler: &Address,
    waste_type: WasteType,
    weight: u128,
    latitude: i128,
    longitude: i128,
) {
    env.events().publish(
        (WASTE_REGISTERED, waste_id),
        (waste_type, weight, recycler, latitude, longitude),
    );
}

/// Emit event when a donation is made to charity
pub fn emit_donation_made(
    env: &Env,
    donor: &Address,
    amount: i128,
    charity_contract: &Address,
) {
    env.events().publish(
        (DONATION_MADE, donor),
        (amount, charity_contract),
    );
}

/// Emit event when waste is transferred (v1 API — u64 waste ID)
pub fn emit_waste_transferred(
    env: &Env,
    waste_id: u64,
    from: &Address,
    to: &Address,
) {
    env.events().publish(
        (WASTE_TRANSFERRED, waste_id),
        (from, to),
    );
}

/// Emit event when a v2 waste item is transferred (u128 waste ID, includes timestamp)
pub fn emit_waste_transferred_v2(
    env: &Env,
    waste_id: u128,
    from: &Address,
    to: &Address,
    timestamp: u64,
) {
    env.events().publish(
        (WASTE_TRANSFERRED, waste_id),
        (from, to, timestamp),
    );
}

/// Emit event when waste is confirmed by a third party
pub fn emit_waste_confirmed(
    env: &Env,
    waste_id: u128,
    confirmer: &Address,
) {
    env.events().publish(
        (WASTE_CONFIRMED, waste_id),
        confirmer,
    );
}

/// Emit event when a participant registers
pub fn emit_participant_registered(
    env: &Env,
    address: &Address,
    role: ParticipantRole,
    name: Symbol,
    latitude: i128,
    longitude: i128,
) {
    env.events().publish(
        (PARTICIPANT_REGISTERED, address),
        (role.to_u32(), name, latitude, longitude),
    );
}

/// Emit event when tokens are rewarded
pub fn emit_tokens_rewarded(
    env: &Env,
    recipient: &Address,
    amount: u128,
    waste_id: u64,
) {
    env.events().publish(
        (TOKENS_REWARDED, recipient),
        (amount, waste_id),
    );
}

/// Emit event when a participant updates their location
pub fn emit_participant_location_updated(
    env: &Env,
    address: &Address,
    latitude: i128,
    longitude: i128,
) {
    env.events().publish(
        (PARTICIPANT_LOCATION_UPDATED, address),
        (latitude, longitude),
    );
}

pub fn emit_admin_transferred(env: &Env, previous_admin: &Address) {
    env.events().publish((ADMIN_TRANSFERRED,), previous_admin);
}

pub fn emit_contract_paused(env: &Env, admin: &Address) {
    env.events().publish((CONTRACT_PAUSED,), admin);
}

pub fn emit_contract_unpaused(env: &Env, admin: &Address) {
    env.events().publish((CONTRACT_UNPAUSED,), admin);
}

/// Emit event when an incentive's reward points / budget are updated
pub fn emit_incentive_updated(
    env: &Env,
    incentive_id: u64,
    rewarder: &Address,
    new_reward_points: u64,
    new_total_budget: u64,
) {
    env.events().publish(
        (INCENTIVE_UPDATED, incentive_id),
        (rewarder, new_reward_points, new_total_budget),
    );
}

/// Emit event when a collector hands aggregated waste directly to a manufacturer
pub fn emit_bulk_transfer(
    env: &Env,
    waste_id: u128,
    collector: &Address,
    manufacturer: &Address,
    waste_type: WasteType,
    timestamp: u64,
) {
    env.events().publish(
        (BULK_TRANSFER, waste_id),
        (collector, manufacturer, waste_type, timestamp),
    );
}

/// Emit event when a waste item's confirmation status is reset by its owner
pub fn emit_waste_confirmation_reset(env: &Env, waste_id: u128, owner: &Address, timestamp: u64) {
    env.events().publish(
        (WASTE_CONFIRMATION_RESET, waste_id),
        (owner, timestamp),
    );
}

/// Emit event when a waste item is permanently deactivated
pub fn emit_waste_deactivated(env: &Env, waste_id: u128, admin: &Address, timestamp: u64) {
    env.events().publish(
        (WASTE_DEACTIVATED, waste_id),
        (admin, timestamp),
    );
}
