use redis::RedisError;

/// 应用错误
///
/// 把其他库的错误转化为统一的错误
#[derive(Debug, thiserror::Error)]
pub enum AppErrors {
    #[error("Err: {0}")]
    CommonErr(String),
    #[error("JwtValidateErr raw token {token:?}")]
    JwtValidateErr {
        token: String,
        #[source]
        source: jsonwebtoken::errors::Error,
    },
    /// Jwt 创建出错
    #[error(transparent)]
    JwtCreateErr(#[from] jsonwebtoken::errors::Error),

    #[error(transparent)]
    PoolErr(#[from] r2d2::Error),

    /// redis 操作出错
    #[error(transparent)]
    RedisErr(#[from] RedisError),

    #[error(transparent)]
    SerdeJsonErr(#[from] serde_json::Error),
}
