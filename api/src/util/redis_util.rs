use cache::RedisConnectionManager;
use common::cfg::AppCfg;
use r2d2::Pool;
use rand::Rng;
use serde::Serialize;
use serde_json::json;

use crate::rsp::AppErrors;

pub struct ReidsUtil {
    cfg: AppCfg,
    pool: Pool<RedisConnectionManager>,
}
impl ReidsUtil {
    pub fn new(cfg: AppCfg, pool: Pool<RedisConnectionManager>) -> Self {
        Self { cfg, pool }
    }

    fn cache_key_from_str(&self, parts: &String) -> String {
        format!("{}:{}", self.cfg.cache.prefix, parts)
    }
    fn ttl(&self) -> i64 {
        let ttl = self.cfg.cache.ttl;
        let mut rng = rand::rng();
        let ttl = ttl + rng.random_range(0..self.cfg.cache.ttl_delta);
        ttl
    }

    pub fn cache_json_str<T: Serialize>(&self, parts: &String, data: &T) -> Result<(), AppErrors> {
        let key = self.cache_key_from_str(parts);
        let mut pool = self.pool.get()?;
        redis::cmd("SET")
            .arg(&key)
            .arg(json!(data).to_string())
            .arg("EX")
            .arg(self.ttl())
            .exec(&mut pool)?;
        Ok(())
    }

    pub fn fetch_and_dejson<T: serde::de::DeserializeOwned>(
        &self,
        parts: &String,
    ) -> Result<Option<T>, AppErrors> {
        let key = self.cache_key_from_str(parts);
        let mut pool = self.pool.get()?;
        let cached: Option<String> = redis::cmd("GET").arg(&key).query(&mut pool)?;

        match cached {
            Some(cached_str) => {
                let model: T = serde_json::from_str(&cached_str)?;
                Ok(Some(model))
            }
            None => {
                log::debug!("Cache not found, run db query {:?}", key);
                Ok(None)
            }
        }
    }
}
