use serde::{Deserialize, Serialize};
use std::fmt;

pub type ProofAmount = u64;

pub const PROOF_PER_PRM: ProofAmount = 100_000_000;
pub const MAX_SUPPLY_PRM: ProofAmount = 100_000_000;
pub const MAX_SUPPLY_PROOF: ProofAmount = MAX_SUPPLY_PRM * PROOF_PER_PRM;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Amount(pub ProofAmount);

impl Amount {
    pub fn zero() -> Self {
        Self(0)
    }

    pub fn from_proof(value: ProofAmount) -> Self {
        Self(value)
    }

    pub fn from_prm(value: u64) -> Self {
        Self(value.saturating_mul(PROOF_PER_PRM))
    }

    pub fn checked_add(self, other: Amount) -> Option<Amount> {
        self.0.checked_add(other.0).map(Amount)
    }

    pub fn checked_sub(self, other: Amount) -> Option<Amount> {
        self.0.checked_sub(other.0).map(Amount)
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn to_prm_string(&self) -> String {
        let whole = self.0 / PROOF_PER_PRM;
        let fractional = self.0 % PROOF_PER_PRM;
        format!("{}.{:08}", whole, fractional)
    }
}

impl Default for Amount {
    fn default() -> Self {
        Self::zero()
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_prm_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_prm_equals_100m_proof() {
        assert_eq!(Amount::from_prm(1).0, 100_000_000);
    }

    #[test]
    fn zero_formats_correctly() {
        assert_eq!(Amount::zero().to_prm_string(), "0.00000000");
    }

    #[test]
    fn fractional_format_works() {
        assert_eq!(
            Amount::from_proof(123_456_789).to_prm_string(),
            "1.23456789"
        );
    }
}
