use base64::{engine::general_purpose, Engine as _};
use const_format::formatcp;

const LOWERS: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const CHARSET: &str = formatcp!("{LOWERS}{UPPERS}{DIGITS}");

pub fn encode(bytes: &[u8]) -> String {
    let mut out: String = String::new();
    let length = CHARSET.len();

    for byte in bytes {
        let biased = byte % (length as u8); // TODO: make sure max(byte) is divisible by length to remove bias
        let encoded = CHARSET.chars().nth(biased as usize).unwrap(); // None case shouldn't be possible under the modulo

        out.push(encoded)
    }
    out
}

pub fn base64(s: &str) -> String {
    general_purpose::STANDARD_NO_PAD.encode(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(encode(&[0; 1]), "a");
        assert_eq!(encode(&[26; 1]), "A");
        assert_eq!(encode(&[52; 1]), "0");
        assert_eq!(encode(&[25, 51, 61]), "zZ9");
    }

    #[test]
    fn test_encode_wraps_around_length() {
        assert_eq!(encode(&[CHARSET.len() as u8]), "a");
    }

    #[test]
    fn test_base64() {
        assert_eq!(base64("foo bar"), "Zm9vIGJhcg");
    }
}
