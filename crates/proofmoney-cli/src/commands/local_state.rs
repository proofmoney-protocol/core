use anyhow::{bail, Result};
use proofmoney_ledger::{
    compute_balances_from_events, compute_public_fund_from_events, compute_supply_from_events,
    verify_event_hash,
};
use proofmoney_storage::{ledger_path, load_or_init_ledger};
use proofmoney_types::LedgerState;
use serde_json::{json, Value};
use std::fs;

const SAFETY_NOTICE: &str = "Local MVP state only. No public network exists and no PRM with monetary value is created.";

pub fn reset_ledger(yes: bool, json_output: bool) -> Result<()> {
    if !yes {
        bail!("reset-ledger requires --yes because it overwrites local MVP ledger state only");
    }

    let path = ledger_path()?;

    if path.exists() {
        fs::remove_file(&path)?;
    }

    let state = load_or_init_ledger("v1")?;

    let output = json!({
        "status": "reset",
        "ledger_path": path.display().to_string(),
        "ledger_height": state.current_height,
        "event_count": state.events.len(),
        "wallet_reset": false,
        "safety_notice": SAFETY_NOTICE
    });

    if json_output {
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        println!("Local Ledger Reset\n");
        println!("Status: reset");
        println!("Ledger Path: {}", path.display());
        println!("Ledger Height: {}", state.current_height);
        println!("Event Count: {}", state.events.len());
        println!("Wallet Reset: false");
        println!("Safety: {}", SAFETY_NOTICE);
    }

    Ok(())
}

pub fn validate_local_state(json_output: bool) -> Result<()> {
    let report = local_state_report("validate_local_state")?;
    let valid = report["valid"].as_bool().unwrap_or(false);

    if json_output {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        print_human_report("Local State Validation", &report);
    }

    if !valid {
        bail!("local MVP state validation failed");
    }

    Ok(())
}

pub fn detect_tampering(json_output: bool) -> Result<()> {
    let report = local_state_report("detect_tampering")?;
    let valid = report["valid"].as_bool().unwrap_or(false);

    if json_output {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        print_human_report("Local Tamper Detection", &report);
    }

    if !valid {
        bail!("local ledger tampering or invalid local state detected");
    }

    Ok(())
}

fn print_human_report(title: &str, report: &Value) {
    println!("{title}\n");
    println!("Valid: {}", report["valid"].as_bool().unwrap_or(false));
    println!("Ledger Path: {}", report["ledger_path"].as_str().unwrap_or(""));
    println!("Current Height: {}", report["ledger_height"]);
    println!("Event Count: {}", report["event_count"]);
    println!("Last Event Hash: {}", report["last_event_hash"].as_str().unwrap_or("none"));

    let issues = report["issues"].as_array().cloned().unwrap_or_default();

    if issues.is_empty() {
        println!("Issues: none");
    } else {
        println!("Issues:");
        for issue in issues {
            println!("- {}", issue.as_str().unwrap_or("unknown issue"));
        }
    }

    println!("Safety: {}", SAFETY_NOTICE);
}

fn local_state_report(command: &str) -> Result<Value> {
    let path = ledger_path()?;
    let ledger = load_or_init_ledger("v1")?;
    let issues = validate_ledger(&ledger);
    let valid = issues.is_empty();

    Ok(json!({
        "command": command,
        "valid": valid,
        "status": if valid { "valid" } else { "invalid" },
        "ledger_path": path.display().to_string(),
        "ledger_height": ledger.current_height,
        "event_count": ledger.events.len(),
        "last_event_hash": ledger.last_event_hash,
        "issues": issues,
        "checks": {
            "event_hashes": "checked",
            "rule_versions": "checked",
            "height_order": "checked",
            "computed_supply": "checked",
            "public_proof_fund": "checked",
            "balances": "checked",
            "last_event_hash": "checked"
        },
        "safety_notice": SAFETY_NOTICE
    }))
}

fn validate_ledger(ledger: &LedgerState) -> Vec<String> {
    let mut issues = Vec::new();

    let mut previous_height = 0;

    for event in &ledger.events {
        match verify_event_hash(event) {
            Ok(true) => {}
            Ok(false) => issues.push(format!("event {} has invalid hash", event.id)),
            Err(err) => issues.push(format!("event {} hash check failed: {err}", event.id)),
        }

        if event.rule_version != ledger.rule_version {
            issues.push(format!(
                "event {} rule version {} does not match ledger rule version {}",
                event.id, event.rule_version, ledger.rule_version
            ));
        }

        if event.height <= previous_height {
            issues.push(format!(
                "event {} has non-increasing height {}",
                event.id, event.height
            ));
        }

        previous_height = event.height;
    }

    if let Some(last) = ledger.events.last() {
        if ledger.current_height != last.height {
            issues.push(format!(
                "ledger current height {} does not match last event height {}",
                ledger.current_height, last.height
            ));
        }

        if ledger.last_event_hash.as_deref() != Some(last.hash.as_str()) {
            issues.push("ledger last_event_hash does not match last event hash".to_string());
        }
    } else {
        if ledger.current_height != 0 {
            issues.push("empty ledger should have current height 0".to_string());
        }

        if ledger.last_event_hash.is_some() {
            issues.push("empty ledger should not have last_event_hash".to_string());
        }
    }

    match compute_supply_from_events(ledger) {
        Ok(computed) if computed != ledger.current_supply => issues.push(format!(
            "computed supply {} does not match stored supply {}",
            computed.to_prm_string(),
            ledger.current_supply.to_prm_string()
        )),
        Ok(_) => {}
        Err(err) => issues.push(format!("computed supply failed: {err}")),
    }

    match compute_public_fund_from_events(ledger) {
        Ok(computed) if computed != ledger.public_fund_balance => issues.push(format!(
            "computed Public Proof Fund {} does not match stored balance {}",
            computed.to_prm_string(),
            ledger.public_fund_balance.to_prm_string()
        )),
        Ok(_) => {}
        Err(err) => issues.push(format!("computed Public Proof Fund failed: {err}")),
    }

    if let Err(err) = compute_balances_from_events(ledger) {
        issues.push(format!("balance recomputation failed: {err}"));
    }

    issues
}
