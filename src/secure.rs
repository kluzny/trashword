use crate::encoder::encode;
use rand::{rngs::StdRng, RngCore, SeedableRng};

pub fn generate_password(length: u16) {
    println!("{}", secure_pass(length));
}

fn secure_pass(length: u16) -> String {
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
    fn test_secure_pass() {
        assert_eq!(secure_pass(1).len(), 1);
        assert_eq!(secure_pass(256).len(), 256);
    }

    #[test]
    fn test_secure_pass_is_not_idempotent() {
        assert_ne!(secure_pass(256), secure_pass(256)) //  all knowledge degenerates into probability - hume
    }
}