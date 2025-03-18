use common::cfg;

pub fn gen_key(cfg: &cfg::Cache, part: Vec<String>) -> String {
    let name = part.join(":");
    format!("{}:{}", cfg.prefix, name)
}
pub fn gen_key_with_prefix(prefix: String, part: Vec<String>) -> String {
    let name = part.join(":");
    format!("{}:{}", prefix, name)
}
