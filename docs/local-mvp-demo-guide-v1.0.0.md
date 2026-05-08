# ProofMoney Local MVP Demo Guide v1.0.0

## Purpose

This guide explains how to run the ProofMoney Core v1.0.0 local MVP baseline.

This demo is local only.

It does not create PRM with monetary value.

---

## Requirements

- Rust stable toolchain;
- Git;
- Bash-compatible shell;
- macOS or Linux environment recommended.

---

## Clone

```bash
git clone https://github.com/proofmoney-protocol/core.git
cd core
```

---

## Build

```bash
cargo build --workspace --all-targets
```

---

## Test

```bash
cargo test --workspace --all-targets
```

---

## Basic Local Demo

```bash
bash scripts/demo-local.sh
```

This demo runs:

- build;
- tests;
- starting state;
- release simulation;
- ledger status;
- supply verification;
- wallet creation;
- address inspection;
- proof snapshot export;
- local explorer preparation.

---

## Transfer Demo

```bash
bash scripts/demo-transfer-local.sh
```

This demo runs:

- local ledger reset;
- local wallet creation;
- sender address inspection;
- release event append to sender;
- local transfer event creation;
- transfer append;
- local state validation;
- tamper detection;
- transfer event listing;
- proof snapshot export;
- local explorer preparation.

---

## Manual Flow

```bash
cargo run -p proofmoney-cli -- reset-ledger --yes
cargo run -p proofmoney-cli -- starting-state
cargo run -p proofmoney-cli -- create-wallet --force
cargo run -p proofmoney-cli -- new-address
cargo run -p proofmoney-cli -- simulate-release --interval 1 --append
cargo run -p proofmoney-cli -- ledger-status
cargo run -p proofmoney-cli -- validate-local-state
cargo run -p proofmoney-cli -- detect-tampering
cargo run -p proofmoney-cli -- export-proof-snapshot --json
cargo run -p proofmoney-cli -- prepare-explorer
```

---

## Local Explorer

After running:

```bash
cargo run -p proofmoney-cli -- prepare-explorer
```

Open:

```text
~/.proofmoney/explorer/index.html
```

The explorer is local static output only.

It is not a public block explorer.

---

## Troubleshooting

## Ledger state looks wrong

Run:

```bash
cargo run -p proofmoney-cli -- validate-local-state
cargo run -p proofmoney-cli -- detect-tampering
```

## Need a clean local ledger

Run:

```bash
cargo run -p proofmoney-cli -- reset-ledger --yes
```

## Wallet missing

Run:

```bash
cargo run -p proofmoney-cli -- create-wallet --force
```

## Proof export may be stale

Regenerate:

```bash
cargo run -p proofmoney-cli -- export-proof-site-data
cargo run -p proofmoney-cli -- export-proof-snapshot --json
```

---

## Safety Notice

The local demo is for MVP testing only.

It does not create PRM with monetary value.

It does not connect to a public network.

It is not a token sale, airdrop, yield product, exchange integration, or production wallet.
