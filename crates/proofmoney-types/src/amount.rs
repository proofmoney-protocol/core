use crate::ProofMoneyError;
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

    pub fn parse_prm_decimal(input: &str) -> Result<Self, ProofMoneyError> {
        let trimmed = input.trim();

        if trimmed.is_empty() || trimmed.starts_with('-') {
            return Err(ProofMoneyError::InvalidAmount);
        }

        let mut parts = trimmed.split('.');
        let whole = parts.next().ok_or(ProofMoneyError::InvalidAmount)?;
        let fractional = parts.next().unwrap_or("");

        if parts.next().is_some() {
            return Err(ProofMoneyError::InvalidAmount);
        }

        if whole.is_empty() || !whole.chars().all(|c| c.is_ascii_digit()) {
            return Err(ProofMoneyError::InvalidAmount);
        }

        if fractional.len() > 8 || !fractional.chars().all(|c| c.is_ascii_digit()) {
            return Err(ProofMoneyError::InvalidAmount);
        }

        let whole_value: u64 = whole
            .parse()
            .map_err(|_| ProofMoneyError::InvalidAmount)?;

        let mut fractional_string = fractional.to_string();
        while fractional_string.len() < 8 {
            fractional_string.push('0');
        }

        let fractional_value: u64 = if fractional_string.is_empty() {
            0
        } else {
            fractional_string
                .parse()
                .map_err(|_| ProofMoneyError::InvalidAmount)?
        };

        let whole_proof = whole_value
            .checked_mul(PROOF_PER_PRM)
            .ok_or(ProofMoneyError::InvalidAmount)?;

        let total = whole_proof
            .checked_add(fractional_value)
            .ok_or(ProofMoneyError::InvalidAmount)?;

        if total > MAX_SUPPLY_PROOF {
            return Err(ProofMoneyError::InvalidAmount);
        }

        Ok(Amount(total))
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

    #[test]
    fn parse_decimal_amount() {
        assert_eq!(Amount::parse_prm_decimal("1.25").unwrap().0, 125_000_000);
        assert_eq!(
            Amount::parse_prm_decimal("0.00000001").unwrap().0,
            1
        );
        assert!(Amount::parse_prm_decimal("1.123456789").is_err());
        assert!(Amount::parse_prm_decimal("-1").is_err());
        assert!(Amount::parse_prm_decimal("abc").is_err());
    }
}
