use serde::{Deserialize, Serialize};

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

    pub fn is_valid(&self) -> bool {
        self.amount > 0.0 && !self.from.trim().is_empty() && !self.to.trim().is_empty()
    }
}
