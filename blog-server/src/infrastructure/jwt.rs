use core::str;

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i64,
    pub username: String,
    pub exp: usize,
}

#[derive(Clone)]
pub struct JwtService {
    secret: String,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        // Initialize any necessary data, e.g., secret key
        Self {
            secret: secret.into(),
        }
    }

    pub fn generate_token(
        user_id: i64,
        user_name: &str,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        todo!("Implement token creation")
    }

    pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        todo!("Implement token verification")
    }

}
