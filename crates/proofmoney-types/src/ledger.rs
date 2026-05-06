use serde::{Deserialize, Serialize};

use crate::{Amount, Event};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerState {
    pub ledger_version: String,
    pub current_height: u64,
    pub current_supply: Amount,
    pub public_fund_balance: Amount,
    pub rule_version: String,
    pub last_event_hash: Option<String>,
    pub events: Vec<Event>,
}

impl LedgerState {
    pub fn new(rule_version: &str) -> Self {
        Self {
            ledger_version: "v1".to_string(),
            current_height: 0,
            current_supply: Amount::zero(),
            public_fund_balance: Amount::zero(),
            rule_version: rule_version.to_string(),
            last_event_hash: None,
            events: Vec::new(),
        }
    }
}
