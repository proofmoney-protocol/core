# ProofMoney Core

ProofMoney is an open protocol initiative for **verifiable money integrity**.

> If money cannot be verified, it is only a promise.

This repository contains the Rust local MVP baseline and public testnet design materials for the ProofMoney Integrity Stack.

## Current Status

Current development target:

```text
v1.1.0-testnet-design
```

ProofMoney Core has reached its first stable local MVP baseline and is now entering public testnet architecture design.

This is still design work.

It is not a public testnet launch.  
It is not a mainnet launch.  
It does not create PRM with monetary value.  
It does not represent a token sale, investment opportunity, yield product, airdrop claim, exchange listing, hosted public API, completed audit, or future allocation right.

## Baseline Status

```text
Local MVP baseline: v1.0.0-local-mvp-freeze
Current workstream: v1.1.0-testnet-design
External audit status: Not audited
Network status: Local MVP only
Public testnet status: Design phase only
Mainnet status: Not launched
Wallet status: Experimental local MVP wallet only
Token sale status: Not offered
```

## Local MVP Includes

- Starting State Proof
- Proof of Issuance
- Proof of Supply
- Proof of Ownership
- Proof of Flow
- Proof of Rule
- local ledger persistence
- local ledger reset
- local state validation
- local tamper detection
- release event append flow
- computed supply verification
- local wallet persistence
- transfer event creation
- local balance tracking
- local proof JSON export
- proof snapshot generation with freshness metadata
- release event proof listing
- transfer event proof listing
- static local Proof Explorer prototype
- developer quickstart
- local demo scripts
- isolated CLI integration tests
- protocol specification documents
- local MVP baseline documents
- public testnet design documents

## Not Included

The current repository does not include:

- live public testnet;
- mainnet;
- public consensus;
- exchange integration;
- PRM sale;
- private allocation;
- airdrop claim;
- yield product;
- production wallet security;
- hosted public API;
- public production explorer;
- completed external audit.

## Build

```bash
cargo build --workspace --all-targets
```

## Test

```bash
cargo test --workspace --all-targets
```

## Local Demo

```bash
bash scripts/demo-local.sh
bash scripts/demo-transfer-local.sh
```

## Key Documentation

- [Local MVP Baseline Manifest](docs/local-mvp-baseline-v1.0.0.md)
- [Core Documentation Index](docs/core-documentation-index-v1.0.0.md)
- [Protocol Spec Index](docs/specs/protocol-spec-index-v0.1.md)
- [Testnet Design Index](docs/testnet/testnet-design-index-v1.1.0.md)
- [Testnet Goals and Scope](docs/testnet/testnet-goals-and-scope-v1.1.0.md)
- [Testnet Node Architecture](docs/testnet/testnet-node-architecture-v1.1.0.md)
- [Testnet Network Message Model](docs/testnet/testnet-network-message-model-v1.1.0.md)
- [Testnet Ledger Sync Model](docs/testnet/testnet-ledger-sync-model-v1.1.0.md)
- [Testnet Faucet Boundary](docs/testnet/testnet-faucet-boundary-v1.1.0.md)
- [Testnet Explorer Requirements](docs/testnet/testnet-explorer-requirements-v1.1.0.md)
- [Testnet Wallet Safety Policy](docs/testnet/testnet-wallet-safety-policy-v1.1.0.md)
- [Security Review Scope](docs/security-review-scope.md)
- [Contributing Guide](CONTRIBUTING.md)

## Risk Notice

ProofMoney is experimental. PRM may never have market value. Test units, if any, have no monetary value. Do not use experimental software with valuable assets.

## Author

Initial design authored by Vingen Motoki.
