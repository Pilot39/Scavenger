pub mod contracts;
pub mod ws;
pub mod export;
pub mod audit;
pub mod verification;
pub mod compliance_api;
pub mod signing_api;
pub mod search;
pub mod archival;
// analytics module removed — legacy endpoints were never registered (#906)
#[cfg(test)]
mod pagination_boundary_tests;
