# ProofMoney Core

ProofMoney is an open protocol initiative for **verifiable money integrity**.

> If money cannot be verified, it is only a promise.

This repository contains the Rust local MVP prototype for the ProofMoney Integrity Stack.

## Current Status

Current development target:

```text
v0.3.0-ownership-and-flow
```

This repository is a local MVP prototype.

It is not a public network.  
It does not create PRM with monetary value.  
It does not represent a token sale, investment opportunity, yield product, or future allocation right.

## MVP Scope

The MVP focuses on:

- Starting State Proof
- Proof of Issuance
- Proof of Supply
- Proof of Ownership
- Proof of Flow
- Proof of Rule
- local ledger persistence
- release event append flow
- computed supply verification
- local wallet persistence
- address inspection
- message signing
- transfer event creation
- local balance tracking
- insufficient balance rejection
- Integrity Status CLI

## Not Included

The MVP does not include:

- public network;
- exchange integration;
- PRM sale;
- private allocation;
- airdrop claim;
- yield product;
- production wallet security.

## Build

```bash
cargo build --workspace --all-targets
```

## Test

```bash
cargo test --workspace --all-targets
```

## CLI

```bash
cargo run -p proofmoney-cli -- starting-state
cargo run -p proofmoney-cli -- starting-state --json
cargo run -p proofmoney-cli -- simulate-release --interval 1
cargo run -p proofmoney-cli -- simulate-release --interval 1 --append
cargo run -p proofmoney-cli -- ledger-status
cargo run -p proofmoney-cli -- verify-supply
cargo run -p proofmoney-cli -- verify-rule
cargo run -p proofmoney-cli -- integrity-status
cargo run -p proofmoney-cli -- create-wallet
cargo run -p proofmoney-cli -- new-address
cargo run -p proofmoney-cli -- sign-message --message "verify ownership"
```

## Local Data

The MVP stores local ledger state at:

```text
~/.proofmoney/ledger.json
```

The MVP stores the local test wallet at:

```text
~/.proofmoney/wallets/default.json
```

These local files are for MVP testing only.

## Example Local Flow

```bash
cargo run -p proofmoney-cli -- create-wallet
cargo run -p proofmoney-cli -- new-address
cargo run -p proofmoney-cli -- sign-message --message "verify ownership"
```

For transfer flow testing, first credit an address through a local release event:

```bash
cargo run -p proofmoney-cli -- simulate-release --interval 1 --recipient <address> --append
cargo run -p proofmoney-cli -- create-transfer --from <address> --to <address> --amount 1.25 --append
```

## Risk Notice

ProofMoney is experimental. PRM may never have market value. Test units, if any, have no monetary value. Do not use experimental software with valuable assets.

## Author

Initial design authored by Vingen Motoki.
