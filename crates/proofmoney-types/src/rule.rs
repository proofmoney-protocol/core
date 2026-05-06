use serde::{Deserialize, Serialize};

use crate::{Amount, MAX_SUPPLY_PROOF, PROOF_PER_PRM};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuleSet {
    pub version: String,
    pub max_supply_boundary: Amount,
    pub initial_supply: Amount,
    pub public_fund_percent: u64,
    pub contributor_percent: u64,
    pub initial_release_amount: Amount,
    pub reduction_window: u64,
    pub protection_window_1_end: u64,
    pub protection_window_2_end: u64,
    pub protection_factor_1_bps: u64,
    pub protection_factor_2_bps: u64,
    pub protection_factor_full_bps: u64,
}

impl RuleSet {
    pub fn default_v1() -> Self {
        Self {
            version: "v1".to_string(),
            max_supply_boundary: Amount(MAX_SUPPLY_PROOF),
            initial_supply: Amount::zero(),
            public_fund_percent: 3,
            contributor_percent: 97,
            initial_release_amount: Amount(25 * PROOF_PER_PRM),
            reduction_window: 2_000_000,
            protection_window_1_end: 10_080,
            protection_window_2_end: 20_160,
            protection_factor_1_bps: 2_000,
            protection_factor_2_bps: 5_000,
            protection_factor_full_bps: 10_000,
        }
    }
}
