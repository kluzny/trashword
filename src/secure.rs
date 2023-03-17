use crate::encoder::encode;
use rand::{rngs::StdRng, RngCore, SeedableRng};

pub fn generate_password(length: u16) -> String {
    // TODO: should error if we exhaust the buffer
    // or more robustly create buffers as needed
    let buffer = random_bytes();
    let subset = &buffer[0 .. usize::from(length)];

    encode(subset)
}

fn random_bytes() -> [u8; 256] {
    let buffer: &mut [u8;256] = &mut [0; 256];
    let mut csrng = StdRng::from_entropy();
    csrng.fill_bytes(buffer);
    *buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_password() {
        assert_eq!(generate_password(1).len(), 1);
        assert_eq!(generate_password(256).len(), 256);
    }

    #[test]
    fn test_generate_password_is_not_idempotent() {
        assert_ne!(generate_password(256), generate_password(256)) //  all knowledge degenerates into probability - hume
    }
}