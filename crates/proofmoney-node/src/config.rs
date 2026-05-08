use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NodeConfig {
    pub node_version: String,
    pub protocol_version: String,
    pub network: String,
    pub mode: NodeMode,
    pub write_api_enabled: bool,
    pub faucet_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NodeMode {
    LocalModelOnly,
    TestnetDesign,
    TestnetSkeleton,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            node_version: "v1.2.0-testnet-node-api".to_string(),
            protocol_version: "v0.1".to_string(),
            network: "proofmoney-testnet-design".to_string(),
            mode: NodeMode::TestnetSkeleton,
            write_api_enabled: false,
            faucet_enabled: false,
        }
    }
}
