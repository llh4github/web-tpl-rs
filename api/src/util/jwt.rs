use cache::RedisConnectionManager;
use chrono::Utc;
use common::cfg;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use r2d2::PooledConnection;
use serde::{Deserialize, Serialize};

use crate::rsp::AppErrors;

const KEY_INFIX: &str = "jwt";

pub fn parse_token(cfg: &cfg::Jwt, token: String) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = cfg.secret.as_bytes();
    // `token` is a struct with 2 fields: `header` and `claims` where `claims` is your own struct.
    let token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )?;
    Ok(token.claims)
}

pub fn create_token(
    cfg: &cfg::Jwt,
    username: String,
) -> Result<String, jsonwebtoken::errors::Error> {
    let my_claims = Claims {
        sub: username,
        iss: cfg.issuer.clone(),
        iat: Utc::now().timestamp(),
        exp: Utc::now().timestamp() + cfg.expiration,
    };
    create_token_with_claims(cfg, &my_claims)
}

fn create_token_with_claims(
    cfg: &cfg::Jwt,
    my_claims: &Claims,
) -> Result<String, jsonwebtoken::errors::Error> {
    let header = Header {
        typ: Some("jwt".to_string()),
        ..Default::default()
    };

    let secret = cfg.secret.as_bytes();
    encode(&header, my_claims, &EncodingKey::from_secret(secret))
}
pub fn validat_token(
    redis_pool: &mut PooledConnection<RedisConnectionManager>,
    cache: &cfg::Cache,
    jwt: &cfg::Jwt,
    token: String,
) -> Result<Claims, AppErrors> {
    let claims = parse_token(jwt, token.clone()).map_err(|e| AppErrors::JwtValidateErr {
        token: token.clone(),
        source: e,
    })?;
    let key = cache::gen_key(cache, vec![KEY_INFIX.to_string(), claims.sub.clone()]);
    let token_db = redis::cmd("get")
        .arg(key)
        .query::<Option<String>>(redis_pool)?;
    match token_db {
        None => Err(AppErrors::CommonErr(format!(
            "token is not in redis: {}",
            token
        ))),
        Some(t) => {
            if t != token {
                Err(AppErrors::CommonErr(format!(
                    "token与redis中的数据不相等: {}",
                    token
                )))
            } else {
                Ok(claims)
            }
        }
    }
}
pub fn create_and_cache_token(
    redis_pool: &mut PooledConnection<RedisConnectionManager>,
    username: String,
    jwt: &cfg::Jwt,
    cache: &cfg::Cache,
) -> Result<String, AppErrors> {
    let my_claims = Claims {
        sub: username.clone(),
        iss: jwt.issuer.clone(),
        iat: Utc::now().timestamp(),
        exp: Utc::now().timestamp() + jwt.expiration,
    };
    let token = create_token_with_claims(jwt, &my_claims)?;
    let key = cache::gen_key(cache, vec![KEY_INFIX.to_string(), username.clone()]);
    redis::cmd("set")
        .arg(key)
        .arg(token.clone())
        .arg("EX")
        .arg(jwt.expiration)
        .exec(redis_pool)?;
    Ok(token.clone())
}
/// JWT Claims结构
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Claims {
    /// 标识 JWT 的过期时间（UNIX 时间戳）
    exp: i64,
    /// 标识 JWT 的签发时间（UNIX 时间戳）
    iat: i64,
    /// 标识 JWT 的签发者
    iss: String,
    /// 标识 JWT 的主题（用户 ID 或邮箱）
    sub: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token() -> Result<(), jsonwebtoken::errors::Error> {
        let cfg = cfg::Jwt {
            issuer: "issuer".to_string(),
            secret: "test".to_string(),
            expiration: 114514,
            header_name: "".to_string(),
            header_prefix: "".to_string(),
        };
        let token = create_token(&cfg, "username".to_string())?;
        println!("{}", token);
        let claims = parse_token(&cfg, token)?;
        println!("{:?}", claims);
        assert_eq!(claims.sub, "username");
        assert_eq!(claims.iss, cfg.issuer);
        Ok(())
    }
}
