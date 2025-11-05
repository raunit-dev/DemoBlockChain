use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use chrono::Utc;
use dotenv::dotenv;
use env_logger;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub transaction: Vec<Txn>,
    pub prev_hash: String,
    pub current_hash: String,
    pub timestampe: i64,
    pub nonce: u64,
}

impl Block {
    pub fn create(new_transaction: Vec<Txn>, prev_hash: String) -> Self {
        Block {
            timestampe: Utc::now().timestamp(),
            transaction: new_transaction,
            prev_hash,
            nonce: 0,
            current_hash: String::new(),
        }
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{:?}{:?}{:?}{:?}",
            self.timestampe, self.transaction, self.prev_hash, self.nonce
        ));
        format!("{:x}", hasher.finalize())
    }
    fn mine_block(&mut self, difficulty: usize) {
        loop {
            let prefix = "0".repeat(difficulty);
            if self.prev_hash.starts_with(&prefix) {
                break;
            }
            self.nonce += 1;
            self.prev_hash = self.calculate_hash();
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Txn {
    amount: f64,
    from: String,
    to: String,
}

impl Txn {
    pub fn new() -> Self {
        Self {
            amount: 0.0,
            from: String::new(),
            to: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::create(vec![], 0.to_string());
        Blockchain {
            blocks: vec![genesis_block],
            difficulty: 2,
        }
    }

    fn add_block(&mut self, transactions: Vec<Txn>) {
        let previous_hash = match self.blocks.last() {
            Some(last) => last.current_hash.clone(),
            None => {
                return;
            }
        };
        let mut new_block = Block::create(transactions, previous_hash);
        new_block.mine_block(self.difficulty);
        self.blocks.push(new_block);
    }

    fn validate(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            let previous_block = &self.blocks[i - 1];
            if block.current_hash != block.calculate_hash() {
                return false;
            }
            if block.prev_hash != previous_block.current_hash {
                return false;
            }
        }
        true
    }

    fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self).unwrap();
        std::fs::write(filename, json)
    }

    fn load_from_file(filename: &str) -> std::io::Result<Self> {
        let data = std::fs::read_to_string(filename)?;
        let obj = serde_json::from_str(&data).unwrap();
        Ok(obj)
    }
}

struct AppState {
    blockchain: Mutex<Blockchain>,
}

// /// GET /chain
// /// Returns the entire blockchain
async fn get_chain(data: web::Data<Arc<AppState>>) -> impl Responder {
    let blockchain = data.blockchain.lock().unwrap().clone();
    HttpResponse::Ok().json(blockchain)
}

// /// POST /add_block
// /// Adds a new block with the given transactions
async fn add_block(
    data: web::Data<Arc<AppState>>,
    transactions: web::Json<Vec<Txn>>,
) -> impl Responder {
    let mut blockchain = data.blockchain.lock().unwrap();
    blockchain.add_block(transactions.into_inner());
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Block added successfully (TODO: implement logic)"
    }))
}

// /// GET /validate
// /// Checks if the blockchain is valid
async fn validate_chain(data: web::Data<Arc<AppState>>) -> impl Responder {
    let blockchain = data.blockchain.lock().unwrap();
    let valid = blockchain.validate();
    HttpResponse::Ok().json(serde_json::json!({
        "valid": valid
    }))
}
