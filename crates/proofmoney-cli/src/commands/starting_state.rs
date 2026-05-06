use anyhow::Result;
use proofmoney_ledger::{generate_starting_state, verify_starting_state};

pub fn run(json: bool) -> Result<()> {
    let state = generate_starting_state("v1")?;
    let proof = verify_starting_state(&state)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&proof)?);
    } else {
        println!("Starting State Verification\n");
        println!("Initial Supply: {}", state.initial_supply.to_prm_string());
        println!("Founder Allocation: {}", state.founder_allocation.to_prm_string());
        println!("Private Sale Allocation: {}", state.private_sale_allocation.to_prm_string());
        println!(
            "Hidden Address Allocation: {}",
            state.hidden_address_allocation.to_prm_string()
        );
        println!(
            "Public Proof Fund Preload: {}",
            state.public_proof_fund_preload.to_prm_string()
        );
        println!("Rule Version: {}", state.rule_version);
        println!("Status: {:?}", proof.status);
    }

    Ok(())
}
