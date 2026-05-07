# ProofMoney Developer Quickstart

## Purpose

This guide helps developers run the ProofMoney Core local MVP from scratch.

This is for local development only.

ProofMoney Core is not a public network, token sale, yield product, exchange integration, airdrop claim, or production wallet.

---

## 1. Clone Repository

```bash
git clone https://github.com/proofmoney-protocol/core.git
cd core
```

---

## 2. Install Rust

Install Rust from the official Rust toolchain site:

```bash
rustc --version
cargo --version
```

Recommended toolchain:

```bash
rustup default stable
```

---

## 3. Build

```bash
cargo build --workspace --all-targets
```

---

## 4. Run Tests

```bash
cargo test --workspace --all-targets
```

---

## 5. Run CLI Smoke Tests

```bash
cargo run -p proofmoney-cli -- starting-state
cargo run -p proofmoney-cli -- starting-state --json
cargo run -p proofmoney-cli -- simulate-release --interval 1 --append
cargo run -p proofmoney-cli -- ledger-status
cargo run -p proofmoney-cli -- verify-supply
cargo run -p proofmoney-cli -- verify-rule
cargo run -p proofmoney-cli -- integrity-status
```

---

## 6. Wallet and Ownership Commands

```bash
cargo run -p proofmoney-cli -- create-wallet --force
cargo run -p proofmoney-cli -- new-address
cargo run -p proofmoney-cli -- sign-message --message "verify ownership"
```

The MVP wallet is experimental.

Do not use it with valuable assets.

---

## 7. Proof Export Commands

```bash
cargo run -p proofmoney-cli -- list-release-events
cargo run -p proofmoney-cli -- list-transfer-events
cargo run -p proofmoney-cli -- export-proof-snapshot --json
cargo run -p proofmoney-cli -- export-proof-snapshot --output proof-snapshot.json
cargo run -p proofmoney-cli -- export-proof-site-data
cargo run -p proofmoney-cli -- prepare-explorer
```

---

## 8. Local Demo Script

```bash
bash scripts/demo-local.sh
```

The demo script resets local MVP data under:

```text
~/.proofmoney
```

Then it runs build, tests, core CLI commands, wallet commands, proof export, and explorer preparation.

---

## 9. Local Paths

Ledger:

```text
~/.proofmoney/ledger.json
```

Wallet:

```text
~/.proofmoney/wallets/default.json
```

Proof export:

```text
~/.proofmoney/export/
```

Static local explorer:

```text
~/.proofmoney/explorer/
```

---

## 10. Troubleshooting

### Cargo command not found

Install Rust and restart your terminal.

### Wallet already exists

Use:

```bash
cargo run -p proofmoney-cli -- create-wallet --force
```

### Release append fails with height error

The local ledger may already contain an event at that height.

For a clean local demo, remove local MVP data:

```bash
rm -rf ~/.proofmoney
```

Then rerun the command.

### Explorer JSON does not load in browser

Some browsers restrict local file loading.

You can still inspect exported JSON files directly under:

```text
~/.proofmoney/export/
```

The explorer is a static local prototype only.

---

## Safety Boundary

ProofMoney Core is a local MVP prototype.

It does not create PRM with monetary value.

It is not a public network, token sale, airdrop, yield product, exchange integration, hosted public API, or production wallet.
