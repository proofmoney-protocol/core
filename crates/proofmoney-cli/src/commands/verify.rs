use anyhow::Result;
use proofmoney_proof::{verify_rule, verify_supply};
use proofmoney_types::{LedgerState, RuleSet};

pub fn verify_supply(json: bool) -> Result<()> {
    let rules = RuleSet::default_v1();
    let state = LedgerState::new("v1");

    let proof = verify_supply(&state, &rules)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&proof)?);
    } else {
        println!("Supply Verification\n");
        println!("{}", proof.summary);
        println!("Status: {:?}", proof.status);
    }

    Ok(())
}

pub fn verify_rule(json: bool) -> Result<()> {
    let rules = RuleSet::default_v1();
    let proof = verify_rule(&rules, &RuleSet::default_v1())?;

    if json {
        println!("{}", serde_json::to_string_pretty(&proof)?);
    } else {
        println!("Rule Verification\n");
        println!("{}", proof.summary);
        println!("Status: {:?}", proof.status);
    }

    Ok(())
}
