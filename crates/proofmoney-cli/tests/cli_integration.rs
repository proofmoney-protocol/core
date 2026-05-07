use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_proofmoney")
}

fn isolated_home(test_name: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be valid")
        .as_nanos();

    let dir = std::env::temp_dir().join(format!("proofmoney-{test_name}-{stamp}"));
    fs::create_dir_all(&dir).expect("temp home should be created");
    dir
}

fn run(home: &PathBuf, args: &[&str]) -> Output {
    Command::new(bin())
        .args(args)
        .env("HOME", home)
        .env("USERPROFILE", home)
        .output()
        .expect("CLI command should run")
}

fn assert_success(output: Output) {
    if !output.status.success() {
        panic!(
            "command failed\nstatus: {:?}\nstdout:\n{}\nstderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn cli_flow_uses_isolated_home_and_exports_data() {
    let home = isolated_home("flow");

    assert_success(run(&home, &["reset-ledger", "--yes"]));
    assert_success(run(&home, &["starting-state"]));
    assert_success(run(
        &home,
        &["simulate-release", "--interval", "1", "--append"],
    ));
    assert_success(run(&home, &["ledger-status"]));
    assert_success(run(&home, &["validate-local-state"]));
    assert_success(run(&home, &["detect-tampering"]));
    assert_success(run(&home, &["export-proof-site-data"]));
    assert_success(run(&home, &["prepare-explorer"]));

    assert!(home.join(".proofmoney/ledger.json").exists());
    assert!(home.join(".proofmoney/export/proof-snapshot.json").exists());
    assert!(home.join(".proofmoney/explorer/index.html").exists());
}

#[test]
fn reset_ledger_requires_explicit_yes() {
    let home = isolated_home("reset-requires-yes");
    let output = run(&home, &["reset-ledger"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--yes"));
}

#[test]
fn reset_ledger_does_not_remove_wallet() {
    let home = isolated_home("reset-keeps-wallet");

    assert_success(run(&home, &["create-wallet", "--force"]));
    let wallet_path = home.join(".proofmoney/wallets/default.json");
    assert!(wallet_path.exists());

    assert_success(run(&home, &["reset-ledger", "--yes"]));
    assert!(wallet_path.exists());
}
