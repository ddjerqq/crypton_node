use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Instant;
use crate::block::Block;
use crate::util::hash_ring::HashRing;

mod transaction;
mod wallet;
mod crypto;
mod block;
mod parallel_miner;
mod util;

const MINERS: usize = 12;

fn miner(
    idx: usize,
    difficulty: &u64,
    mut payload: [u8; 96],
    hash_ring: &HashRing,
    valid_nonce: Arc<AtomicU64>,
) {
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
        let hash = crypto::sha256::hash(&payload);

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

fn main() {
    let mut block = Block::genesis();
    block.difficulty = 3;

    let valid_nonce: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));
    let hash_ring = HashRing::new(MINERS);

    let mut threads = Vec::with_capacity(MINERS);

    let start = Instant::now();

    for idx in 0..MINERS {
        let hash_ring = hash_ring.clone();
        let payload = block.get_payload();
        let nonce = Arc::clone(&valid_nonce);

        threads.push(thread::spawn(move || {
            miner(
                idx,
                &block.difficulty,
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
    println!("{:#?}", block);
    println!("{:#?}", start.elapsed());
}
