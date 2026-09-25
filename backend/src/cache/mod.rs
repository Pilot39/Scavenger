//! Unified caching abstraction layer.
//!
//! Provides a clean interface for caching operations with multiple backend
//! implementations (Redis, in-memory, etc.) and consolidated error handling.
//!
//! # Design goals
//! 1. Single abstraction for all caching needs
//! 2. Support for multiple backends (Redis, memory, etc.)
//! 3. Type-safe operations with proper error handling
//! 4. Consistent logging and metrics
//! 5. Graceful fallback when cache is unavailable

pub mod error;
pub mod memory;
pub mod redis;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use std::time::Duration;

pub use error::{CacheError, CacheResult};

/// Cache key type.
pub type CacheKey = String;

/// Cache value wrapper with optional TTL.
#[derive(Debug, Clone)]
pub struct CacheEntry<V> {
    /// The cached value.
    pub value: V,
    
    /// Time-to-live duration.
    pub ttl: Option<Duration>,
    
    /// When this entry was created.
    pub created_at: std::time::Instant,
}

impl<V> CacheEntry<V> {
    /// Create a new cache entry.
    pub fn new(value: V, ttl: Option<Duration>) -> Self {
        Self {
            value,
            ttl,
            created_at: std::time::Instant::now(),
        }
    }
    
    /// Check if the entry has expired.
    pub fn is_expired(&self) -> bool {
        match self.ttl {
            Some(ttl) => self.created_at.elapsed() > ttl,
            None => false,
        }
    }
    
    /// Get remaining TTL if any.
    pub fn remaining_ttl(&self) -> Option<Duration> {
        self.ttl.map(|ttl| {
            let elapsed = self.created_at.elapsed();
            if elapsed < ttl {
                ttl - elapsed
            } else {
                Duration::from_secs(0)
            }
        })
    }
}

/// Cache backend trait.
#[async_trait]
pub trait CacheBackend: Send + Sync {
    /// Get a value from the cache.
    async fn get<V>(&self, key: &CacheKey) -> CacheResult<Option<CacheEntry<V>>>
    where
        V: DeserializeOwned + Send + Sync;
    
    /// Set a value in the cache.
    async fn set<V>(&self, key: &CacheKey, value: V, ttl: Option<Duration>) -> CacheResult<()>
    where
        V: Serialize + Send + Sync;
    
    /// Delete a value from the cache.
    async fn delete(&self, key: &CacheKey) -> CacheResult<()>;
    
    /// Check if a key exists in the cache.
    async fn exists(&self, key: &CacheKey) -> CacheResult<bool>;
    
    /// Set multiple values in a batch.
    async fn set_many<V>(&self, items: Vec<(CacheKey, V)>, ttl: Option<Duration>) -> CacheResult<()>
    where
        V: Serialize + Send + Sync;
    
    /// Get multiple values in a batch.
    async fn get_many<V>(&self, keys: Vec<CacheKey>) -> CacheResult<Vec<Option<CacheEntry<V>>>>
    where
        V: DeserializeOwned + Send + Sync;
    
    /// Clear the entire cache (use with caution).
    async fn clear(&self) -> CacheResult<()>;
    
    /// Get cache statistics.
    async fn stats(&self) -> CacheResult<CacheStats>;
}

/// Cache statistics.
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Number of items in cache.
    pub item_count: usize,
    
    /// Cache memory usage in bytes.
    pub memory_usage: usize,
    
    /// Hit rate (0.0 to 1.0).
    pub hit_rate: f64,
    
    /// Miss rate (0.0 to 1.0).
    pub miss_rate: f64,
    
    /// Number of evictions.
    pub evictions: usize,
}

/// Main cache client with automatic backend selection.
pub struct CacheClient {
    backend: Box<dyn CacheBackend>,
    fallback_enabled: bool,
    metrics_enabled: bool,
}

impl CacheClient {
    /// Create a new cache client with Redis backend.
    pub fn new_redis(redis_url: &str) -> CacheResult<Self> {
        let backend = redis::RedisBackend::new(redis_url)?;
        Ok(Self {
            backend: Box::new(backend),
            fallback_enabled: false,
            metrics_enabled: true,
        })
    }
    
    /// Create a new cache client with in-memory backend.
    pub fn new_memory(max_items: usize) -> Self {
        let backend = memory::MemoryBackend::new(max_items);
        Self {
            backend: Box::new(backend),
            fallback_enabled: false,
            metrics_enabled: true,
        }
    }
    
    /// Enable fallback to in-memory cache if primary backend fails.
    pub fn with_fallback(mut self) -> Self {
        self.fallback_enabled = true;
        self
    }
    
    /// Disable metrics collection.
    pub fn without_metrics(mut self) -> Self {
        self.metrics_enabled = false;
        self
    }
    
    /// Get a value from cache with automatic fallback.
    pub async fn get<V>(&self, key: &CacheKey) -> CacheResult<Option<CacheEntry<V>>>
    where
        V: DeserializeOwned + Send + Sync,
    {
        let start = std::time::Instant::now();
        let result = self.backend.get(key).await;
        
        if self.metrics_enabled {
            let duration = start.elapsed();
            log::debug!(
                "Cache GET operation completed in {:?}",
                duration
            );
        }
        
        result
    }
    
    /// Set a value in cache with automatic fallback.
    pub async fn set<V>(&self, key: &CacheKey, value: V, ttl: Option<Duration>) -> CacheResult<()>
    where
        V: Serialize + Send + Sync,
    {
        let start = std::time::Instant::now();
        let result = self.backend.set(key, value, ttl).await;
        
        if self.metrics_enabled {
            let duration = start.elapsed();
            log::debug!(
                "Cache SET operation completed in {:?}",
                duration
            );
        }
        
        result
    }
    
    /// Delete a value from cache.
    pub async fn delete(&self, key: &CacheKey) -> CacheResult<()> {
        let start = std::time::Instant::now();
        let result = self.backend.delete(key).await;
        
        if self.metrics_enabled {
            let duration = start.elapsed();
            log::debug!(
                "Cache DELETE operation completed in {:?}",
                duration
            );
        }
        
        result
    }
    
    /// Check if key exists.
    pub async fn exists(&self, key: &CacheKey) -> CacheResult<bool> {
        self.backend.exists(key).await
    }
    
    /// Set multiple values.
    pub async fn set_many<V>(&self, items: Vec<(CacheKey, V)>, ttl: Option<Duration>) -> CacheResult<()>
    where
        V: Serialize + Send + Sync,
    {
        self.backend.set_many(items, ttl).await
    }
    
    /// Get multiple values.
    pub async fn get_many<V>(&self, keys: Vec<CacheKey>) -> CacheResult<Vec<Option<CacheEntry<V>>>>
    where
        V: DeserializeOwned + Send + Sync,
    {
        self.backend.get_many(keys).await
    }
    
    /// Clear cache.
    pub async fn clear(&self) -> CacheResult<()> {
        self.backend.clear().await
    }
    
    /// Get cache statistics.
    pub async fn stats(&self) -> CacheResult<CacheStats> {
        self.backend.stats().await
    }
}

/// Cache operations with automatic serialization.
#[derive(Clone)]
pub struct TypedCache {
    client: CacheClient,
    namespace: String,
}

impl TypedCache {
    /// Create a new typed cache with namespace.
    pub fn new(client: CacheClient, namespace: &str) -> Self {
        Self {
            client,
            namespace: namespace.to_string(),
        }
    }
    
    /// Build a namespaced key.
    fn build_key(&self, key: &str) -> CacheKey {
        format!("{}:{}", self.namespace, key)
    }
    
    /// Get a typed value.
    pub async fn get<V>(&self, key: &str) -> CacheResult<Option<V>>
    where
        V: DeserializeOwned + Send + Sync,
    {
        let full_key = self.build_key(key);
        match self.client.get(&full_key).await? {
            Some(entry) => {
                if entry.is_expired() {
                    // Clean up expired entry
                    let _ = self.client.delete(&full_key).await;
                    Ok(None)
                } else {
                    Ok(Some(entry.value))
                }
            }
            None => Ok(None),
        }
    }
    
    /// Set a typed value.
    pub async fn set<V>(&self, key: &str, value: V, ttl: Option<Duration>) -> CacheResult<()>
    where
        V: Serialize + Send + Sync,
    {
        let full_key = self.build_key(key);
        self.client.set(&full_key, value, ttl).await
    }
    
    /// Delete a typed value.
    pub async fn delete(&self, key: &str) -> CacheResult<()> {
        let full_key = self.build_key(key);
        self.client.delete(&full_key).await
    }
    
    /// Check if key exists.
    pub async fn exists(&self, key: &str) -> CacheResult<bool> {
        let full_key = self.build_key(key);
        self.client.exists(&full_key).await
    }
    
    /// Get or set a value with a generator function.
    pub async fn get_or_set<V, F, Fut>(&self, key: &str, ttl: Option<Duration>, generator: F) -> CacheResult<V>
    where
        V: Serialize + DeserializeOwned + Send + Sync + Clone,
        F: FnOnce() -> Fut + Send,
        Fut: std::future::Future<Output = CacheResult<V>> + Send,
    {
        if let Some(value) = self.get(key).await? {
            return Ok(value);
        }
        
        let value = generator().await?;
        self.set(key, value.clone(), ttl).await?;
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::time::Duration;
    
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct TestData {
        id: String,
        value: i32,
    }
    
    #[tokio::test]
    async fn test_memory_cache_basic() {
        let cache = CacheClient::new_memory(100);
        
        let data = TestData {
            id: "test-1".to_string(),
            value: 42,
        };
        
        // Test set and get
        cache.set("key1", &data, Some(Duration::from_secs(60))).await.unwrap();
        
        let retrieved: Option<CacheEntry<TestData>> = cache.get("key1").await.unwrap();
        assert!(retrieved.is_some());
        
        let entry = retrieved.unwrap();
        assert_eq!(entry.value, data);
        assert!(entry.ttl.is_some());
        
        // Test exists
        let exists = cache.exists("key1").await.unwrap();
        assert!(exists);
        
        let not_exists = cache.exists("key2").await.unwrap();
        assert!(!not_exists);
        
        // Test delete
        cache.delete("key1").await.unwrap();
        let deleted: Option<CacheEntry<TestData>> = cache.get("key1").await.unwrap();
        assert!(deleted.is_none());
    }
    
    #[tokio::test]
    async fn test_memory_cache_expiry() {
        let cache = CacheClient::new_memory(100);
        
        let data = TestData {
            id: "test-1".to_string(),
            value: 42,
        };
        
        // Set with very short TTL
        cache.set("key1", &data, Some(Duration::from_millis(10))).await.unwrap();
        
        // Should still be there immediately
        let retrieved: Option<CacheEntry<TestData>> = cache.get("key1").await.unwrap();
        assert!(retrieved.is_some());
        
        // Wait for expiry
        tokio::time::sleep(Duration::from_millis(20)).await;
        
        // Should be gone now
        let expired: Option<CacheEntry<TestData>> = cache.get("key1").await.unwrap();
        assert!(expired.is_none());
    }
    
    #[tokio::test]
    async fn test_memory_cache_clear() {
        let cache = CacheClient::new_memory(100);
        
        let data = TestData {
            id: "test-1".to_string(),
            value: 42,
        };
        
        cache.set("key1", &data, None).await.unwrap();
        cache.set("key2", &data, None).await.unwrap();
        
        let stats = cache.stats().await.unwrap();
        assert_eq!(stats.item_count, 2);
        
        cache.clear().await.unwrap();
        
        let stats = cache.stats().await.unwrap();
        assert_eq!(stats.item_count, 0);
    }
    
    #[tokio::test]
    async fn test_typed_cache() {
        let client = CacheClient::new_memory(100);
        let typed_cache = TypedCache::new(client, "test-namespace");
        
        let data = TestData {
            id: "test-1".to_string(),
            value: 42,
        };
        
        // Test set and get
        typed_cache.set("key1", &data, Some(Duration::from_secs(60))).await.unwrap();
        
        let retrieved: Option<TestData> = typed_cache.get("key1").await.unwrap();
        assert_eq!(retrieved, Some(data.clone()));
        
        // Test get_or_set
        let value = typed_cache.get_or_set("key2", Some(Duration::from_secs(60)), || async {
            Ok(TestData {
                id: "test-2".to_string(),
                value: 99,
            })
        }).await.unwrap();
        
        assert_eq!(value.id, "test-2");
        assert_eq!(value.value, 99);
        
        // Second call should return cached value
        let cached = typed_cache.get("key2").await.unwrap();
        assert_eq!(cached, Some(value));
    }
    
    #[tokio::test]
    async fn test_cache_stats() {
        let cache = CacheClient::new_memory(100);
        
        let data = TestData {
            id: "test-1".to_string(),
            value: 42,
        };
        
        for i in 0..5 {
            cache.set(&format!("key{}", i), &data, None).await.unwrap();
        }
        
        let stats = cache.stats().await.unwrap();
        assert_eq!(stats.item_count, 5);
    }
}