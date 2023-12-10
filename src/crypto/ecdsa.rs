use crate::crypto::sha256;
use hex::encode;
use std::fmt::{Debug, Formatter};
use secp256k1::{All, Message};
use secp256k1::ecdsa::Signature;

pub struct Ecdsa {
    _curve: secp256k1::Secp256k1<All>,
    pub s_key: secp256k1::SecretKey,
    pub p_key: secp256k1::PublicKey,
}

impl Debug for Ecdsa {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s_key = encode(&self.s_key.secret_bytes());
        let p_key = encode(&self.p_key.serialize());

        write!(f, "Ecdsa S: {} P: {}", s_key, p_key)
    }
}

impl Ecdsa {
    pub fn new() -> Self {
        let curve = secp256k1::Secp256k1::new();
        let s_key = secp256k1::SecretKey::new(&mut rand::thread_rng());
        let p_key = secp256k1::PublicKey::from_secret_key(&curve, &s_key);

        Self {
            _curve: curve,
            s_key,
            p_key,
        }
    }

    pub fn from_passphrase(passphrase: &str) -> Self {
        let seed = sha256::hash(passphrase);

        let curve = secp256k1::Secp256k1::new();
        let s_key = secp256k1::SecretKey::from_slice(&seed).unwrap();
        let p_key = secp256k1::PublicKey::from_secret_key(&curve, &s_key);

        Self {
            _curve: curve,
            s_key,
            p_key,
        }
    }

    pub fn sign(&self, msg: &Message) -> Signature {
        self._curve.sign_ecdsa(msg, &self.s_key)
    }

    pub fn verify(&self, msg: &Message, sig: &Signature) -> bool {
        self._curve.verify_ecdsa(msg, sig, &self.p_key).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ecdsa = Ecdsa::new();

        let payload = b"aaa";
        let msg = Message::from_digest(sha256::hash(&payload));
        let signature = ecdsa.sign(&msg);

        assert!(ecdsa.verify(&msg, &signature));
    }

    #[test]
    fn test_from_passphrase() {
        let passphrase = "correct horse battery staple";
        let ecdsa = Ecdsa::from_passphrase(&passphrase);

        let payload = b"aaa";
        let msg = Message::from_digest(sha256::hash(&payload));
        let signature = ecdsa.sign(&msg);

        assert!(ecdsa.verify(&msg, &signature));
    }

    #[test]
    fn test_sign_verify() {
        let ecdsa = Ecdsa::new();

        let payload = b"aaa";
        let msg = Message::from_digest(sha256::hash(&payload));
        let signature = ecdsa.sign(&msg);

        assert!(ecdsa.verify(&msg, &signature));
    }
}