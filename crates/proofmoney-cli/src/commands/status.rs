use anyhow::Result;
use proofmoney_proof::integrity_status as get_integrity_status;
use proofmoney_types::{LedgerState, RuleSet};

pub fn integrity_status(json: bool) -> Result<()> {
    let rules = RuleSet::default_v1();
    let state = LedgerState::new("v1");

    let proof = get_integrity_status(&state, &rules)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&proof)?);
    } else {
        println!("ProofMoney Integrity Status\n");
        println!("{}", proof.summary);
        println!("Status: {:?}", proof.status);
    }

    Ok(())
}
