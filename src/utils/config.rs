use std::env;

pub struct Config {
    pub port: String,
    pub host: String,
    pub difficulty: usize,
    pub blockchain_file: String,
}

impl Config {
    pub fn from_env() -> Self {
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

        let blockchain_file = env::var("BLOCKCHAIN_FILE")
            .unwrap_or_else(|_| "blockchain.json".to_string());

        Self {
            port,
            host,
            difficulty,
            blockchain_file,
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
