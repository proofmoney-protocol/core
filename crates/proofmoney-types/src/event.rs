use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    StartingState,
    Release,
    Transfer,
    PublicFund,
    Rule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub event_type: EventType,
    pub height: u64,
    pub timestamp: i64,
    pub rule_version: String,
    pub payload: serde_json::Value,
    pub hash: String,
}
