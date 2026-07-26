use soroban_sdk::{symbol_short, Address, Env, Symbol};

use crate::types::{ParticipantRole, WasteGrade, WasteType, CertificationLevel, ParticipantTier};

const WASTE_REGISTERED: Symbol = symbol_short!("recycled");
const DONATION_MADE: Symbol = symbol_short!("donated");
const WASTE_TRANSFERRED: Symbol = symbol_short!("transfer");
const WASTE_CONFIRMED: Symbol = symbol_short!("confirmed");
const PARTICIPANT_REGISTERED: Symbol = symbol_short!("reg");
const TOKENS_REWARDED: Symbol = symbol_short!("rewarded");
const CERTIFICATION_GRANTED: Symbol = symbol_short!("cert_gr");
const AUCTION_CREATED: Symbol = symbol_short!("auc_cre");
const BID_PLACED: Symbol = symbol_short!("bid_plc");
const AUCTION_ENDED: Symbol = symbol_short!("auc_end");
const BULK_IMPORT_COMPLETED: Symbol = symbol_short!("bulk_imp");

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
pub fn emit_donation_made(env: &Env, donor: &Address, amount: i128, charity_contract: &Address) {
    env.events()
        .publish((DONATION_MADE, donor), (amount, charity_contract));
}

/// Emit event when waste is transferred
pub fn emit_waste_transferred(env: &Env, waste_id: u64, from: &Address, to: &Address) {
    env.events()
        .publish((WASTE_TRANSFERRED, waste_id), (from, to));
}

/// Emit event when waste is confirmed by a third party
pub fn emit_waste_confirmed(env: &Env, waste_id: u128, confirmer: &Address) {
    env.events().publish((WASTE_CONFIRMED, waste_id), confirmer);
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
pub fn emit_tokens_rewarded(env: &Env, recipient: &Address, amount: u128, waste_id: u64) {
    env.events()
        .publish((TOKENS_REWARDED, recipient), (amount, waste_id));
}

/// Emit event when a participant updates their location
pub fn emit_participant_location_updated(
    env: &Env,
    address: &Address,
    latitude: i128,
    longitude: i128,
) {
    env.events()
        .publish((symbol_short!("loc_upd"), address), (latitude, longitude));
}

pub fn emit_admin_transferred(env: &Env, previous_admin: &Address) {
    env.events()
        .publish((symbol_short!("adm_xfr"),), previous_admin);
}

pub fn emit_waste_expired(env: &Env, waste_id: u128) {
    env.events().publish(
        (symbol_short!("expired"), waste_id),
        env.ledger().timestamp(),
    );
}

pub fn emit_waste_deactivated(env: &Env, waste_id: u128, admin: &Address) {
    env.events().publish(
        (symbol_short!("deactive"), waste_id),
        (admin, env.ledger().timestamp()),
    );
}

pub fn emit_contract_paused(env: &Env, admin: &Address) {
    env.events().publish((symbol_short!("paused"),), admin);
}

pub fn emit_contract_unpaused(env: &Env, admin: &Address) {
    env.events().publish((symbol_short!("unpaused"),), admin);
}

/// Emit event when a waste item is graded
pub fn emit_waste_graded(env: &Env, waste_id: u128, grade: WasteGrade, grader: &Address) {
    env.events()
        .publish((symbol_short!("graded"), waste_id), (grade as u32, grader));
}

pub fn emit_proposal_created(env: &Env, proposal_id: u64, proposer: &Address) {
    env.events()
        .publish((symbol_short!("prop_new"), proposal_id), proposer);
}

pub fn emit_proposal_approved(env: &Env, proposal_id: u64, approver: &Address) {
    env.events()
        .publish((symbol_short!("prop_apr"), proposal_id), approver);
}

pub fn emit_proposal_executed(env: &Env, proposal_id: u64, executor: &Address) {
    env.events()
        .publish((symbol_short!("prop_exe"), proposal_id), executor);
}

pub fn emit_seasonal_multiplier_set(env: &Env, multiplier: u32, start: u64, end: u64) {
    env.events()
        .publish((symbol_short!("seas_set"),), (multiplier, start, end));
}

pub fn emit_carbon_credits_earned(
    env: &Env,
    participant: &Address,
    waste_type: crate::types::WasteType,
    weight: u128,
    credits: u128,
) {
    env.events()
        .publish((symbol_short!("carbon"), participant), (waste_type, weight, credits));
}

pub fn emit_processing_status_changed(env: &Env, waste_id: u128, status: u32, caller: &Address, timestamp: u64) {
    env.events()
        .publish((symbol_short!("proc_upd"), waste_id), (caller, status, timestamp));
}

pub fn emit_waste_contaminated(env: &Env, waste_id: u128, verifier: &Address, level: u32) {
    env.events()
        .publish((symbol_short!("contam"), waste_id), (verifier, level));
}

/// Emit event when a participant is granted a certification
pub fn emit_certification_granted(env: &Env, participant: &Address, level: CertificationLevel) {
    env.events()
        .publish((CERTIFICATION_GRANTED, participant), level.to_u32());
}

/// Emit event when a participant's tier changes
pub fn emit_participant_tier_changed(
    env: &Env,
    participant: &Address,
    old_tier: ParticipantTier,
    new_tier: ParticipantTier,
) {
    env.events().publish(
        (symbol_short!("tier_upd"), participant),
        (old_tier as u32, new_tier as u32),
    );
}

/// Emit event when an auction is created
pub fn emit_auction_created(env: &Env, auction_id: u64, waste_id: u128, creator: &Address, start_price: u128, end_time: u64) {
    env.events()
        .publish((AUCTION_CREATED, auction_id), (waste_id, creator, start_price, end_time));
}

/// Emit event when a bid is placed
pub fn emit_bid_placed(env: &Env, auction_id: u64, bidder: &Address, amount: u128) {
    env.events()
        .publish((BID_PLACED, auction_id), (bidder, amount));
}

/// Emit event when an auction ends
pub fn emit_auction_ended(env: &Env, auction_id: u64, winner: Option<&Address>, final_price: u128) {
    env.events()
        .publish((AUCTION_ENDED, auction_id), (winner, final_price));
}

/// Emit event when bulk import is completed
pub fn emit_bulk_import_completed(env: &Env, item_type: &str, count: u32) {
    env.events()
        .publish((BULK_IMPORT_COMPLETED,), (item_type, count));
}

pub fn emit_waste_split(env: &Env, waste_id: u128, owner: &Address, child_ids: &soroban_sdk::Vec<u128>) {
    env.events()
        .publish((symbol_short!("split"), waste_id), (owner, child_ids.len()));
}

pub fn emit_wastes_merged(env: &Env, merged_id: u128, owner: &Address, source_ids: &soroban_sdk::Vec<u128>) {
    env.events()
        .publish((symbol_short!("merged"), merged_id), (owner, source_ids.len()));
}

pub fn emit_waste_reserved(env: &Env, waste_id: u128, reserver: &Address, until: u64) {
    env.events()
        .publish((symbol_short!("reserved"), waste_id), (reserver, until));
}

pub fn emit_reservation_cancelled(env: &Env, waste_id: u128, caller: &Address) {
    env.events()
        .publish((symbol_short!("res_canc"), waste_id), caller);
}

pub fn emit_incentive_scheduled(
    env: &Env,
    incentive_id: u64,
    rewarder: &Address,
    starts_at: Option<u64>,
    ends_at: Option<u64>,
) {
    env.events()
        .publish((symbol_short!("inc_sched"), incentive_id), (rewarder, starts_at, ends_at));
}

pub fn emit_goal_achieved(env: &Env, participant: &Address, target_weight: u128) {
    env.events()
        .publish((symbol_short!("goal_ach"), participant), target_weight);
}

pub fn emit_carbon_credits_redeemed(
    env: &Env,
    participant: &Address,
    amount: u128,
    remaining: u128,
) {
    env.events()
        .publish((symbol_short!("carb_rdm"), participant), (amount, remaining));
}

pub fn emit_carbon_listing_created(
    env: &Env,
    listing_id: u64,
    seller: &Address,
    amount: u128,
    price_per_credit: i128,
) {
    env.events()
        .publish((symbol_short!("carb_lst"), listing_id), (seller, amount, price_per_credit));
}

pub fn emit_carbon_listing_cancelled(env: &Env, listing_id: u64, seller: &Address) {
    env.events()
        .publish((symbol_short!("carb_cnc"), listing_id), seller);
}

pub fn emit_carbon_listing_purchased(
    env: &Env,
    listing_id: u64,
    seller: &Address,
    buyer: &Address,
    amount: u128,
    total_price: i128,
) {
    env.events()
        .publish((symbol_short!("carb_buy"), listing_id), (seller, buyer, amount, total_price));
}

// ============ Verification Workflow Events (Issue #653) ============

pub fn emit_verification_started(
    env: &Env,
    waste_id: u128,
    verification_id: u64,
    verifier: &Address,
) {
    env.events()
        .publish((symbol_short!("ver_start"), verification_id), (waste_id, verifier));
}

pub fn emit_verification_completed(
    env: &Env,
    waste_id: u128,
    verification_id: u64,
    quality_score: u32,
) {
    env.events()
        .publish((symbol_short!("ver_comp"), verification_id), (waste_id, quality_score));
}

pub fn emit_verification_failed(env: &Env, waste_id: u128, verification_id: u64) {
    env.events()
        .publish((symbol_short!("ver_fail"), verification_id), waste_id);
}

pub fn emit_verification_expired(env: &Env, waste_id: u128, verification_id: u64) {
    env.events()
        .publish((symbol_short!("ver_exp"), verification_id), waste_id);
}

// ============ Upgrade System Events (Issue #652) ============

pub fn emit_upgrade_proposed(env: &Env, proposal_id: u64, new_implementation: &Address) {
    env.events()
        .publish((symbol_short!("upg_prop"), proposal_id), new_implementation);
}

pub fn emit_upgrade_approved(env: &Env, proposal_id: u64) {
    env.events()
        .publish((symbol_short!("upg_app"), proposal_id), ());
}

pub fn emit_upgrade_executed(env: &Env, proposal_id: u64, version: u32) {
    env.events()
        .publish((symbol_short!("upg_exec"), proposal_id), version);
}

pub fn emit_upgrade_rejected(env: &Env, proposal_id: u64) {
    env.events()
        .publish((symbol_short!("upg_rej"), proposal_id), ());
}

// ============ Blockchain Explorer Events (Issue #651) ============

pub fn emit_transaction_tracked(env: &Env, tx_id: u64, tx_hash: &String) {
    env.events()
        .publish((symbol_short!("tx_track"), tx_id), tx_hash);
}

pub fn emit_transaction_status_updated(
    env: &Env,
    tx_id: u64,
    status: crate::explorer::TransactionStatus,
) {
    env.events()
        .publish((symbol_short!("tx_stat"), tx_id), status.to_u32());
}

// ============ Advanced Analytics Events (Issue #650) ============

pub fn emit_analytics_report_created(
    env: &Env,
    report_id: u64,
    report_type: crate::analytics::ReportType,
) {
    env.events()
        .publish((symbol_short!("ana_rep"), report_id), report_type.to_u32());
}

pub fn emit_custom_query_created(env: &Env, query_id: u64) {
    env.events()
        .publish((symbol_short!("qry_cre"), query_id), ());
}

pub fn emit_custom_query_executed(env: &Env, query_id: u64) {
    env.events()
        .publish((symbol_short!("qry_exe"), query_id), ());
}

// ============ RBAC Events (#704) ============

pub fn emit_permission_granted(
    env: &Env,
    subject: &Address,
    permission: u32,
    granted_by: &Address,
) {
    env.events()
        .publish((symbol_short!("perm_gr"), subject), (permission, granted_by));
}

pub fn emit_permission_revoked(
    env: &Env,
    subject: &Address,
    permission: u32,
    revoked_by: &Address,
) {
    env.events()
        .publish((symbol_short!("perm_rv"), subject), (permission, revoked_by));
}

// ============ Reconciliation Events (#706) ============

pub fn emit_waste_reconciled(
    env: &Env,
    waste_id: u128,
    original_weight: u128,
    adjusted_weight: u128,
    reconciled_by: &Address,
) {
    env.events().publish(
        (symbol_short!("reconcil"), waste_id),
        (original_weight, adjusted_weight, reconciled_by),
    );
}