use std::time::Instant;
use once_cell::sync::Lazy;
use crate::block::Block;
use crate::miner::mine;
use crate::transaction::Transaction;
use crate::wallet::Wallet;

mod transaction;
mod wallet;
mod crypto;
mod block;
mod parallel_miner;
mod util;
mod miner;

static ALICE: Lazy<Wallet> = Lazy::new(|| Wallet::from_passphrase("alice"));
static BOB: Lazy<Wallet> = Lazy::new(|| Wallet::from_passphrase("bob"));

fn add_transactions(block: &mut Block) {
    for i in 0..1024 {
        let txn = Transaction::new(&ALICE, &BOB, i, 1);
        block.add_transaction(&txn);
    }
}

fn main() {
    let mut block = Block::genesis();
    block.difficulty = 3;

    for _ in 0..100 {
        add_transactions(&mut block);

        let start = Instant::now();
        mine(&mut block);
        println!("{:#?}", block);
        println!("elapsed {:?}\n", start.elapsed());

        block = block.next();
    }
}