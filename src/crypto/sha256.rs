use crate::crypto::hex;
use sha2::{Sha256, Digest};

pub fn hash<T: AsRef<[u8]>>(data: T) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();

    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&hash);

    bytes
}

pub fn digest(hash: &[u8; 32]) -> String {
    hex::encode(hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let data = "aaa";
        let expected = "9834876dcfb05cb167a5c24953eba58c4ac89b1adf57f28f2f9d09af107ee8f0";

        let hash = hash(data.as_bytes());
        let hash_hex = hex::encode(&hash);

        assert_eq!(hash_hex, expected);
    }

    #[test]
    fn test_digest() {
        let data = "aaa";
        let expected = "9834876dcfb05cb167a5c24953eba58c4ac89b1adf57f28f2f9d09af107ee8f0";

        let hash = hash(data.as_bytes());
        let digest = digest(&hash);

        assert_eq!(digest, expected);
    }
}