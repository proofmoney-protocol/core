use anyhow::{anyhow, Result};
use std::path::PathBuf;

pub fn proofmoney_home() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| anyhow!("home directory not found"))?;
    Ok(home.join(".proofmoney"))
}

pub fn ledger_path() -> Result<PathBuf> {
    Ok(proofmoney_home()?.join("ledger.json"))
}

pub fn wallet_dir() -> Result<PathBuf> {
    Ok(proofmoney_home()?.join("wallets"))
}

pub fn default_wallet_path() -> Result<PathBuf> {
    Ok(wallet_dir()?.join("default.json"))
}
