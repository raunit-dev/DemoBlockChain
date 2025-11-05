use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use dotenv::dotenv;
use env_logger;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::env;
use std::sync::{Arc, Mutex};

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
        // Calculate initial hash for genesis block
        block.current_hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{:?}{:?}{:?}{:?}",
            self.timestamp, self.transaction, self.prev_hash, self.nonce
        ));
        format!("{:x}", hasher.finalize())
    }

    fn mine_block(&mut self, difficulty: usize) {
        let prefix = "0".repeat(difficulty);
        loop {
            self.current_hash = self.calculate_hash(); // Fixed: Should check current_hash, not prev_hash
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Txn {
    pub amount: f64,
    pub from: String,
    pub to: String,
}

impl Txn {
    pub fn new(amount: f64, from: String, to: String) -> Self {
        Self { amount, from, to }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    fn new(difficulty: usize) -> Self {
        let mut genesis_block = Block::create(vec![], String::from("0"));
        genesis_block.current_hash = genesis_block.calculate_hash();
        
        log::info!("Creating new blockchain with difficulty: {}", difficulty);
        
        Blockchain {
            blocks: vec![genesis_block],
            difficulty,
        }
    }

    fn add_block(&mut self, transactions: Vec<Txn>) {
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

    fn validate(&self) -> bool {
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

    fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(filename, json)?;
        log::info!("Blockchain saved to {}", filename);
        Ok(())
    }

    fn load_from_file(filename: &str) -> std::io::Result<Self> {
        let data = std::fs::read_to_string(filename)?;
        let blockchain: Blockchain = serde_json::from_str(&data)?;
        Ok(blockchain)
    }

    // New method to update difficulty
    fn update_difficulty(&mut self, new_difficulty: usize) {
        self.difficulty = new_difficulty;
        log::info!("Blockchain difficulty updated to: {}", new_difficulty);
    }
}

struct AppState {
    blockchain: Mutex<Blockchain>,
}

// New struct for difficulty update request
#[derive(Deserialize)]
struct DifficultyUpdate {
    difficulty: usize,
}

async fn get_chain(data: web::Data<Arc<AppState>>) -> impl Responder {
    let blockchain = data.blockchain.lock().unwrap().clone();
    HttpResponse::Ok().json(blockchain)
}

async fn add_block(
    data: web::Data<Arc<AppState>>,
    transactions: web::Json<Vec<Txn>>,
) -> impl Responder {
    let txs = transactions.into_inner();

    // Validate transactions
    for (i, txn) in txs.iter().enumerate() {
        if txn.amount <= 0.0 || txn.from.trim().is_empty() || txn.to.trim().is_empty() {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid transaction",
                "index": i,
                "details": "Amount must be positive and from/to addresses cannot be empty"
            }));
        }
    }

    let mut blockchain = data.blockchain.lock().unwrap();
    blockchain.add_block(txs);

    HttpResponse::Ok().json(serde_json::json!({
        "message": "Block added successfully",
        "block_height": blockchain.blocks.len() - 1,
        "current_difficulty": blockchain.difficulty
    }))
}

async fn validate_chain(data: web::Data<Arc<AppState>>) -> impl Responder {
    let blockchain = data.blockchain.lock().unwrap();
    let valid = blockchain.validate();
    
    let message = if valid {
        "Blockchain is valid and integrity verified"
    } else {
        "Blockchain validation failed chain may be corrupted"
    };
    
    HttpResponse::Ok().json(serde_json::json!({
        "valid": valid,
        "message": message,
        "chain_length": blockchain.blocks.len(),
        "difficulty": blockchain.difficulty
    }))
}

async fn latest_block(data: web::Data<Arc<AppState>>) -> impl Responder {
    let blockchain = data.blockchain.lock().unwrap();
    match blockchain.blocks.last() {
        Some(block) => HttpResponse::Ok().json(serde_json::json!({
            "block": block,
            "block_number": blockchain.blocks.len() - 1,
            "current_difficulty": blockchain.difficulty
        })),
        None => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "No blocks in chain"
        })),
    }
}

async fn get_difficulty(data: web::Data<Arc<AppState>>) -> impl Responder {
    let blockchain = data.blockchain.lock().unwrap();
    HttpResponse::Ok().json(serde_json::json!({
        "current_difficulty": blockchain.difficulty,
        "total_blocks": blockchain.blocks.len()
    }))
}

async fn update_difficulty(
    data: web::Data<Arc<AppState>>,
    update: web::Json<DifficultyUpdate>,
) -> impl Responder {
    if update.difficulty == 0 || update.difficulty > 10 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Difficulty must be between 1 and 10"
        }));
    }

    let mut blockchain = data.blockchain.lock().unwrap();
    let old_difficulty = blockchain.difficulty;
    blockchain.update_difficulty(update.difficulty);
    
    // Save the updated blockchain
    if let Err(e) = blockchain.save_to_file("blockchain.json") {
        log::error!("Failed to save blockchain after difficulty update: {}", e);
    }

    HttpResponse::Ok().json(serde_json::json!({
        "message": "Difficulty updated successfully",
        "old_difficulty": old_difficulty,
        "new_difficulty": update.difficulty
    }))
}

// Health check endpoint
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "blockchain-api"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    
    // Get difficulty from environment variable, default to 2 if not set
    let difficulty: usize = env::var("DIFFICULTY")
        .unwrap_or_else(|_| "2".to_string())
        .parse()
        .unwrap_or(2);
    
    // Validate difficulty range
    let difficulty = if difficulty == 0 || difficulty > 10 {
        log::warn!("Invalid DIFFICULTY value. Using default: 2");
        2
    } else {
        log::info!("Starting with mining difficulty: {}", difficulty);
        difficulty
    };
    
    let address = format!("{}:{}", host, port);

    let filename = "blockchain.json";
    let mut blockchain = match Blockchain::load_from_file(filename) {
        Ok(mut chain) => {
            log::info!("Loaded existing blockchain from {}", filename);
            log::info!("Chain has {} blocks, difficulty: {}", chain.blocks.len(), chain.difficulty);
            
            // Optionally update difficulty if env variable is different
            if chain.difficulty != difficulty {
                log::info!("Updating difficulty from {} to {}", chain.difficulty, difficulty);
                chain.update_difficulty(difficulty);
                chain.save_to_file(filename).ok();
            }
            
            chain
        }
        Err(err) => {
            log::warn!("Could not load {}: {}. Starting new chain...", filename, err);
            Blockchain::new(difficulty)
        }
    };

    if blockchain.validate() {
        log::info!("Blockchain validation passed");
    } else {
        log::error!("Blockchain validation failed! Consider starting fresh.");
    }

    let app_state = Arc::new(AppState {
        blockchain: Mutex::new(blockchain),
    });

    log::info!("Starting server at http://{}", address);

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(app_state.clone()))
            .route("/health", web::get().to(health_check))
            .route("/chain", web::get().to(get_chain))
            .route("/add_block", web::post().to(add_block))
            .route("/validate", web::get().to(validate_chain))
            .route("/latest_block", web::get().to(latest_block))
            .route("/get_difficulty", web::get().to(get_difficulty))
            .route("/update_difficulty", web::put().to(update_difficulty))
    })
    .bind(&address)?
    .run()
    .await
}