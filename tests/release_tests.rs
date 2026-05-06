use proofmoney_release::{
    calculate_actual_release, calculate_public_fund_allocation, get_protection_factor_bps,
};
use proofmoney_types::{RuleSet, PROOF_PER_PRM};

#[test]
fn protection_windows_are_correct() {
    let rules = RuleSet::default_v1();

    assert_eq!(get_protection_factor_bps(1, &rules), 2_000);
    assert_eq!(get_protection_factor_bps(10_080, &rules), 2_000);
    assert_eq!(get_protection_factor_bps(10_081, &rules), 5_000);
    assert_eq!(get_protection_factor_bps(20_160, &rules), 5_000);
    assert_eq!(get_protection_factor_bps(20_161, &rules), 10_000);
}

#[test]
fn interval_one_release_is_five_prm() {
    let rules = RuleSet::default_v1();
    assert_eq!(calculate_actual_release(1, &rules).0, 5 * PROOF_PER_PRM);
}

#[test]
fn public_fund_is_three_percent() {
    let rules = RuleSet::default_v1();
    let release = calculate_actual_release(1, &rules);
    assert_eq!(calculate_public_fund_allocation(release, &rules).to_prm_string(), "0.15000000");
}
