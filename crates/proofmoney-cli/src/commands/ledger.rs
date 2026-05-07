use anyhow::Result;
use proofmoney_storage::{ledger_path, load_or_init_ledger};

pub fn ledger_status(json: bool) -> Result<()> {
    let ledger = load_or_init_ledger("v1")?;
    let path = ledger_path()?;

    let output = serde_json::json!({
        "ledger_version": ledger.ledger_version,
        "current_height": ledger.current_height,
        "current_supply": ledger.current_supply.to_prm_string(),
        "public_proof_fund_balance": ledger.public_fund_balance.to_prm_string(),
        "rule_version": ledger.rule_version,
        "event_count": ledger.events.len(),
        "last_event_hash": ledger.last_event_hash,
        "local_storage_path": path.display().to_string()
    });

    if json {
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        println!("ProofMoney Local Ledger Status\n");
        println!("Ledger Version: {}", ledger.ledger_version);
        println!("Current Height: {}", ledger.current_height);
        println!("Current Supply: {}", ledger.current_supply.to_prm_string());
        println!(
            "Public Proof Fund Balance: {}",
            ledger.public_fund_balance.to_prm_string()
        );
        println!("Rule Version: {}", ledger.rule_version);
        println!("Event Count: {}", ledger.events.len());
        println!(
            "Last Event Hash: {}",
            ledger.last_event_hash.unwrap_or_else(|| "None".to_string())
        );
        println!("Local Storage Path: {}", path.display());
    }

    Ok(())
}
