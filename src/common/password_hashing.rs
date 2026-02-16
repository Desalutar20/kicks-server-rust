use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

use crate::{Error, Result};

pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| Error::Internal(e.to_string()))?
        .to_string();

    Ok(password_hash)
}

pub fn verify(password: &str, password_hash: &str) -> Result<bool> {
    let parsed_hash =
        PasswordHash::new(password_hash).map_err(|e| Error::Internal(e.to_string()))?;

    let result = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_success() {
        let password = "password";

        let hash = hash_password(password);
        assert!(hash.is_ok());

        let result = verify(password, &hash.unwrap());
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_verify_wrong_password() {
        let password = "password";
        let wrong_password = "wrongpassword";

        let hash = hash_password(password);
        assert!(hash.is_ok());

        let result = verify(wrong_password, &hash.unwrap());
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_verify_invalid_hash() {
        let password = "password";

        let hash = hash_password(password);
        assert!(hash.is_ok());

        let result = verify(password, "invalid hash");
        assert!(result.is_err());
    }
}
