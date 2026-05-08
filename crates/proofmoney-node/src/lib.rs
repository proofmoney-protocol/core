pub mod api;
pub mod config;
pub mod server;
pub mod service;

pub use api::*;
pub use config::*;
pub use server::*;
pub use service::*;

pub const TESTNET_SAFETY_NOTICE: &str = "ProofMoney local read-only testnet API server skeleton only. Not a live public testnet, not mainnet, not a token sale, not an airdrop, not a yield product, not an exchange integration, and not a production wallet.";
