use std::fmt::{Debug, Formatter};
use crate::crypto::sha256;
use crate::transaction::Transaction;
use crate::util::mrkl_root;

const PAYLOAD_SIZE: usize = 96;

pub struct Block<'a> {
    pub difficulty: u64,

    pub index: u64,
    pub timestamp: u64,
    pub nonce: u64,
    pub parent_hash: [u8; 32],

    pub transactions: Vec<Transaction<'a>>,
}

impl<'a> Block<'a> {
    pub fn genesis() -> Self {
        let block = Self {
            difficulty: 0,
            index: 0,
            timestamp: 0,
            nonce: 0,
            parent_hash: [0u8; 32],
            transactions: Vec::new(),
        };

        block
    }

    pub fn next(&self) -> Self {
        Self {
            difficulty: self.difficulty,
            index: self.index + 1,
            timestamp: 0,
            nonce: 0,
            parent_hash: self.get_hash(),
            transactions: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: &Transaction<'a>) {
        self.transactions.push(transaction.clone());
    }

    pub fn get_total_fee(&self) -> u64 {
        self.transactions
            .iter()
            .map(|txn| txn.fee)
            .sum()
    }

    pub fn get_hash(&self) -> [u8; 32] {
        sha256::hash(self.get_payload())
    }

    pub fn get_digest(&self) -> String {
        sha256::digest(&self.get_hash())
    }

    pub fn get_mrkl_root(&self) -> [u8; 32] {
        let txn_hashes: Vec<[u8; 32]> = self.transactions
            .iter()
            .map(|txn| txn.hash)
            .collect();

        mrkl_root::calculate_mrkl_root(&txn_hashes)
    }

    pub fn get_payload(&self) -> [u8; PAYLOAD_SIZE] {
        let mut buffer = [0u8; PAYLOAD_SIZE];

        let difficulty = self.difficulty.to_be_bytes();
        buffer[..8].copy_from_slice(&difficulty);

        let idx = self.index.to_be_bytes();
        buffer[8..16].copy_from_slice(&idx);

        let timestamp = self.timestamp.to_be_bytes();
        buffer[16..24].copy_from_slice(&timestamp);

        let parent_hash = self.parent_hash;
        buffer[24..56].copy_from_slice(&parent_hash);

        let mrkl_root = self.get_mrkl_root();
        buffer[56..88].copy_from_slice(&mrkl_root);

        let nonce = self.nonce.to_be_bytes();
        buffer[88..96].copy_from_slice(&nonce);

        buffer
    }

    pub fn mine(&mut self) {
        let mut buffer = self.get_payload();

        loop {
            // hash
            buffer[88..96].copy_from_slice(&self.nonce.to_be_bytes());
            let hash = sha256::hash(buffer);

            let is_valid = hash.iter()
                .take(self.difficulty as usize)
                .all(|b| *b == 0u8);

            if is_valid {
                break;
            }

            // increment
            self.nonce += 1;
        }
    }
}

impl<'a> Debug for Block<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Block")
            .field("difficulty", &self.difficulty)
            .field("index", &self.index)
            .field("timestamp", &self.timestamp)
            .field("transactions", &self.transactions.len())
            .field("parent_hash", &sha256::digest(&self.parent_hash))
            .field("mrkl_root", &sha256::digest(&self.get_mrkl_root()))
            .field("nonce", &self.nonce)
            .field("hash", &sha256::digest(&self.get_hash()))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use crate::wallet::Wallet;
    use super::*;

    #[test]
    fn test_block_hash() {
        let mut block = Block::genesis();
        block.difficulty = 2;

        let start = Instant::now();
        block.mine();
        let elapsed = start.elapsed();
        println!("{:?}", elapsed);

        println!("{:#?}", block);
    }

    #[test]
    fn test_custom_block() {
        let mut block = Block::genesis();
        block.difficulty = 2;

        let start = Instant::now();
        block.mine();
        let elapsed = start.elapsed();
        println!("{:?}", elapsed);

        let alice = Wallet::from_passphrase("alice");
        let bob = Wallet::from_passphrase("bob");

        for i in 0..1024 {
            let txn = Transaction::new(&alice, &bob, i, 1);
            block.add_transaction(&txn);
        }
        block.mine();

        println!("{:#?}", block);
    }
}