use proofmoney_ledger::{generate_starting_state, verify_starting_state, StartingState};
use proofmoney_types::{Amount, ProofStatus};

#[test]
fn starting_state_is_valid() {
    let state = generate_starting_state("v1").unwrap();
    let proof = verify_starting_state(&state).unwrap();
    assert_eq!(proof.status, ProofStatus::Valid);
}

#[test]
fn non_zero_founder_allocation_fails() {
    let state = StartingState::with_founder_allocation(Amount::from_prm(1)).unwrap();
    let proof = verify_starting_state(&state).unwrap();
    assert_eq!(proof.status, ProofStatus::Invalid);
}
