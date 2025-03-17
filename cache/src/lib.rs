#[cfg(feature = "cluster")]
mod cluster;

mod keys;
#[cfg(feature = "standalone")]
mod standalone;

use common::cfg::RedisMode;
use r2d2::{ManageConnection, Pool};
use redis::RedisError;
use std::fmt;

pub use keys::{gen_key, gen_key_with_prefix};

/// 创建 Redis 连接池
pub fn create_redis_pool(config: &RedisMode) -> Result<Pool<RedisConnectionManager>, RedisError> {
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
    pub fn new(config: &RedisMode) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

impl fmt::Debug for RedisConnectionManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RedisConnectionManager")
    }
}

#[cfg(all(feature = "standalone", feature = "cluster"))]
compile_error!("standalone 和 cluster 是互斥的，不能同时启用！");

/// 避免IDE提示错误
#[cfg(all(not(feature = "standalone"), not(feature = "cluster")))]
impl ManageConnection for RedisConnectionManager {
    type Connection = redis::Connection;
    type Error = RedisError;
    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        unimplemented!()
    }

    fn is_valid(&self, _conn: &mut Self::Connection) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        unimplemented!()
    }
}
