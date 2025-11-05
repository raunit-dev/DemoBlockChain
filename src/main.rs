mod handlers;
mod models;
mod state;
mod utils;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use env_logger;

use handlers::{
    add_block, get_chain, get_difficulty, health_check, latest_block, update_difficulty,
    validate_chain,
};
use models::Blockchain;
use state::AppState;
use utils::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = Config::from_env();
    let address = config.address();

    let blockchain = match Blockchain::load_from_file(&config.blockchain_file) {
        Ok(mut chain) => {
            log::info!("Loaded existing blockchain from {}", config.blockchain_file);
            log::info!(
                "Chain has {} blocks, difficulty: {}",
                chain.blocks.len(),
                chain.difficulty
            );

            // Optionally update difficulty if env variable is different
            if chain.difficulty != config.difficulty {
                log::info!(
                    "Updating difficulty from {} to {}",
                    chain.difficulty,
                    config.difficulty
                );
                chain.update_difficulty(config.difficulty);
                chain.save_to_file(&config.blockchain_file).ok();
            }

            chain
        }
        Err(err) => {
            log::warn!(
                "Could not load {}: {}. Starting new chain...",
                config.blockchain_file,
                err
            );
            Blockchain::new(config.difficulty)
        }
    };

    if blockchain.validate() {
        log::info!("Blockchain validation passed");
    } else {
        log::error!("Blockchain validation failed! Consider starting fresh.");
    }

    let app_state = web::Data::new(AppState::new(blockchain));

    log::info!("Starting server at http://{}", address);

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(app_state.clone())
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