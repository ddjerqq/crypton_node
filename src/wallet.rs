use secp256k1::ecdsa::Signature;
use secp256k1::Message;
use crate::crypto::ecdsa::Ecdsa;
use crate::crypto::sha256;

#[derive(Debug)]
pub struct Wallet {
    pub ecdsa: Ecdsa,
    pub balance: u64,
    pub address: String,
}

impl Wallet {
    pub fn new(ecdsa: Ecdsa) -> Self {
        let address = {
            let p_key_bytes = ecdsa.p_key.serialize();

            let hash = sha256::hash(&p_key_bytes);
            let digest = sha256::digest(&hash);

            format!("0x{}", digest[22..64].to_uppercase().to_string())
        };

        Self {
            ecdsa,
            address,
            balance: 0,
        }
    }

    pub fn from_passphrase(passphrase: &str) -> Self {
        let ecdsa = Ecdsa::from_passphrase(passphrase);
        Self::new(ecdsa)
    }

    pub fn sign(&self, data: &[u8]) -> Signature {
        let hash = sha256::hash(data);

        let msg = Message::from_digest_slice(&hash)
            // because hash is always 32 bytes long
            .unwrap();

        self.ecdsa.sign(&msg)
    }

    pub fn verify(&self, data: &[u8], sig: &Signature) -> bool {
        let hash = sha256::hash(data);

        let msg = Message::from_digest_slice(&hash)
            // because hash is always 32 bytes long
            .unwrap();

        self.ecdsa.verify(&msg, &sig)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_new() {
        let wallet = Wallet::new(Ecdsa::new());
        let data = "test".as_bytes();
        let sig = wallet.sign(data);

        println!("{}", wallet.address);

        assert!(wallet.verify(data, &sig));
    }

    #[test]
    fn test_address() {
        for _ in 0..10 {
            let wallet = Wallet::from_passphrase("test phone elliptic curve");
            println!("{}", wallet.address);
        }
    }

    #[test]
    fn test_wallet_from_passphrase() {
        let wallet = Wallet::from_passphrase("test phone elliptic curve");
        let data = "test".as_bytes();
        let sig = wallet.sign(data);

        println!("{}", wallet.address);

        assert!(wallet.verify(data, &sig));
    }
}