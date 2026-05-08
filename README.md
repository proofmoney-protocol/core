# ProofMoney Core

ProofMoney is an open protocol initiative for **verifiable money integrity**.

> If money cannot be verified, it is only a promise.

This repository contains the Rust local MVP baseline for the ProofMoney Integrity Stack.

## Current Status

Current baseline:

```text
v1.0.0-local-mvp-freeze
```

ProofMoney Core has reached its first stable local MVP baseline.

This repository is still local MVP software.

It is not a public network.  
It does not create PRM with monetary value.  
It does not represent a token sale, investment opportunity, yield product, airdrop claim, exchange listing, hosted public API, completed audit, or future allocation right.

## Baseline Status

```text
Baseline type: Stable local MVP baseline
External audit status: Not audited
Network status: Local MVP only
Wallet status: Experimental local MVP wallet only
Public network status: Not launched
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
- sample proof fixtures
- architecture overview
- security review scope
- contributor guide
- MVP scope freeze
- internal review and hardening notes
- protocol specification documents
- local MVP baseline manifest
- local MVP command reference
- local MVP demo guide
- local MVP risk and limitation summary
- local MVP release history
- local MVP public summary

## Not Included

The local MVP does not include:

- public network;
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

## Core CLI Examples

```bash
cargo run -p proofmoney-cli -- reset-ledger --yes
cargo run -p proofmoney-cli -- starting-state
cargo run -p proofmoney-cli -- simulate-release --interval 1 --append
cargo run -p proofmoney-cli -- ledger-status
cargo run -p proofmoney-cli -- validate-local-state
cargo run -p proofmoney-cli -- detect-tampering
cargo run -p proofmoney-cli -- verify-supply
cargo run -p proofmoney-cli -- create-wallet --force
cargo run -p proofmoney-cli -- new-address
cargo run -p proofmoney-cli -- export-proof-snapshot --json
cargo run -p proofmoney-cli -- prepare-explorer
```

## Key Documentation

- [Local MVP Baseline Manifest](docs/local-mvp-baseline-v1.0.0.md)
- [Core Documentation Index](docs/core-documentation-index-v1.0.0.md)
- [Local MVP Command Reference](docs/local-mvp-command-reference-v1.0.0.md)
- [Local MVP Demo Guide](docs/local-mvp-demo-guide-v1.0.0.md)
- [Local MVP Risk and Limitations](docs/local-mvp-risk-and-limitations-v1.0.0.md)
- [Local MVP Release History](docs/local-mvp-release-history-v1.0.0.md)
- [Local MVP Public Summary](docs/local-mvp-public-summary-v1.0.0.md)
- [Protocol Spec Index](docs/specs/protocol-spec-index-v0.1.md)
- [Developer Quickstart](docs/developer-quickstart.md)
- [Architecture Overview](docs/architecture-overview.md)
- [Security Review Scope](docs/security-review-scope.md)
- [Contributing Guide](CONTRIBUTING.md)

## Local Data

The MVP stores local ledger state at:

```text
~/.proofmoney/ledger.json
```

The MVP stores the local test wallet at:

```text
~/.proofmoney/wallets/default.json
```

The MVP exports local proof JSON files to:

```text
~/.proofmoney/export/
```

The MVP prepares local Proof Explorer files at:

```text
~/.proofmoney/explorer/
```

These local files are for MVP testing only.

## Risk Notice

ProofMoney is experimental. PRM may never have market value. Test units, if any, have no monetary value. Do not use experimental software with valuable assets.

## Author

Initial design authored by Vingen Motoki.
