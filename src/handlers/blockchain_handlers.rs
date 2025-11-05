use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::models::Txn;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct DifficultyUpdate {
    pub difficulty: usize,
}

pub async fn get_chain(data: web::Data<AppState>) -> impl Responder {
    let blockchain = data.blockchain.lock().unwrap().clone();
    HttpResponse::Ok().json(blockchain)
}

pub async fn add_block(
    data: web::Data<AppState>,
    transactions: web::Json<Vec<Txn>>,
) -> impl Responder {
    let txs = transactions.into_inner();

    // Validate transactions
    for (i, txn) in txs.iter().enumerate() {
        if !txn.is_valid() {
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

pub async fn validate_chain(data: web::Data<AppState>) -> impl Responder {
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

pub async fn latest_block(data: web::Data<AppState>) -> impl Responder {
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

pub async fn get_difficulty(data: web::Data<AppState>) -> impl Responder {
    let blockchain = data.blockchain.lock().unwrap();
    HttpResponse::Ok().json(serde_json::json!({
        "current_difficulty": blockchain.difficulty,
        "total_blocks": blockchain.blocks.len()
    }))
}

pub async fn update_difficulty(
    data: web::Data<AppState>,
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

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "blockchain-api"
    }))
}
