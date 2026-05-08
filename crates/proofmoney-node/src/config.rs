use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NodeConfig {
    pub node_version: String,
    pub protocol_version: String,
    pub network: String,
    pub mode: NodeMode,
    pub bind_host: String,
    pub bind_port: u16,
    pub write_api_enabled: bool,
    pub faucet_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NodeMode {
    LocalModelOnly,
    TestnetDesign,
    TestnetSkeleton,
    LocalReadOnlyApi,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            node_version: "v1.3.0-testnet-read-api-server".to_string(),
            protocol_version: "v0.1".to_string(),
            network: "proofmoney-testnet-local-readonly".to_string(),
            mode: NodeMode::LocalReadOnlyApi,
            bind_host: "127.0.0.1".to_string(),
            bind_port: 8787,
            write_api_enabled: false,
            faucet_enabled: false,
        }
    }
}
