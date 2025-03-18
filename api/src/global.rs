use cache::RedisConnectionManager;
use r2d2::Pool;
use sea_orm::DatabaseConnection;

/// Application resources
#[derive(Debug, Clone)]
#[deprecated(note = "稍后删除")]
pub struct AppResources {
    pub db: DatabaseConnection,
    pub redis_pool: Pool<RedisConnectionManager>,
    pub cfg: common::cfg::AppCfg,
}
