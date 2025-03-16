use chrono::Utc;
use common::cfg;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

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

    let header = Header {
        typ: Some("jwt".to_string()),
        ..Default::default()
    };

    let secret = cfg.secret.as_bytes();
    encode(&header, &my_claims, &EncodingKey::from_secret(secret))
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
