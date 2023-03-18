use crate::encoder::base64;
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};

pub fn hash(key: String) -> String {
    let encoded = base64(&key);
    let salt = SaltString::from_b64(&encoded).unwrap();

    let argon2 = Argon2::default(); // Argon2 with default params (Argon2id v19)

    let password_hash = argon2.hash_password(key.as_bytes(), &salt).unwrap();

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
