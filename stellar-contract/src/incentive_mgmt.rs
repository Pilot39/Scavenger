//! Incentive management domain module (issue #925).
//!
//! Re-exports incentive-related types for domain-scoped imports.
//!
//! State-changing operations on `ScavengerContract` in `lib.rs`:
//! - `create_incentive` (manufacturer only)
//! - `update_incentive`, `update_incentive_status`, `deactivate_incentive`
//! - `schedule_incentive`
//! - `claim_incentive_reward`
//! - `distribute_rewards`
//! - `get_incentive_by_id`, `get_incentives`, `get_active_incentives`
//! - `get_incentives_by_waste_type`, `get_incentives_by_rewarder`
//! - `get_active_mfr_incentive`
//! - `calculate_incentive_reward`

pub use crate::types::Incentive;
