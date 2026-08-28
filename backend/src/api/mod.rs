/// Top-level API module.
///
/// Exposes the shared `errors` module and the `contracts` sub-API, wired
/// together for route registration in `main.rs`.
pub mod contracts;
pub mod errors;

use actix_web::web;

/// Mounts all API routes under `/api`.
pub fn configure_api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(contracts::configure_contract_routes),
    );
}
