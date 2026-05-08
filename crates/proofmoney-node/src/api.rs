use serde::{Deserialize, Serialize};

use crate::{NodeConfig, TESTNET_SAFETY_NOTICE};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub node_version: String,
    pub mode: String,
    pub safety_notice: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NodeStatusResponse {
    pub node_version: String,
    pub network: String,
    pub protocol_version: String,
    pub mode: String,
    pub ledger_height: u64,
    pub event_count: usize,
    pub last_event_hash: Option<String>,
    pub sync_status: String,
    pub write_api_enabled: bool,
    pub faucet_enabled: bool,
    pub safety_notice: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LedgerStatusResponse {
    pub ledger_version: String,
    pub current_height: u64,
    pub current_supply: String,
    pub public_proof_fund_balance: String,
    pub rule_version: String,
    pub event_count: usize,
    pub last_event_hash: Option<String>,
    pub network: String,
    pub safety_notice: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofQueryRequest {
    pub proof_type: ProofQueryType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProofQueryType {
    StartingState,
    ProofOfSupply,
    ProofOfRule,
    ProofOfFlow,
    IntegrityStatus,
}

impl ProofQueryType {
    pub fn from_path(value: &str) -> Option<Self> {
        match value {
            "starting-state" => Some(Self::StartingState),
            "supply" => Some(Self::ProofOfSupply),
            "rule" => Some(Self::ProofOfRule),
            "flow" => Some(Self::ProofOfFlow),
            "integrity-status" => Some(Self::IntegrityStatus),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProofQueryResponse {
    pub proof_type: ProofQueryType,
    pub status: String,
    pub rule_version: String,
    pub data: serde_json::Value,
    pub summary: String,
    pub safety_notice: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventListQuery {
    pub limit: usize,
    pub cursor: Option<String>,
    pub event_type: Option<String>,
}

impl Default for EventListQuery {
    fn default() -> Self {
        Self {
            limit: 50,
            cursor: None,
            event_type: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventSummary {
    pub event_id: String,
    pub event_type: String,
    pub height: u64,
    pub timestamp: i64,
    pub event_hash: String,
    pub event_hash_valid: bool,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventListResponse {
    pub network: String,
    pub events: Vec<EventSummary>,
    pub limit: usize,
    pub next_cursor: Option<String>,
    pub safety_notice: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventSubmissionRequest {
    pub event: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventSubmissionResponse {
    pub accepted: bool,
    pub disabled_by_default: bool,
    pub event_id: Option<String>,
    pub event_hash: Option<String>,
    pub rejection_reason: Option<String>,
    pub safety_notice: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FaucetRequest {
    pub address: String,
    pub requested_amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FaucetResponse {
    pub accepted: bool,
    pub disabled_by_default: bool,
    pub address: String,
    pub amount: Option<String>,
    pub rate_limit_status: String,
    pub denial_reason: Option<String>,
    pub no_monetary_value_notice: String,
    pub no_airdrop_notice: String,
    pub no_test_to_main_conversion_notice: String,
    pub safety_notice: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApiErrorResponse {
    pub code: String,
    pub message: String,
    pub recoverable: bool,
    pub safety_notice: String,
}

impl HealthResponse {
    pub fn from_config(config: &NodeConfig) -> Self {
        Self {
            status: "ok".to_string(),
            service: "proofmoney-node".to_string(),
            node_version: config.node_version.clone(),
            mode: format!("{:?}", config.mode),
            safety_notice: TESTNET_SAFETY_NOTICE.to_string(),
        }
    }
}

impl NodeStatusResponse {
    pub fn from_config(config: &NodeConfig) -> Self {
        Self {
            node_version: config.node_version.clone(),
            network: config.network.clone(),
            protocol_version: config.protocol_version.clone(),
            mode: format!("{:?}", config.mode),
            ledger_height: 0,
            event_count: 0,
            last_event_hash: None,
            sync_status: "not_started".to_string(),
            write_api_enabled: config.write_api_enabled,
            faucet_enabled: config.faucet_enabled,
            safety_notice: TESTNET_SAFETY_NOTICE.to_string(),
        }
    }
}

impl EventSubmissionResponse {
    pub fn disabled() -> Self {
        Self {
            accepted: false,
            disabled_by_default: true,
            event_id: None,
            event_hash: None,
            rejection_reason: Some("event submission API is disabled by default".to_string()),
            safety_notice: TESTNET_SAFETY_NOTICE.to_string(),
        }
    }
}

impl FaucetResponse {
    pub fn disabled(address: impl Into<String>) -> Self {
        Self {
            accepted: false,
            disabled_by_default: true,
            address: address.into(),
            amount: None,
            rate_limit_status: "not_evaluated".to_string(),
            denial_reason: Some("testnet faucet API is disabled by default".to_string()),
            no_monetary_value_notice: "Testnet units have no monetary value.".to_string(),
            no_airdrop_notice: "Testnet faucet distribution is not an airdrop and does not imply future PRM allocation.".to_string(),
            no_test_to_main_conversion_notice: "Testnet units do not convert to mainnet PRM.".to_string(),
            safety_notice: TESTNET_SAFETY_NOTICE.to_string(),
        }
    }
}
