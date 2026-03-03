use argon2::{
    Algorithm, Argon2, Params, Version,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};

use rand_core::OsRng;

fn argon2_instance() -> Argon2<'static> {
    let params = Params::new(46 * 1024, 3, 1, None)
        .expect("hardcoded Argon2 params are valid");
    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
}

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = argon2_instance();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed = PasswordHash::new(hash)?;
    let argon2 = argon2_instance();
    Ok(argon2.verify_password(password.as_bytes(), &parsed).is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_then_verify_succeeds() {
        let password = "correct-horse-battery-staple";
        let hash = hash_password(password).expect("hashing should succeed");
        let valid = verify_password(password, &hash).expect("verify should succeed");
        assert!(valid);
    }

    #[test]
    fn wrong_password_fails_verification() {
        let hash = hash_password("real-password").expect("hashing should succeed");
        let valid = verify_password("wrong-password", &hash).expect("verify should succeed");
        assert!(!valid);
    }

    #[test]
    fn hash_is_unique_per_call() {
        let password = "same-password";
        let hash1 = hash_password(password).expect("hashing should succeed");
        let hash2 = hash_password(password).expect("hashing should succeed");
        assert_ne!(hash1, hash2, "different salts should produce different hashes");
    }

    #[test]
    fn empty_password_hashes_and_verifies() {
        let hash = hash_password("").expect("hashing empty password should succeed");
        assert!(verify_password("", &hash).expect("verify should succeed"));
        assert!(!verify_password("non-empty", &hash).expect("verify should succeed"));
    }

    #[test]
    fn verify_with_invalid_hash_string_returns_error() {
        let result = verify_password("password", "not-a-valid-hash");
        assert!(result.is_err());
    }

    #[test]
    fn hash_output_is_argon2_format() {
        let hash = hash_password("test").expect("hashing should succeed");
        assert!(hash.starts_with("$argon2"), "hash should be in PHC string format");
    }
}
