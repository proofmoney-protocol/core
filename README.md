# ProofMoney Core

ProofMoney is an open protocol initiative for **verifiable money integrity**.

> If money cannot be verified, it is only a promise.

This repository contains the Rust local MVP prototype for the ProofMoney Integrity Stack.

## Current Status

Current development target:

```text
v0.8.0-cli-integration-hardening
```

This repository is a local MVP prototype under founder-led internal review and CLI integration hardening.

It is not a public network.  
It does not create PRM with monetary value.  
It does not represent a token sale, investment opportunity, yield product, airdrop claim, exchange listing, hosted public API, completed audit, or future allocation right.

## Review Status

```text
Founder-led internal review and CLI hardening in progress
External audit status: Not audited
Network status: Local MVP only
Wallet status: Experimental local MVP wallet only
```

## MVP Scope

The MVP currently includes:

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

## Not Included

The MVP does not include:

- public network;
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

## Documentation

- [Developer Quickstart](docs/developer-quickstart.md)
- [Architecture Overview](docs/architecture-overview.md)
- [Proof Explorer and Local Proof API](docs/proof-explorer-api.md)
- [CLI Integration Hardening](docs/cli-integration-hardening.md)
- [Local State Validation](docs/local-state-validation.md)
- [Transfer Demo](docs/transfer-demo.md)
- [Security Review Scope](docs/security-review-scope.md)
- [Contributing Guide](CONTRIBUTING.md)
- [MVP Scope Freeze](docs/mvp-scope-freeze.md)
- [Internal Review Index](docs/internal-review-index.md)

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
