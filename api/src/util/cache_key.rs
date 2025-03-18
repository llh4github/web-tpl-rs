use common::cfg::AppCfg;

pub struct CacheKeyUtil {
    cfg: AppCfg,
}
impl CacheKeyUtil {
    pub fn new(cfg: AppCfg) -> Self {
        Self { cfg }
    }

    pub fn cache_key(&self, infix: &str, key: String) -> String {
        format!("{}:{}:{}", self.cfg.cache.prefix, infix, key)
    }
    pub fn cache_key_i32(&self, infix: &str, key: i32) -> String {
        format!("{}:{}:{}", self.cfg.cache.prefix, infix, key)
    }
    pub fn cache_key_from_str(&self, parts: Vec<&str>) -> String {
        let key = parts.join(":");
        format!("{}:{}", self.cfg.cache.prefix, key)
    }
    pub fn cache_key_from_strings(&self, parts: Vec<String>) -> String {
        let key = parts.join(":");
        format!("{}:{}", self.cfg.cache.prefix, key)
    }
}
