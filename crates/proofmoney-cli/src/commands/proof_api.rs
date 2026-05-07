use anyhow::Result;
use chrono::Utc;
use proofmoney_ledger::{
    generate_starting_state, state_before_event, verify_event_hash, verify_starting_state,
};
use proofmoney_proof::{
    integrity_status as proof_integrity_status, verify_flow, verify_rule as proof_verify_rule,
    verify_supply as proof_verify_supply,
};
use proofmoney_release::validate_release_event;
use proofmoney_storage::{
    explorer_data_dir, explorer_dir, export_dir, ledger_path, load_or_init_ledger, save_json,
};
use proofmoney_types::{Event, EventType, LedgerState, ProofStatus, RuleSet};
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

const SAFETY_NOTICE: &str = "ProofMoney local MVP data only. Not a public network, token sale, yield product, airdrop, exchange integration, or production wallet.";

pub fn export_proof_snapshot(output: Option<String>, json_output: bool) -> Result<()> {
    let snapshot = proof_snapshot_value()?;

    if let Some(output_path) = output {
        save_json(Path::new(&output_path), &snapshot)?;
        println!("Proof snapshot exported to: {}", output_path);
        return Ok(());
    }

    if json_output {
        println!("{}", serde_json::to_string_pretty(&snapshot)?);
    } else {
        println!("ProofMoney Local Proof Snapshot\n");
        println!("Generated At: {}", snapshot["generated_at"]);
        println!("Ledger Path: {}", snapshot["ledger_status"]["data"]["local_storage_path"]);
        println!("Safety: {}", SAFETY_NOTICE);
    }

    Ok(())
}

pub fn export_proof_site_data() -> Result<()> {
    let dir = export_dir()?;
    export_all_to_dir(&dir)?;

    println!("Proof site data exported to: {}", dir.display());
    println!("Safety: {}", SAFETY_NOTICE);
    Ok(())
}

pub fn list_release_events(json_output: bool) -> Result<()> {
    let value = release_events_value()?;

    if json_output {
        println!("{}", serde_json::to_string_pretty(&value)?);
    } else {
        println!("Local Release Events\n");

        let events = value["data"]["events"].as_array().cloned().unwrap_or_default();

        if events.is_empty() {
            println!("No release events found.");
        }

        for event in events {
            println!("Event ID: {}", event["event_id"].as_str().unwrap_or(""));
            println!("Height: {}", event["height"]);
            println!("Recipient: {}", event["recipient"].as_str().unwrap_or(""));
            println!("Actual Release: {}", event["actual_release"].as_str().unwrap_or(""));
            println!(
                "Public Proof Fund: {}",
                event["public_proof_fund"].as_str().unwrap_or("")
            );
            println!("Hash Valid: {}", event["event_hash_valid"]);
            println!();
        }

        println!("Safety: {}", SAFETY_NOTICE);
    }

    Ok(())
}

pub fn list_transfer_events(json_output: bool) -> Result<()> {
    let value = transfer_events_value()?;

    if json_output {
        println!("{}", serde_json::to_string_pretty(&value)?);
    } else {
        println!("Local Transfer Events\n");

        let events = value["data"]["events"].as_array().cloned().unwrap_or_default();

        if events.is_empty() {
            println!("No transfer events found.");
        }

        for event in events {
            println!("Transaction ID: {}", event["transaction_id"].as_str().unwrap_or(""));
            println!("From: {}", event["from"].as_str().unwrap_or(""));
            println!("To: {}", event["to"].as_str().unwrap_or(""));
            println!("Amount: {}", event["amount"].as_str().unwrap_or(""));
            println!("Flow Status: {}", event["flow_status"].as_str().unwrap_or(""));
            println!("Hash Valid: {}", event["event_hash_valid"]);
            println!();
        }

        println!("Safety: {}", SAFETY_NOTICE);
    }

    Ok(())
}

pub fn prepare_explorer() -> Result<()> {
    let explorer = explorer_dir()?;
    let data = explorer_data_dir()?;

    fs::create_dir_all(&data)?;
    export_all_to_dir(&data)?;

    fs::write(explorer.join("index.html"), include_str!("../../../../explorer/index.html"))?;
    fs::write(explorer.join("styles.css"), include_str!("../../../../explorer/styles.css"))?;
    fs::write(explorer.join("README.md"), include_str!("../../../../explorer/README.md"))?;

    println!("Proof Explorer prepared at: {}", explorer.display());
    println!("Open locally: {}", explorer.join("index.html").display());
    println!("Explorer data: {}", data.display());
    println!("Safety: {}", SAFETY_NOTICE);

    Ok(())
}

fn export_all_to_dir(dir: &Path) -> Result<()> {
    fs::create_dir_all(dir)?;

    save_json(&dir.join("starting-state.json"), &starting_state_value()?)?;
    save_json(&dir.join("ledger-status.json"), &ledger_status_value()?)?;
    save_json(&dir.join("supply.json"), &supply_value()?)?;
    save_json(&dir.join("rule.json"), &rule_value()?)?;
    save_json(&dir.join("integrity-status.json"), &integrity_status_value()?)?;
    save_json(&dir.join("release-events.json"), &release_events_value()?)?;
    save_json(&dir.join("transfer-events.json"), &transfer_events_value()?)?;
    save_json(&dir.join("proof-snapshot.json"), &proof_snapshot_value()?)?;

    Ok(())
}

fn proof_snapshot_value() -> Result<Value> {
    Ok(json!({
        "project": "ProofMoney",
        "snapshot_version": "v0.4.0-proof-explorer-api",
        "generated_at": Utc::now().to_rfc3339(),
        "safety_notice": SAFETY_NOTICE,
        "starting_state": starting_state_value()?,
        "ledger_status": ledger_status_value()?,
        "supply": supply_value()?,
        "rule": rule_value()?,
        "integrity_status": integrity_status_value()?,
        "release_events": release_events_value()?,
        "transfer_events": transfer_events_value()?
    }))
}

fn starting_state_value() -> Result<Value> {
    let state = generate_starting_state("v1")?;
    let proof = verify_starting_state(&state)?;

    Ok(json!({
        "status": format!("{:?}", proof.status),
        "proof_type": proof.proof_type,
        "rule_version": proof.rule_version,
        "data": proof.data,
        "summary": proof.summary,
        "safety_notice": SAFETY_NOTICE
    }))
}

fn ledger_status_value() -> Result<Value> {
    let ledger = load_or_init_ledger("v1")?;
    let path = ledger_path()?;

    Ok(json!({
        "status": "Valid",
        "proof_type": "ledger_status",
        "rule_version": ledger.rule_version,
        "data": {
            "ledger_version": ledger.ledger_version,
            "current_height": ledger.current_height,
            "current_supply": ledger.current_supply.to_prm_string(),
            "public_proof_fund_balance": ledger.public_fund_balance.to_prm_string(),
            "event_count": ledger.events.len(),
            "last_event_hash": ledger.last_event_hash,
            "local_storage_path": path.display().to_string()
        },
        "summary": "Local ledger status exported.",
        "safety_notice": SAFETY_NOTICE
    }))
}

fn supply_value() -> Result<Value> {
    let rules = RuleSet::default_v1();
    let ledger = load_or_init_ledger("v1")?;
    let proof = proof_verify_supply(&ledger, &rules)?;

    Ok(json!({
        "status": format!("{:?}", proof.status),
        "proof_type": proof.proof_type,
        "rule_version": proof.rule_version,
        "data": proof.data,
        "summary": proof.summary,
        "safety_notice": SAFETY_NOTICE
    }))
}

fn rule_value() -> Result<Value> {
    let rules = RuleSet::default_v1();
    let proof = proof_verify_rule(&rules, &RuleSet::default_v1())?;

    Ok(json!({
        "status": format!("{:?}", proof.status),
        "proof_type": proof.proof_type,
        "rule_version": proof.rule_version,
        "data": proof.data,
        "summary": proof.summary,
        "safety_notice": SAFETY_NOTICE
    }))
}

fn integrity_status_value() -> Result<Value> {
    let rules = RuleSet::default_v1();
    let ledger = load_or_init_ledger("v1")?;
    let proof = proof_integrity_status(&ledger, &rules)?;

    Ok(json!({
        "status": format!("{:?}", proof.status),
        "proof_type": proof.proof_type,
        "rule_version": proof.rule_version,
        "data": proof.data,
        "summary": proof.summary,
        "safety_notice": SAFETY_NOTICE
    }))
}

fn release_events_value() -> Result<Value> {
    let ledger = load_or_init_ledger("v1")?;
    let rules = RuleSet::default_v1();
    let mut events = Vec::new();

    for event in ledger
        .events
        .iter()
        .filter(|event| event.event_type == EventType::Release)
    {
        events.push(release_event_value(event, &ledger, &rules)?);
    }

    Ok(json!({
        "status": "Valid",
        "proof_type": "release_events",
        "rule_version": rules.version,
        "data": {
            "count": events.len(),
            "events": events
        },
        "summary": "Local release event proof listing exported.",
        "safety_notice": SAFETY_NOTICE
    }))
}

fn release_event_value(event: &Event, ledger: &LedgerState, rules: &RuleSet) -> Result<Value> {
    let proof = validate_release_event(event, ledger, rules)?;
    let event_hash_valid = verify_event_hash(event)?;

    Ok(json!({
        "event_id": event.id,
        "height": event.height,
        "timestamp": event.timestamp,
        "recipient": event.payload.get("recipient").and_then(|v| v.as_str()).unwrap_or(""),
        "actual_release": event.payload.get("actual_release").and_then(|v| v.as_str()).unwrap_or(""),
        "public_proof_fund": event.payload.get("public_proof_fund").and_then(|v| v.as_str()).unwrap_or(""),
        "proof_contributor_reward": event.payload.get("proof_contributor_reward").and_then(|v| v.as_str()).unwrap_or(""),
        "event_hash": event.hash,
        "event_hash_valid": event_hash_valid,
        "proof_status": format!("{:?}", proof.status)
    }))
}

fn transfer_events_value() -> Result<Value> {
    let ledger = load_or_init_ledger("v1")?;
    let mut events = Vec::new();

    for event in ledger
        .events
        .iter()
        .filter(|event| event.event_type == EventType::Transfer)
    {
        events.push(transfer_event_value(event, &ledger)?);
    }

    Ok(json!({
        "status": "Valid",
        "proof_type": "transfer_events",
        "rule_version": ledger.rule_version,
        "data": {
            "count": events.len(),
            "events": events
        },
        "summary": "Local transfer event proof listing exported.",
        "safety_notice": SAFETY_NOTICE
    }))
}

fn transfer_event_value(event: &Event, ledger: &LedgerState) -> Result<Value> {
    let event_hash_valid = verify_event_hash(event)?;
    let flow_status = match state_before_event(ledger, &event.id) {
        Ok((state_before, transfer_event)) => {
            let proof = verify_flow(&transfer_event, &state_before)?;
            format!("{:?}", proof.status)
        }
        Err(_) => format!("{:?}", ProofStatus::Invalid),
    };

    Ok(json!({
        "transaction_id": event.id,
        "height": event.height,
        "timestamp": event.timestamp,
        "from": event.payload.get("from").and_then(|v| v.as_str()).unwrap_or(""),
        "to": event.payload.get("to").and_then(|v| v.as_str()).unwrap_or(""),
        "amount": event.payload.get("amount").and_then(|v| v.as_str()).unwrap_or(""),
        "fee": event.payload.get("fee").and_then(|v| v.as_str()).unwrap_or(""),
        "event_hash": event.hash,
        "event_hash_valid": event_hash_valid,
        "flow_status": flow_status,
        "mvp_scope": "local_transfer_event"
    }))
}
