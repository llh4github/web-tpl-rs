use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: u64,
}
#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

    #[test]
    fn test_a() {
        let my_claims = Claims {
            sub: "b@b.com".to_owned(),
            company: "ACME".to_owned(),
            exp: 10000000000,
        };
        let key = b"secret";

        let header = Header {
            kid: Some("signing_key".to_owned()),
            alg: Algorithm::HS512,
            ..Default::default()
        };
        let token = match encode(&header, &my_claims, &EncodingKey::from_secret(key)) {
            Ok(t) => println!("{}", t),
            Err(_) => panic!(), // in practice you would return the error
        };
    }
}
