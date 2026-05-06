# ProofMoney Core Local Verification Guide

## Goal

Verify that the Rust MVP scaffold builds, tests, and runs locally.

## Requirements

Install Rust:

```bash
rustc --version
cargo --version
```

## Build

```bash
cargo build --workspace --all-targets
```

## Test

```bash
cargo test --workspace --all-targets
```

## Format

```bash
cargo fmt --all
```

Check formatting:

```bash
cargo fmt --all -- --check
```

## CLI Smoke Tests

```bash
cargo run -p proofmoney-cli -- starting-state
cargo run -p proofmoney-cli -- starting-state --json
cargo run -p proofmoney-cli -- simulate-release --interval 1
cargo run -p proofmoney-cli -- simulate-release --interval 1 --json
cargo run -p proofmoney-cli -- verify-supply
cargo run -p proofmoney-cli -- verify-rule
cargo run -p proofmoney-cli -- integrity-status
cargo run -p proofmoney-cli -- create-wallet
```

## Expected Starting State

```text
Initial Supply: 0.00000000
Founder Allocation: 0.00000000
Private Sale Allocation: 0.00000000
Hidden Address Allocation: 0.00000000
Public Proof Fund Preload: 0.00000000
Status: Valid
```

## Expected Release Simulation for Interval 1

```text
Base Release: 25.00000000
Protection Factor BPS: 2000
Actual Release: 5.00000000
Proof Contributor Reward: 4.85000000
Public Proof Fund Allocation: 0.15000000
Status: Valid
```

## Current MVP Limitation

This is a local prototype only.

It is not a public network, token sale, airdrop claim, yield product, exchange integration, or production wallet.
