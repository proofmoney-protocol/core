use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

static TEST_LOCK: Mutex<()> = Mutex::new(());

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
        .env("XDG_DATA_HOME", home.join(".local/share"))
        .env("XDG_CONFIG_HOME", home.join(".config"))
        .env("XDG_CACHE_HOME", home.join(".cache"))
        .output()
        .expect("CLI command should run")
}

fn assert_success(args: &[&str], output: Output) {
    if !output.status.success() {
        panic!(
            "command failed\nargs: {:?}\nstatus: {:?}\nstdout:\n{}\nstderr:\n{}",
            args,
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn cli_flow_uses_isolated_home_and_exports_data() {
    let _guard = TEST_LOCK.lock().expect("test lock should be available");
    let home = isolated_home("flow");

    let commands: Vec<Vec<&str>> = vec![
        vec!["reset-ledger", "--yes"],
        vec!["starting-state"],
        vec!["simulate-release", "--interval", "1", "--append"],
        vec!["ledger-status"],
        vec!["validate-local-state"],
        vec!["detect-tampering"],
        vec!["export-proof-site-data"],
        vec!["prepare-explorer"],
    ];

    for args in commands {
        let output = run(&home, &args);
        assert_success(&args, output);
    }

    assert!(home.join(".proofmoney/ledger.json").exists());
    assert!(home.join(".proofmoney/export/proof-snapshot.json").exists());
    assert!(home.join(".proofmoney/explorer/index.html").exists());
}

#[test]
fn reset_ledger_requires_explicit_yes() {
    let _guard = TEST_LOCK.lock().expect("test lock should be available");
    let home = isolated_home("reset-requires-yes");
    let output = run(&home, &["reset-ledger"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--yes"));
}

#[test]
fn reset_ledger_does_not_remove_wallet() {
    let _guard = TEST_LOCK.lock().expect("test lock should be available");
    let home = isolated_home("reset-keeps-wallet");

    assert_success(
        &["create-wallet", "--force"],
        run(&home, &["create-wallet", "--force"]),
    );

    let wallet_path = home.join(".proofmoney/wallets/default.json");
    assert!(wallet_path.exists());

    assert_success(
        &["reset-ledger", "--yes"],
        run(&home, &["reset-ledger", "--yes"]),
    );

    assert!(wallet_path.exists());
}
