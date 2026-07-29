pub mod cache;
pub mod distributed;
pub mod invalidation;
pub mod warming;
pub mod ttl;

// Explicit re-exports to avoid name conflicts between cache::CacheMetrics
// and distributed::CacheMetrics (both define the same struct independently).
pub use cache::{Cache, CacheMetrics};
pub use distributed::{DistributedCache, CacheWarmer as DistributedCacheWarmer};
pub use invalidation::{
    CacheInvalidationManager, InvalidationEvent, InvalidationStrategy, InvalidationStrategyType,
    CacheWarmingStrategy,
};
pub use warming::{CacheWarmer, WarmTask};
pub use ttl::{CacheTtl, keys as cache_keys};
