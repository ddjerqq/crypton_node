use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use crate::block::Block;
use crate::crypto::sha256;
use crate::util::hash_ring::HashRing;

fn mine_single_core(block: &mut Block) {
    let mut buffer = block.get_payload();

    loop {
        // hash
        buffer[88..96].copy_from_slice(&block.nonce.to_be_bytes());
        let hash = sha256::hash(buffer);

        let is_valid = hash.iter()
            .take(block.difficulty as usize)
            .all(|b| *b == 0u8);

        if is_valid {
            break;
        }

        // increment
        block.nonce += 1;
    }
}

fn miner_thread(idx: usize, difficulty: &u64, mut payload: [u8; 96], hash_ring: &HashRing, valid_nonce: Arc<AtomicU64>) {
    // fetch
    for nonce in (0..)
        // only try nonces which belong to this thread.
        .filter(|n| hash_ring.get(*n) == idx) {

        // check to stop
        // if the valid_nonce is already found, stop execution
        if valid_nonce.load(Ordering::SeqCst) != 0
        {
            return;
        }

        // set up
        payload[88..96].copy_from_slice(&nonce.to_be_bytes());
        let hash = sha256::hash(&payload);

        // check
        let is_valid = hash.iter()
            .take(*difficulty as usize)
            .all(|b| *b == 0u8);

        if is_valid {
            // set
            valid_nonce.store(nonce as u64, Ordering::SeqCst);
            return;
        }
    }
}

fn mine_multi_core(block: &mut Block, parallelism_available: usize) {
    let valid_nonce: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));
    let hash_ring = HashRing::new(parallelism_available);

    let mut threads = Vec::with_capacity(parallelism_available);

    for idx in 0..parallelism_available {
        let hash_ring = hash_ring.clone();
        let payload = block.get_payload();
        let difficulty = block.difficulty;
        let nonce = Arc::clone(&valid_nonce);

        threads.push(thread::spawn(move || {
            miner_thread(
                idx,
                &difficulty,
                payload.clone(),
                &hash_ring,
                Arc::clone(&nonce),
            );
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    block.nonce = valid_nonce.load(Ordering::Relaxed);
}

pub fn mine(block: &mut Block) {
    let parallelism_available = thread::available_parallelism()
        .map(|s| s.get())
        .unwrap_or(1);

    if block.difficulty >= 2 && parallelism_available > 1 {
        mine_multi_core(block, parallelism_available);
    } else {
        mine_single_core(block);
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use once_cell::sync::Lazy;
    use crate::transaction::Transaction;
    use crate::wallet::Wallet;
    use super::*;

    static ALICE: Lazy<Wallet> = Lazy::new(|| Wallet::from_passphrase("alice"));
    static BOB: Lazy<Wallet> = Lazy::new(|| Wallet::from_passphrase("bob"));

    fn add_transactions(block: &mut Block) {
        for i in 0..1024 {
            let txn = Transaction::new(&ALICE, &BOB, i, 1);
            block.add_transaction(&txn);
        }
    }

    #[test]
    fn test_mine() {
        for d in 1..=3 {
            println!("difficulty: {}", d);
            let mut block = Block::genesis();
            block.difficulty = d;

            let start = Instant::now();
            mine(&mut block);
            println!("block: {:#?}", block);
            println!("elapsed: {:?}\n", start.elapsed());
        }
    }

    #[test]
    fn test_custom_block() {
        let mut block = Block::genesis();
        block.difficulty = 2;

        let alice = Wallet::from_passphrase("alice");
        let bob = Wallet::from_passphrase("bob");

        for i in 0..1024 {
            let txn = Transaction::new(&alice, &bob, i, 1);
            block.add_transaction(&txn);
        }

        println!("{:#?}", block);
    }

    #[test]
    fn test_block_chain() {
        let mut block = Block::genesis();
        block.difficulty = 2;

        for _ in 0..10 {
            add_transactions(&mut block);
            mine(&mut block);

            let start = Instant::now();
            println!("{:#?}", block);
            println!("elapsed {:?}\n", start.elapsed());

            block = block.next();
        }
    }
}