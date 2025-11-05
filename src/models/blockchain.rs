use serde::{Deserialize, Serialize};

use super::block::Block;
use super::transaction::Txn;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut genesis_block = Block::create(vec![], String::from("0"));
        genesis_block.current_hash = genesis_block.calculate_hash();

        log::info!("Creating new blockchain with difficulty: {}", difficulty);

        Blockchain {
            blocks: vec![genesis_block],
            difficulty,
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Txn>) {
        let previous_hash = match self.blocks.last() {
            Some(last) => last.current_hash.clone(),
            None => {
                log::error!("Cannot add block: blockchain is empty");
                return;
            }
        };

        log::info!("Mining new block with {} transactions...", transactions.len());
        let mut new_block = Block::create(transactions, previous_hash);
        new_block.mine_block(self.difficulty);

        self.blocks.push(new_block);
        log::info!("Block #{} added to the chain", self.blocks.len() - 1);

        // Auto-save after adding block
        if let Err(e) = self.save_to_file("blockchain.json") {
            log::error!("Failed to auto-save blockchain: {}", e);
        }
    }

    pub fn validate(&self) -> bool {
        if self.blocks.is_empty() {
            return false;
        }

        for (i, block) in self.blocks.iter().enumerate() {
            // Skip genesis block for previous block validation
            if i == 0 {
                continue;
            }

            let previous_block = &self.blocks[i - 1];

            // Check if current hash is valid
            if block.current_hash != block.calculate_hash() {
                log::error!("Invalid hash at block {}", i);
                return false;
            }

            // Check if previous hash matches
            if block.prev_hash != previous_block.current_hash {
                log::error!("Chain broken at block {}", i);
                return false;
            }

            // Check if block meets difficulty requirement
            let prefix = "0".repeat(self.difficulty);
            if !block.current_hash.starts_with(&prefix) {
                log::error!("Block {} doesn't meet difficulty requirement", i);
                return false;
            }
        }
        true
    }

    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(filename, json)?;
        log::info!("Blockchain saved to {}", filename);
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> std::io::Result<Self> {
        let data = std::fs::read_to_string(filename)?;
        let blockchain: Blockchain = serde_json::from_str(&data)?;
        Ok(blockchain)
    }

    pub fn update_difficulty(&mut self, new_difficulty: usize) {
        self.difficulty = new_difficulty;
        log::info!("Blockchain difficulty updated to: {}", new_difficulty);
    }
}
