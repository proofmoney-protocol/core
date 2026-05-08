pub mod api;
pub mod config;
pub mod service;

pub use api::*;
pub use config::*;
pub use service::*;

pub const TESTNET_SAFETY_NOTICE: &str = "ProofMoney testnet node/API skeleton only. Not a live public testnet, not mainnet, not a token sale, not an airdrop, not a yield product, not an exchange integration, and not a production wallet.";
