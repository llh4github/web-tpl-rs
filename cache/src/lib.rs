mod cluster;
mod standalone;

use common::cfg::RedisMode;
use r2d2::{ManageConnection, Pool};
use redis::{ConnectionLike, RedisError};
use std::fmt;

/// 创建 Redis 连接池
pub fn create_redis_pool(config: RedisMode) -> Result<Pool<RedisConnectionManager>, RedisError> {
    let manager = RedisConnectionManager::new(config);
    let pool = Pool::builder().build(manager).unwrap();
    Ok(pool)
}

/// 自定义连接管理器
#[derive(Clone)]
pub struct RedisConnectionManager {
    config: RedisMode,
}
impl RedisConnectionManager {
    pub fn new(config: RedisMode) -> Self {
        Self { config }
    }
}

impl fmt::Debug for RedisConnectionManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RedisConnectionManager")
    }
}

#[cfg(all(feature = "standalone", feature = "cluster"))]
compile_error!("feature_a 和 feature_b 是互斥的，不能同时启用！");
