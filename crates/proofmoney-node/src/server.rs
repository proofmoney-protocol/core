use serde::Serialize;

use crate::{
    ApiErrorResponse, EventListQuery, NodeConfig, NodeService, ProofQueryRequest, ProofQueryType,
};

#[derive(Debug, Clone)]
pub struct LocalReadApiServer {
    service: NodeService,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerBindConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiServerResponse {
    pub status_code: u16,
    pub body: String,
}

impl LocalReadApiServer {
    pub fn new(config: NodeConfig) -> Self {
        Self {
            service: NodeService::new(config),
        }
    }

    pub fn bind_config(&self) -> ServerBindConfig {
        ServerBindConfig {
            host: self.service.config().bind_host.clone(),
            port: self.service.config().bind_port,
        }
    }

    pub fn handle_get(&self, path: &str) -> ApiServerResponse {
        match path {
            "/health" => json_ok(&self.service.health()),
            "/status" => json_ok(&self.service.status()),
            "/ledger/status" => json_ok(&self.service.ledger_status_placeholder()),
            "/events" => json_ok(&self.service.event_list_placeholder(EventListQuery::default())),
            "/events/releases" => json_ok(&self.service.event_list_placeholder(EventListQuery {
                event_type: Some("release".to_string()),
                ..EventListQuery::default()
            })),
            "/events/transfers" => json_ok(&self.service.event_list_placeholder(EventListQuery {
                event_type: Some("transfer".to_string()),
                ..EventListQuery::default()
            })),
            _ if path.starts_with("/proofs/") => {
                let proof_type = path.trim_start_matches("/proofs/");
                match ProofQueryType::from_path(proof_type) {
                    Some(proof_type) => json_ok(&self.service.proof_query_placeholder(
                        ProofQueryRequest { proof_type },
                    )),
                    None => json_error(
                        404,
                        &NodeService::unsupported_proof_type_error(proof_type),
                    ),
                }
            }
            _ => json_error(404, &NodeService::unsupported_route_error(path)),
        }
    }
}

fn json_ok<T: Serialize>(value: &T) -> ApiServerResponse {
    ApiServerResponse {
        status_code: 200,
        body: serde_json::to_string(value).expect("API response should serialize"),
    }
}

fn json_error(status_code: u16, value: &ApiErrorResponse) -> ApiServerResponse {
    ApiServerResponse {
        status_code,
        body: serde_json::to_string(value).expect("API error should serialize"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_bind_is_localhost_only() {
        let server = LocalReadApiServer::new(NodeConfig::default());
        let bind = server.bind_config();

        assert_eq!(bind.host, "127.0.0.1");
        assert_eq!(bind.port, 8787);
    }

    #[test]
    fn health_endpoint_returns_json() {
        let server = LocalReadApiServer::new(NodeConfig::default());
        let response = server.handle_get("/health");

        assert_eq!(response.status_code, 200);
        assert!(response.body.contains("proofmoney-node"));
        assert!(response.body.contains("safety_notice"));
    }

    #[test]
    fn status_endpoint_returns_json() {
        let server = LocalReadApiServer::new(NodeConfig::default());
        let response = server.handle_get("/status");

        assert_eq!(response.status_code, 200);
        assert!(response.body.contains("write_api_enabled"));
        assert!(response.body.contains("faucet_enabled"));
        assert!(response.body.contains("false"));
    }

    #[test]
    fn ledger_status_endpoint_returns_json() {
        let server = LocalReadApiServer::new(NodeConfig::default());
        let response = server.handle_get("/ledger/status");

        assert_eq!(response.status_code, 200);
        assert!(response.body.contains("current_height"));
        assert!(response.body.contains("public_proof_fund_balance"));
    }

    #[test]
    fn proof_endpoint_supports_known_types() {
        let server = LocalReadApiServer::new(NodeConfig::default());

        for path in [
            "/proofs/starting-state",
            "/proofs/supply",
            "/proofs/rule",
            "/proofs/flow",
            "/proofs/integrity-status",
        ] {
            let response = server.handle_get(path);
            assert_eq!(response.status_code, 200);
            assert!(response.body.contains("not_executed"));
        }
    }

    #[test]
    fn unsupported_proof_type_returns_structured_error() {
        let server = LocalReadApiServer::new(NodeConfig::default());
        let response = server.handle_get("/proofs/unknown");

        assert_eq!(response.status_code, 404);
        assert!(response.body.contains("unsupported_proof_type"));
        assert!(response.body.contains("safety_notice"));
    }

    #[test]
    fn event_endpoints_return_empty_lists() {
        let server = LocalReadApiServer::new(NodeConfig::default());

        for path in ["/events", "/events/releases", "/events/transfers"] {
            let response = server.handle_get(path);
            assert_eq!(response.status_code, 200);
            assert!(response.body.contains("events"));
            assert!(response.body.contains("safety_notice"));
        }
    }

    #[test]
    fn unsupported_route_returns_structured_error() {
        let server = LocalReadApiServer::new(NodeConfig::default());
        let response = server.handle_get("/unknown");

        assert_eq!(response.status_code, 404);
        assert!(response.body.contains("unsupported_route"));
    }
}
