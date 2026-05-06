use proofmoney_types::{Amount, RuleSet};

pub fn get_epoch(interval: u64, rules: &RuleSet) -> u64 {
    if interval == 0 {
        0
    } else {
        (interval - 1) / rules.reduction_window
    }
}

pub fn get_base_release(interval: u64, rules: &RuleSet) -> Amount {
    let epoch = get_epoch(interval, rules);

    if epoch >= 64 {
        return Amount::zero();
    }

    Amount(rules.initial_release_amount.0 >> epoch)
}

pub fn get_protection_factor_bps(interval: u64, rules: &RuleSet) -> u64 {
    if interval <= rules.protection_window_1_end {
        rules.protection_factor_1_bps
    } else if interval <= rules.protection_window_2_end {
        rules.protection_factor_2_bps
    } else {
        rules.protection_factor_full_bps
    }
}

pub fn calculate_actual_release(interval: u64, rules: &RuleSet) -> Amount {
    let base = get_base_release(interval, rules);
    let factor_bps = get_protection_factor_bps(interval, rules);

    Amount(base.0.saturating_mul(factor_bps) / 10_000)
}

pub fn calculate_public_fund_allocation(total: Amount, rules: &RuleSet) -> Amount {
    Amount(total.0.saturating_mul(rules.public_fund_percent) / 100)
}

pub fn calculate_contributor_reward(total: Amount, rules: &RuleSet) -> Amount {
    let public_fund = calculate_public_fund_allocation(total, rules);
    Amount(total.0.saturating_sub(public_fund.0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use proofmoney_types::{PROOF_PER_PRM, RuleSet};

    #[test]
    fn interval_1_uses_20_percent() {
        let rules = RuleSet::default_v1();
        assert_eq!(get_protection_factor_bps(1, &rules), 2_000);
        assert_eq!(calculate_actual_release(1, &rules).0, 5 * PROOF_PER_PRM);
    }

    #[test]
    fn interval_10081_uses_50_percent() {
        let rules = RuleSet::default_v1();
        assert_eq!(get_protection_factor_bps(10_081, &rules), 5_000);
    }

    #[test]
    fn interval_20161_uses_100_percent() {
        let rules = RuleSet::default_v1();
        assert_eq!(get_protection_factor_bps(20_161, &rules), 10_000);
    }

    #[test]
    fn public_fund_is_3_percent() {
        let rules = RuleSet::default_v1();
        let total = calculate_actual_release(1, &rules);
        assert_eq!(calculate_public_fund_allocation(total, &rules).to_prm_string(), "0.15000000");
        assert_eq!(calculate_contributor_reward(total, &rules).to_prm_string(), "4.85000000");
    }
}
