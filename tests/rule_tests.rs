use proofmoney_proof::verify_rule;
use proofmoney_types::{ProofStatus, RuleSet};

#[test]
fn default_rule_is_valid() {
    let rules = RuleSet::default_v1();
    let proof = verify_rule(&rules, &RuleSet::default_v1()).unwrap();
    assert_eq!(proof.status, ProofStatus::Valid);
}

#[test]
fn changed_max_supply_is_invalid() {
    let mut rules = RuleSet::default_v1();
    rules.max_supply_boundary = proofmoney_types::Amount::from_prm(200_000_000);
    let proof = verify_rule(&rules, &RuleSet::default_v1()).unwrap();
    assert_eq!(proof.status, ProofStatus::Invalid);
}
