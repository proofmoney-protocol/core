use anyhow::Result;
use proofmoney_types::LedgerState;

use crate::{ledger_path, load_json, save_json};

pub fn save_ledger(state: &LedgerState) -> Result<()> {
    save_json(&ledger_path()?, state)
}

pub fn load_ledger() -> Result<LedgerState> {
    load_json(&ledger_path()?)
}

pub fn load_or_init_ledger(rule_version: &str) -> Result<LedgerState> {
    let path = ledger_path()?;

    if path.exists() {
        load_json(&path)
    } else {
        let state = LedgerState::new(rule_version);
        save_json(&path, &state)?;
        Ok(state)
    }
}
