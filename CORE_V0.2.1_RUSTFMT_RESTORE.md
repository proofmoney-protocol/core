# ProofMoney Core v0.2.1 Rustfmt Restore

## Purpose

This patch restores strict Rust formatting enforcement in GitHub Actions.

It addresses Issue 7:

```text
Restore strict rustfmt check in CI
```

## Changes

This patch adds the following CI step back into `.github/workflows/rust-ci.yml`:

```bash
cargo fmt --all -- --check
```

It also updates the CLI verify command import formatting so that it follows standard `rustfmt` output style.

## Expected CI Flow

After upload, GitHub Actions should run:

```bash
cargo fmt --all -- --check
cargo build --workspace --all-targets
cargo test --workspace --all-targets
cargo run -p proofmoney-cli -- starting-state
cargo run -p proofmoney-cli -- starting-state --json
cargo run -p proofmoney-cli -- simulate-release --interval 1
cargo run -p proofmoney-cli -- simulate-release --interval 1 --append
cargo run -p proofmoney-cli -- ledger-status
cargo run -p proofmoney-cli -- verify-supply
cargo run -p proofmoney-cli -- verify-rule
cargo run -p proofmoney-cli -- integrity-status
```

## Safety Boundary

This patch only affects code formatting and CI quality control.

It does not change:

- protocol rules;
- PRM issuance;
- supply logic;
- wallet behavior;
- release curve parameters;
- any user-facing monetary claim.
