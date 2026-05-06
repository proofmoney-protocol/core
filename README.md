# ProofMoney Core

ProofMoney is an open protocol initiative for **verifiable money integrity**.

> If money cannot be verified, it is only a promise.

This repository contains the Rust local MVP prototype for the ProofMoney Integrity Stack.

## Current Status

This repository is in local MVP prototype stage.

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
cargo build
```

## Test

```bash
cargo test
```

## CLI

```bash
cargo run -p proofmoney-cli -- starting-state
cargo run -p proofmoney-cli -- starting-state --json
cargo run -p proofmoney-cli -- simulate-release --interval 1
cargo run -p proofmoney-cli -- verify-supply
cargo run -p proofmoney-cli -- verify-rule
cargo run -p proofmoney-cli -- integrity-status
cargo run -p proofmoney-cli -- create-wallet
```

## Risk Notice

ProofMoney is experimental. PRM may never have market value. Test units, if any, have no monetary value. Do not use experimental software with valuable assets.

## Author

Initial design authored by Vingen Motoki.
