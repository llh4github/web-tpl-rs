use crate::consts::DEV;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Settings {
    pub debug: bool,
    pub network: ApiNetwork,
    pub database: Database,
    pub redis: RedisMode,
    pub jwt: Jwt,
}
#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Jwt {
    pub issuer: String,
    pub secret: String,
    pub expiration: i64,
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
}
impl Database {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        // 从环境变量中获取运行模式
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| DEV.into());

        let config = Config::builder()
            .add_source(File::with_name("./application").required(false))
            .add_source(File::with_name("config/application").required(false))
            .add_source(Environment::with_prefix("app"));

        // dev 环境下允许向上两级查找配置文件
        let config = if run_mode.to_lowercase() == DEV {
            config
                .add_source(File::with_name("../config/application").required(false))
                .add_source(File::with_name("../../config/application").required(false))
        } else {
            config
        };
        config.build()?.try_deserialize()
    }
}
