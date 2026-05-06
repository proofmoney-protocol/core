use serde::{Deserialize, Serialize};

use crate::Amount;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferEvent {
    pub id: String,
    pub from: String,
    pub to: String,
    pub amount: Amount,
    pub fee: Amount,
    pub signature: Option<String>,
    pub rule_version: String,
    pub timestamp: i64,
}
