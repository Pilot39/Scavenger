// #1158: `csrf.rs` was removed — it was imported but never `.wrap()`-ed into
// the active router (see main.rs's App builder), so it was dead weight left
// over from an earlier auth scheme. CSRF protection is out of scope for the
// currently registered middleware stack.
pub mod rate_limit;
pub mod request_id;
pub mod validation;
// #919: Idempotency key support for write operations
pub mod idempotency;

pub use idempotency::IdempotencyMiddleware;
// #1158: `RateLimitLayer` was a stale re-export left over from a superseded
// rate-limiting implementation; no such type is defined in `rate_limit`, and
// `RateLimitMiddleware` is the one actually registered in the router.
pub use rate_limit::{RateLimitConfig, RateLimitMiddleware, RateLimitTier, RouteRateLimitConfig};
pub use request_id::{RequestId, RequestIdMiddleware};
pub use validation::ValidationMiddleware;
