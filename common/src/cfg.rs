use crate::consts::DEV;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ApiNetwork {
    pub port: u16,
    pub prefix: String,
}
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub debug: bool,
    pub network: ApiNetwork,
    pub database: Database,
    // pub redis: Redis,
}
#[derive(Debug, Deserialize)]
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
