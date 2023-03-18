use crate::encoder::base64;
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};

// TODO: handle the 3 unwraps
pub fn hash(key: String) -> String {
    let encoded = base64(&key);

    // TODO: panic if short, truncate if > Salt::RECOMMENDED_LENGTH (16 bytes)
    let salt = SaltString::from_b64(&encoded).unwrap();

    // let's hope this isn't too sensitive to updates lest we blow away our secrets
    let argon2 = Argon2::default(); // Argon2 with default params (Argon2id v19)

    // TODO: investigate failure modes
    let password_hash = argon2.hash_password(key.as_bytes(), &salt).unwrap();

    // TODO: log the full PHC form on verbose 3

    password_hash.hash.unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let key = String::from("password");
        let result = hash(key);
        assert!(result.len() > 1)
    }

    #[test]
    fn test_hash_is_idempotent() {
        let key = String::from("password");
        assert_eq!(hash(key.clone()), hash(key));
    }
}
