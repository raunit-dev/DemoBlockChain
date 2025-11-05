use std::sync::Mutex;

use crate::models::Blockchain;

pub struct AppState {
    pub blockchain: Mutex<Blockchain>,
}

impl AppState {
    pub fn new(blockchain: Blockchain) -> Self {
        Self {
            blockchain: Mutex::new(blockchain),
        }
    }
}
