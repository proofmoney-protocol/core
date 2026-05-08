use crate::{
    ApiErrorResponse, EventListQuery, EventListResponse, EventSubmissionRequest,
    EventSubmissionResponse, FaucetRequest, FaucetResponse, HealthResponse, LedgerStatusResponse,
    NodeConfig, NodeStatusResponse, ProofQueryRequest, ProofQueryResponse, TESTNET_SAFETY_NOTICE,
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

    pub fn health(&self) -> HealthResponse {
        HealthResponse::from_config(&self.config)
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
                "reason": "local read-only API server skeleton"
            }),
            summary: "Proof query placeholder. No live public testnet is running.".to_string(),
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

    pub fn unsupported_route_error(path: &str) -> ApiErrorResponse {
        ApiErrorResponse {
            code: "unsupported_route".to_string(),
            message: format!("Unsupported local read-only API route: {path}"),
            recoverable: true,
            safety_notice: TESTNET_SAFETY_NOTICE.to_string(),
        }
    }

    pub fn unsupported_proof_type_error(proof_type: &str) -> ApiErrorResponse {
        ApiErrorResponse {
            code: "unsupported_proof_type".to_string(),
            message: format!("Unsupported proof type: {proof_type}"),
            recoverable: true,
            safety_notice: TESTNET_SAFETY_NOTICE.to_string(),
        }
    }
}
