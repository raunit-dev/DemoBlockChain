pub mod blockchain_handlers;

pub use blockchain_handlers::{
    add_block, get_chain, get_difficulty, health_check, latest_block, update_difficulty,
    validate_chain,
};
