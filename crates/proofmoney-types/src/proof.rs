use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProofStatus {
    Valid,
    Invalid,
    Warning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofResult {
    pub proof_type: String,
    pub status: ProofStatus,
    pub rule_version: String,
    pub data: serde_json::Value,
    pub summary: String,
}

impl ProofResult {
    pub fn valid(
        proof_type: impl Into<String>,
        rule_version: impl Into<String>,
        data: serde_json::Value,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            proof_type: proof_type.into(),
            status: ProofStatus::Valid,
            rule_version: rule_version.into(),
            data,
            summary: summary.into(),
        }
    }

    pub fn invalid(
        proof_type: impl Into<String>,
        rule_version: impl Into<String>,
        data: serde_json::Value,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            proof_type: proof_type.into(),
            status: ProofStatus::Invalid,
            rule_version: rule_version.into(),
            data,
            summary: summary.into(),
        }
    }

    pub fn warning(
        proof_type: impl Into<String>,
        rule_version: impl Into<String>,
        data: serde_json::Value,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            proof_type: proof_type.into(),
            status: ProofStatus::Warning,
            rule_version: rule_version.into(),
            data,
            summary: summary.into(),
        }
    }
}
