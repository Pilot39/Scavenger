//! # Event Emission Utilities — Issue #814
//!
//! Reusable event builder pattern and formatting utilities for the Scavngr
//! Soroban contract.  All helpers are `no_std` / WASM-safe.
//!
//! ## Design
//!
//! * [`EventBuilder`] — fluent builder that accumulates a topic tuple and a
//!   data value, then publishes in one call.
//! * [`EventCategory`] — logical grouping used to filter events by subsystem.
//! * [`EventFormatter`] — human-readable string label generation.
//!
//! ## Quick start
//!
//! ```ignore
//! use crate::event_builder::{EventBuilder, EventCategory};
//!
//! // 2-topic event (matches existing events.rs style)
//! EventBuilder::new(env).publish2(symbol_short!("recycled"), waste_id, (waste_type, weight, recycler));
//!
//! // 1-topic event
//! EventBuilder::new(env).publish1(symbol_short!("paused"), admin);
//! ```

use soroban_sdk::{symbol_short, Env, IntoVal, Symbol, Val};

// ─── Category ────────────────────────────────────────────────────────────────

/// Logical grouping for events.
///
/// Used by off-chain indexers and the [`EventFilter`] helper to quickly select
/// events of interest without decoding the full topic tuple.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventCategory {
    /// Waste registration, transfer, confirmation, deactivation
    Waste,
    /// Participant registration, role changes, location updates
    Participant,
    /// Incentive creation, updates, deactivation, reward distribution
    Incentive,
    /// Token and donation events
    Token,
    /// Administrative events (admin transfer, pause/unpause)
    Admin,
    /// Verification workflow events
    Verification,
    /// Contract upgrade events
    Upgrade,
    /// General / uncategorised
    Other,
}

impl EventCategory {
    /// Returns the short `Symbol` prefix used as the first topic element.
    ///
    /// Keep symbols ≤ 9 chars (Soroban `symbol_short!` limit).
    pub fn prefix(&self) -> Symbol {
        match self {
            EventCategory::Waste => symbol_short!("waste"),
            EventCategory::Participant => symbol_short!("part"),
            EventCategory::Incentive => symbol_short!("inc"),
            EventCategory::Token => symbol_short!("token"),
            EventCategory::Admin => symbol_short!("admin"),
            EventCategory::Verification => symbol_short!("verif"),
            EventCategory::Upgrade => symbol_short!("upg"),
            EventCategory::Other => symbol_short!("other"),
        }
    }

    /// Static string label — useful for off-chain logging / metrics.
    pub fn label(&self) -> &'static str {
        match self {
            EventCategory::Waste => "WASTE",
            EventCategory::Participant => "PARTICIPANT",
            EventCategory::Incentive => "INCENTIVE",
            EventCategory::Token => "TOKEN",
            EventCategory::Admin => "ADMIN",
            EventCategory::Verification => "VERIFICATION",
            EventCategory::Upgrade => "UPGRADE",
            EventCategory::Other => "OTHER",
        }
    }

    /// Infers the category from a first-topic `Symbol` by prefix matching.
    ///
    /// Primarily used by the off-chain indexer; not called on-chain.
    pub fn from_symbol(sym: &Symbol) -> Self {
        if *sym == symbol_short!("waste") {
            EventCategory::Waste
        } else if *sym == symbol_short!("part") {
            EventCategory::Participant
        } else if *sym == symbol_short!("inc") {
            EventCategory::Incentive
        } else if *sym == symbol_short!("token") {
            EventCategory::Token
        } else if *sym == symbol_short!("admin") {
            EventCategory::Admin
        } else if *sym == symbol_short!("verif") {
            EventCategory::Verification
        } else if *sym == symbol_short!("upg") {
            EventCategory::Upgrade
        } else {
            EventCategory::Other
        }
    }
}

// ─── Builder ─────────────────────────────────────────────────────────────────

/// Fluent builder for Soroban contract events.
///
/// Soroban events require a **topics tuple** and a **data value**.
/// `EventBuilder` wraps the common 1-topic and 2-topic patterns behind a
/// readable API, matching exactly what existing `events.rs` functions do.
///
/// # Example — 2-topic event
///
/// ```ignore
/// EventBuilder::new(env)
///     .publish2(WASTE_REGISTERED, waste_id, (waste_type, weight, recycler, lat, lon));
/// ```
///
/// # Example — 1-topic event
///
/// ```ignore
/// EventBuilder::new(env)
///     .publish1(symbol_short!("paused"), admin);
/// ```
pub struct EventBuilder<'a> {
    env: &'a Env,
}

impl<'a> EventBuilder<'a> {
    /// Creates a new builder.
    pub fn new(env: &'a Env) -> Self {
        Self { env }
    }

    /// Publishes a 1-topic event: `topics = (t1,)`.
    pub fn publish1<T1, D>(self, t1: T1, data: D)
    where
        T1: IntoVal<Env, Val>,
        D: IntoVal<Env, Val>,
    {
        self.env.events().publish((t1,), data);
    }

    /// Publishes a 2-topic event: `topics = (t1, t2)`.
    pub fn publish2<T1, T2, D>(self, t1: T1, t2: T2, data: D)
    where
        T1: IntoVal<Env, Val>,
        T2: IntoVal<Env, Val>,
        D: IntoVal<Env, Val>,
    {
        self.env.events().publish((t1, t2), data);
    }

    /// Publishes a 3-topic event: `topics = (t1, t2, t3)`.
    pub fn publish3<T1, T2, T3, D>(self, t1: T1, t2: T2, t3: T3, data: D)
    where
        T1: IntoVal<Env, Val>,
        T2: IntoVal<Env, Val>,
        T3: IntoVal<Env, Val>,
        D: IntoVal<Env, Val>,
    {
        self.env.events().publish((t1, t2, t3), data);
    }
}

// ─── Formatter ───────────────────────────────────────────────────────────────

/// Utility for generating human-readable event labels.
///
/// Used off-chain (indexer / frontend) to produce display strings; on-chain
/// compilation is kept in the WASM binary only to satisfy the module boundary.
pub struct EventFormatter;

impl EventFormatter {
    /// Returns a concise label for a known event symbol.
    ///
    /// Falls back to `"UNKNOWN"` for unrecognised symbols.
    pub fn label_for(sym: &Symbol) -> &'static str {
        if *sym == symbol_short!("recycled") {
            "WASTE_REGISTERED"
        } else if *sym == symbol_short!("transfer") {
            "WASTE_TRANSFERRED"
        } else if *sym == symbol_short!("confirmed") {
            "WASTE_CONFIRMED"
        } else if *sym == symbol_short!("reg") {
            "PARTICIPANT_REGISTERED"
        } else if *sym == symbol_short!("rewarded") {
            "TOKENS_REWARDED"
        } else if *sym == symbol_short!("donated") {
            "DONATION_MADE"
        } else if *sym == symbol_short!("deactive") {
            "WASTE_DEACTIVATED"
        } else if *sym == symbol_short!("expired") {
            "WASTE_EXPIRED"
        } else if *sym == symbol_short!("paused") {
            "CONTRACT_PAUSED"
        } else if *sym == symbol_short!("unpaused") {
            "CONTRACT_UNPAUSED"
        } else if *sym == symbol_short!("ver_start") {
            "VERIFICATION_STARTED"
        } else if *sym == symbol_short!("ver_comp") {
            "VERIFICATION_COMPLETED"
        } else if *sym == symbol_short!("ver_fail") {
            "VERIFICATION_FAILED"
        } else {
            "UNKNOWN"
        }
    }

    /// Returns `true` if the symbol belongs to the waste subsystem.
    pub fn is_waste_event(sym: &Symbol) -> bool {
        *sym == symbol_short!("recycled")
            || *sym == symbol_short!("transfer")
            || *sym == symbol_short!("confirmed")
            || *sym == symbol_short!("deactive")
            || *sym == symbol_short!("expired")
            || *sym == symbol_short!("split")
            || *sym == symbol_short!("merged")
            || *sym == symbol_short!("reserved")
    }

    /// Returns `true` if the symbol belongs to the participant subsystem.
    pub fn is_participant_event(sym: &Symbol) -> bool {
        *sym == symbol_short!("reg")
            || *sym == symbol_short!("loc_upd")
            || *sym == symbol_short!("tier_upd")
            || *sym == symbol_short!("cert_gr")
    }

    /// Returns `true` if the symbol is a security-relevant event that should
    /// be flagged for monitoring.
    pub fn is_security_event(sym: &Symbol) -> bool {
        *sym == symbol_short!("adm_xfr")
            || *sym == symbol_short!("paused")
            || *sym == symbol_short!("upg_exec")
            || *sym == symbol_short!("perm_gr")
            || *sym == symbol_short!("perm_rv")
    }
}

// ─── Filter ───────────────────────────────────────────────────────────────────

/// Predicate-based event filter helper.
///
/// Off-chain callers build a `FilterConfig` and pass event topic symbols
/// through `EventFilter::matches` to decide whether to index/process an event.
#[derive(Clone)]
pub struct EventFilter {
    /// If `Some`, only events matching this category pass.
    pub category: Option<EventCategory>,
    /// If `true`, only security events pass (overrides `category`).
    pub security_only: bool,
    /// If `true`, only waste-related events pass (unless `security_only`).
    pub waste_only: bool,
}

impl EventFilter {
    /// Creates a filter that accepts all events.
    pub fn all() -> Self {
        Self {
            category: None,
            security_only: false,
            waste_only: false,
        }
    }

    /// Creates a filter restricted to a single category.
    pub fn category(cat: EventCategory) -> Self {
        Self {
            category: Some(cat),
            security_only: false,
            waste_only: false,
        }
    }

    /// Creates a filter that accepts only security-relevant events.
    pub fn security() -> Self {
        Self {
            category: None,
            security_only: true,
            waste_only: false,
        }
    }

    /// Returns `true` if the event identified by `symbol` passes this filter.
    pub fn matches(&self, symbol: &Symbol) -> bool {
        if self.security_only {
            return EventFormatter::is_security_event(symbol);
        }
        if self.waste_only {
            return EventFormatter::is_waste_event(symbol);
        }
        if let Some(cat) = &self.category {
            return match cat {
                EventCategory::Waste => EventFormatter::is_waste_event(symbol),
                EventCategory::Participant => EventFormatter::is_participant_event(symbol),
                _ => false, // extend as needed
            };
        }
        true
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{symbol_short, Env};

    #[test]
    fn event_category_labels_are_stable() {
        assert_eq!(EventCategory::Waste.label(), "WASTE");
        assert_eq!(EventCategory::Participant.label(), "PARTICIPANT");
        assert_eq!(EventCategory::Admin.label(), "ADMIN");
    }

    #[test]
    fn event_formatter_labels_known_symbols() {
        assert_eq!(
            EventFormatter::label_for(&symbol_short!("recycled")),
            "WASTE_REGISTERED"
        );
        assert_eq!(
            EventFormatter::label_for(&symbol_short!("transfer")),
            "WASTE_TRANSFERRED"
        );
        assert_eq!(
            EventFormatter::label_for(&symbol_short!("confirmed")),
            "WASTE_CONFIRMED"
        );
        assert_eq!(
            EventFormatter::label_for(&symbol_short!("unknown_x")),
            "UNKNOWN"
        );
    }

    #[test]
    fn formatter_correctly_classifies_waste_events() {
        assert!(EventFormatter::is_waste_event(&symbol_short!("recycled")));
        assert!(EventFormatter::is_waste_event(&symbol_short!("transfer")));
        assert!(EventFormatter::is_waste_event(&symbol_short!("split")));
        assert!(!EventFormatter::is_waste_event(&symbol_short!("reg")));
    }

    #[test]
    fn formatter_correctly_classifies_participant_events() {
        assert!(EventFormatter::is_participant_event(&symbol_short!("reg")));
        assert!(EventFormatter::is_participant_event(&symbol_short!("loc_upd")));
        assert!(!EventFormatter::is_participant_event(&symbol_short!("recycled")));
    }

    #[test]
    fn formatter_correctly_classifies_security_events() {
        assert!(EventFormatter::is_security_event(&symbol_short!("adm_xfr")));
        assert!(EventFormatter::is_security_event(&symbol_short!("paused")));
        assert!(!EventFormatter::is_security_event(&symbol_short!("recycled")));
    }

    #[test]
    fn filter_all_passes_every_symbol() {
        let f = EventFilter::all();
        assert!(f.matches(&symbol_short!("recycled")));
        assert!(f.matches(&symbol_short!("reg")));
        assert!(f.matches(&symbol_short!("adm_xfr")));
    }

    #[test]
    fn filter_waste_only_passes_waste_symbols() {
        let f = EventFilter {
            category: None,
            security_only: false,
            waste_only: true,
        };
        assert!(f.matches(&symbol_short!("recycled")));
        assert!(!f.matches(&symbol_short!("reg")));
    }

    #[test]
    fn filter_security_only_passes_security_symbols() {
        let f = EventFilter::security();
        assert!(f.matches(&symbol_short!("adm_xfr")));
        assert!(f.matches(&symbol_short!("paused")));
        assert!(!f.matches(&symbol_short!("recycled")));
    }

    #[test]
    fn filter_category_waste_passes_waste_events() {
        let f = EventFilter::category(EventCategory::Waste);
        assert!(f.matches(&symbol_short!("recycled")));
        assert!(f.matches(&symbol_short!("transfer")));
        assert!(!f.matches(&symbol_short!("reg")));
    }

    #[test]
    fn event_builder_publish1_works() {
        let env = Env::default();
        EventBuilder::new(&env).publish1(symbol_short!("paused"), 1_u32);
    }

    #[test]
    fn event_builder_publish2_works() {
        let env = Env::default();
        EventBuilder::new(&env).publish2(symbol_short!("recycled"), 42_u64, 100_u64);
    }

    #[test]
    fn event_builder_publish3_works() {
        let env = Env::default();
        EventBuilder::new(&env)
            .publish3(symbol_short!("recycled"), 42_u64, symbol_short!("extra"), (1_u32,));
    }

    #[test]
    fn event_builder_topic2_convenience_works() {
        let env = Env::default();
        EventBuilder::new(&env).publish2(symbol_short!("recycled"), 99_u128, ("plastic", 500_u128));
    }
}
