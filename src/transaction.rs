use std::fmt::{Debug, Formatter};
use std::time::UNIX_EPOCH;
use secp256k1::ecdsa::Signature;
use secp256k1::Message;
use crate::crypto::{hex, sha256};
use crate::wallet::Wallet;

const PAYLOAD_SIZE: usize = 112;
const EMPTY_HASH: [u8; 32] = [0u8; 32];
const EMPTY_SIGN: [u8; 64] = [0u8; 64];

#[derive(Clone)]
pub struct Transaction<'a> {
    pub sender: &'a Wallet,
    pub recipient: &'a Wallet,
    pub amount: u64,
    pub fee: u64,
    pub timestamp: u64,

    pub hash: [u8; 32],
    pub signature: Signature,
}

impl<'a> Transaction<'a> {
    pub fn new(sender: &'a Wallet, recipient: &'a Wallet, amount: u64, fee: u64) -> Self {
        let timestamp = UNIX_EPOCH.elapsed().unwrap().as_secs();

        let mut txn = Self {
            sender,
            recipient,
            amount,
            fee,
            timestamp,
            hash: EMPTY_HASH,
            signature: Signature::from_compact(&EMPTY_SIGN).unwrap(),
        };

        let payload = txn.get_payload();

        txn.hash = sha256::hash(payload);
        let msg = Message::from_digest(txn.hash);
        txn.signature = sender.ecdsa.sign(&msg);

        txn
    }

    pub fn get_payload(&self) -> [u8; PAYLOAD_SIZE] {
        let mut buffer = [0u8; PAYLOAD_SIZE];

        let sender_address = self.sender.address.as_bytes();
        buffer[..44].copy_from_slice(sender_address);

        let recipient_address = self.recipient.address.as_bytes();
        buffer[44..88].copy_from_slice(recipient_address);

        let amount = self.amount.to_be_bytes();
        buffer[88..96].copy_from_slice(&amount);

        let fee = self.fee.to_be_bytes();
        buffer[96..104].copy_from_slice(&fee);

        let timestamp = self.timestamp.to_be_bytes();
        buffer[104..112].copy_from_slice(&timestamp);

        buffer
    }
}

impl<'a> Debug for Transaction<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Transaction")
            .field("sender", &self.sender.address)
            .field("recipient", &self.recipient.address)
            .field("amount", &self.amount)
            .field("fee", &self.fee)
            .field("timestamp", &self.timestamp)
            .field("hash", &sha256::digest(&self.hash))
            .field("signature", &hex::encode(&self.signature.serialize_compact()))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        let s = Wallet::from_passphrase("bob");
        let r = Wallet::from_passphrase("alice");

        let txn = Transaction::new(&s, &r, 100, 1);

        println!("{:#?}", txn);
    }

    #[test]
    fn test_hash() {
        let s = Wallet::from_passphrase("bob");
        let r = Wallet::from_passphrase("alice");

        let txn = Transaction::new(&s, &r, 100, 1);

        let payload = txn.get_payload();
        let hash = sha256::hash(payload);

        assert_eq!(txn.hash, hash);
    }

    #[test]
    fn test_signature() {
        let s = Wallet::from_passphrase("bob");
        let r = Wallet::from_passphrase("alice");

        let t = Transaction::new(&s, &r, 100, 1);

        assert!(s.verify(&t.get_payload(), &t.signature));
    }
}