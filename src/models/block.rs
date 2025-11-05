use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::transaction::Txn;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub transaction: Vec<Txn>,
    pub prev_hash: String,
    pub current_hash: String,
    pub timestamp: i64,
    pub nonce: u64,
}

impl Block {
    pub fn create(new_transaction: Vec<Txn>, prev_hash: String) -> Self {
        let mut block = Block {
            timestamp: Utc::now().timestamp(),
            transaction: new_transaction,
            prev_hash,
            nonce: 0,
            current_hash: String::new(),
        };
        // Calculate initial hash
        block.current_hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{:?}{:?}{:?}{:?}",
            self.timestamp, self.transaction, self.prev_hash, self.nonce
        ));
        format!("{:x}", hasher.finalize())
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let prefix = "0".repeat(difficulty);
        loop {
            self.current_hash = self.calculate_hash();
            if self.current_hash.starts_with(&prefix) {
                break;
            }
            self.nonce += 1;
        }
        log::info!(
            "Block mined successfully! Hash: {}, Nonce: {}",
            self.current_hash,
            self.nonce
        );
    }
}
