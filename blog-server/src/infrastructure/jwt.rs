use core::str;

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i64,
    pub user_name: String,
    pub exp: usize,
}

#[derive(Clone)]
pub struct JwtService {
    secret: String,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        Self {
            secret: secret.into(),
        }
    }

    pub fn generate_token(
        &self,
        user_id: i64,
        user_name: &str,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = Claims {
            user_id,
            user_name: user_name.to_string(),
            exp: chrono::Utc::now()
                .checked_add_signed(chrono::Duration::hours(1))
                .ok_or_else(|| {
                    jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken)
                })?
                .timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )?;
        Ok(data.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};

    #[test]
    fn generate_and_verify_roundtrip() {
        let svc = JwtService::new("test-secret");
        let token = svc.generate_token(42, "alice").unwrap();
        let claims = svc.verify_token(&token).unwrap();
        assert_eq!(claims.user_id, 42);
        assert_eq!(claims.user_name, "alice");
    }

    #[test]
    fn wrong_secret_rejects_token() {
        let svc_a = JwtService::new("secret-a");
        let svc_b = JwtService::new("secret-b");
        let token = svc_a.generate_token(1, "bob").unwrap();
        assert!(svc_b.verify_token(&token).is_err());
    }

    #[test]
    fn garbage_token_returns_error() {
        let svc = JwtService::new("secret");
        assert!(svc.verify_token("not.a.jwt").is_err());
    }

    #[test]
    fn empty_token_returns_error() {
        let svc = JwtService::new("secret");
        assert!(svc.verify_token("").is_err());
    }

    #[test]
    fn expired_token_is_rejected() {
        let svc = JwtService::new("test-secret");
        let claims = Claims {
            user_id: 1,
            user_name: "expired_user".into(),
            exp: 0, // epoch — long expired
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(b"test-secret"),
        )
        .unwrap();
        assert!(svc.verify_token(&token).is_err());
    }

    #[test]
    fn token_preserves_unicode_username() {
        let svc = JwtService::new("secret");
        let token = svc.generate_token(7, "Пользователь").unwrap();
        let claims = svc.verify_token(&token).unwrap();
        assert_eq!(claims.user_name, "Пользователь");
    }

    #[test]
    fn different_users_get_different_tokens() {
        let svc = JwtService::new("secret");
        let t1 = svc.generate_token(1, "alice").unwrap();
        let t2 = svc.generate_token(2, "bob").unwrap();
        assert_ne!(t1, t2);
    }

    #[test]
    fn token_with_wrong_algorithm_is_rejected() {
        let claims = Claims {
            user_id: 1,
            user_name: "alice".into(),
            exp: (chrono::Utc::now().timestamp() + 3600) as usize,
        };
        let token = encode(
            &Header::new(Algorithm::HS384),
            &claims,
            &EncodingKey::from_secret(b"test-secret"),
        )
        .unwrap();
        let svc = JwtService::new("test-secret");
        assert!(svc.verify_token(&token).is_err());
    }
}
