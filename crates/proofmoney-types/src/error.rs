use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProofMoneyError {
    #[error("invalid amount")]
    InvalidAmount,

    #[error("invalid starting state")]
    InvalidStartingState,

    #[error("invalid release event")]
    InvalidReleaseEvent,

    #[error("invalid ownership proof")]
    InvalidOwnershipProof,

    #[error("invalid flow proof")]
    InvalidFlowProof,

    #[error("invalid rule set")]
    InvalidRuleSet,

    #[error("storage error: {0}")]
    Storage(String),

    #[error("crypto error: {0}")]
    Crypto(String),
}
