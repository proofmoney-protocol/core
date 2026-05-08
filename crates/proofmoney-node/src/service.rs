use crate::{
    ApiErrorResponse, EventListQuery, EventListResponse, EventSubmissionRequest,
    EventSubmissionResponse, FaucetRequest, FaucetResponse, LedgerStatusResponse, NodeConfig,
    NodeStatusResponse, ProofQueryRequest, ProofQueryResponse, TESTNET_SAFETY_NOTICE,
};

#[derive(Debug, Clone)]
pub struct NodeService {
    config: NodeConfig,
}

impl NodeService {
    pub fn new(config: NodeConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &NodeConfig {
        &self.config
    }

    pub fn status(&self) -> NodeStatusResponse {
        NodeStatusResponse::from_config(&self.config)
    }

    pub fn ledger_status_placeholder(&self) -> LedgerStatusResponse {
        LedgerStatusResponse {
            ledger_version: "local-mvp-compatible".to_string(),
            current_height: 0,
            current_supply: "0.00000000".to_string(),
            public_proof_fund_balance: "0.00000000".to_string(),
            rule_version: "v1".to_string(),
            event_count: 0,
            last_event_hash: None,
            network: self.config.network.clone(),
            safety_notice: TESTNET_SAFETY_NOTICE.to_string(),
        }
    }

    pub fn proof_query_placeholder(&self, request: ProofQueryRequest) -> ProofQueryResponse {
        ProofQueryResponse {
            proof_type: request.proof_type,
            status: "not_executed".to_string(),
            rule_version: "v1".to_string(),
            data: serde_json::json!({
                "placeholder": true,
                "reason": "proof query API model only"
            }),
            summary: "Proof query model placeholder. No live public testnet is running.".to_string(),
            safety_notice: TESTNET_SAFETY_NOTICE.to_string(),
        }
    }

    pub fn event_list_placeholder(&self, query: EventListQuery) -> EventListResponse {
        EventListResponse {
            network: self.config.network.clone(),
            events: Vec::new(),
            limit: query.limit,
            next_cursor: None,
            safety_notice: TESTNET_SAFETY_NOTICE.to_string(),
        }
    }

    pub fn submit_event_disabled(
        &self,
        _request: EventSubmissionRequest,
    ) -> EventSubmissionResponse {
        EventSubmissionResponse::disabled()
    }

    pub fn faucet_disabled(&self, request: FaucetRequest) -> FaucetResponse {
        FaucetResponse::disabled(request.address)
    }

    pub fn unsupported_proof_type_error() -> ApiErrorResponse {
        ApiErrorResponse {
            code: "unsupported_proof_type".to_string(),
            message: "Unsupported proof type for the testnet node/API skeleton.".to_string(),
            recoverable: true,
            safety_notice: TESTNET_SAFETY_NOTICE.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ProofQueryType, TESTNET_SAFETY_NOTICE};

    #[test]
    fn node_status_serializes_with_safety_notice() {
        let service = NodeService::new(NodeConfig::default());
        let status = service.status();
        let json = serde_json::to_string(&status).expect("status should serialize");

        assert!(json.contains("v1.2.0-testnet-node-api"));
        assert!(json.contains("proofmoney-testnet-design"));
        assert!(json.contains("safety_notice"));
        assert_eq!(status.safety_notice, TESTNET_SAFETY_NOTICE);
        assert!(!status.write_api_enabled);
        assert!(!status.faucet_enabled);
    }

    #[test]
    fn ledger_status_placeholder_serializes() {
        let service = NodeService::new(NodeConfig::default());
        let status = service.ledger_status_placeholder();
        let json = serde_json::to_string(&status).expect("ledger status should serialize");

        assert!(json.contains("current_height"));
        assert!(json.contains("public_proof_fund_balance"));
        assert!(json.contains("safety_notice"));
    }

    #[test]
    fn proof_query_placeholder_serializes() {
        let service = NodeService::new(NodeConfig::default());
        let response = service.proof_query_placeholder(ProofQueryRequest {
            proof_type: ProofQueryType::ProofOfSupply,
        });

        let json = serde_json::to_string(&response).expect("proof response should serialize");
        assert!(json.contains("proof_of_supply"));
        assert!(json.contains("not_executed"));
    }

    #[test]
    fn event_list_placeholder_serializes() {
        let service = NodeService::new(NodeConfig::default());
        let response = service.event_list_placeholder(EventListQuery {
            limit: 50,
            cursor: None,
            event_type: None,
        });

        let json = serde_json::to_string(&response).expect("event list should serialize");
        assert!(json.contains("events"));
        assert_eq!(response.limit, 50);
    }

    #[test]
    fn event_submission_is_disabled_by_default() {
        let service = NodeService::new(NodeConfig::default());
        let response = service.submit_event_disabled(EventSubmissionRequest {
            event: serde_json::json!({ "type": "placeholder" }),
        });

        assert!(!response.accepted);
        assert!(response.disabled_by_default);
        assert!(response.rejection_reason.unwrap().contains("disabled"));
    }

    #[test]
    fn faucet_is_disabled_by_default_and_non_monetary() {
        let service = NodeService::new(NodeConfig::default());
        let response = service.faucet_disabled(FaucetRequest {
            address: "tprm1example".to_string(),
            requested_amount: "1.00000000".to_string(),
        });

        assert!(!response.accepted);
        assert!(response.disabled_by_default);
        assert!(response.no_monetary_value_notice.contains("no monetary value"));
        assert!(response.no_airdrop_notice.contains("not an airdrop"));
        assert!(response
            .no_test_to_main_conversion_notice
            .contains("do not convert"));
    }
}
