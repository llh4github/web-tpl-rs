use crate::consts::{DEV, PROD};
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

/// 应用配置
#[derive(Debug, Deserialize, Clone)]
pub struct AppCfg {
    pub network: ApiNetwork,
    pub database: Database,
    pub redis: RedisMode,
    pub jwt: Jwt,
    pub cache: Cache,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Cache {
    /// 缓存前缀
    pub prefix: String,
    /// 缓存过期时间 秒
    pub ttl: i64,
    /// 缓存时间波动值 秒
    pub ttl_delta: i64,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Jwt {
    pub header_name: String,
    pub header_prefix: String,
    pub issuer: String,
    pub secret: String,
    pub expiration: i64,
    pub anno_url: Vec<String>,
}

/// URL format: `{redis|rediss}://[<username>][:<password>@]<hostname>[:port][/<db>]`
///
/// - Basic: `redis://127.0.0.1:6379`
/// - Username & Password: `redis://user:password@127.0.0.1:6379`
/// - Password only: `redis://:password@127.0.0.1:6379`
/// - Specifying DB: `redis://127.0.0.1:6379/0`
/// - Enabling TLS: `rediss://127.0.0.1:6379`
/// - Enabling Insecure TLS: `rediss://127.0.0.1:6379/#insecure`
/// - Enabling RESP3: `redis://127.0.0.1:6379/?protocol=resp3`
#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")] // 使用 "type" 字段确定枚举变体
pub enum RedisMode {
    Standalone { node: String },
    Cluster { nodes: Vec<String> },
}
#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct ApiNetwork {
    pub port: u16,
    pub prefix: String,
}
#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Database {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: i32,
    pub database: String,
    pub show_sql: bool,
}
impl Database {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

impl AppCfg {
    pub fn new() -> Result<Self, ConfigError> {
        // 从环境变量中获取运行模式
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| DEV.into());

        let config = Config::builder()
            .add_source(File::with_name("./application").required(false))
            .add_source(File::with_name("config/application").required(false))
            .add_source(Environment::with_prefix("app"));

        // 其他环境根据运行模式加载配置文件
        let config: config::ConfigBuilder<config::builder::DefaultState> =
            if run_mode.to_lowercase() != PROD {
                // log not init, so use println
                println!("run mode is : {}", run_mode);
                config
                    .add_source(
                        File::with_name(format!("config/application-{}", run_mode).as_str())
                            .required(false),
                    )
                    .add_source(
                        File::with_name(format!("./application-{}", run_mode).as_str())
                            .required(false),
                    )
            } else {
                config
            };
        config.build()?.try_deserialize()
    }
}
