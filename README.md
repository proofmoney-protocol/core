# ProofMoney Core

ProofMoney is an open protocol initiative for **verifiable money integrity**.

> If money cannot be verified, it is only a promise.

This repository contains the Rust local MVP prototype for the ProofMoney Integrity Stack.

## Current Status

Current development target:

```text
v0.7.0-internal-review-hardening
```

This repository is a local MVP prototype under founder-led internal review and hardening.

It is not a public network.  
It does not create PRM with monetary value.  
It does not represent a token sale, investment opportunity, yield product, airdrop claim, exchange listing, hosted public API, completed audit, or future allocation right.

## Review Status

```text
Founder-led internal review in progress
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
- release event append flow
- computed supply verification
- local wallet persistence
- transfer event creation
- local balance tracking
- local proof JSON export
- proof snapshot generation
- release event proof listing
- transfer event proof listing
- static local Proof Explorer prototype
- developer quickstart
- local demo script
- sample proof fixtures
- architecture overview
- security review scope
- contributor guide
- MVP scope freeze
- external review preparation notes
- internal review and hardening notes
- negative test coverage for selected invalid local MVP states

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
```

## Core CLI Examples

```bash
cargo run -p proofmoney-cli -- starting-state
cargo run -p proofmoney-cli -- simulate-release --interval 1 --append
cargo run -p proofmoney-cli -- ledger-status
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
- [Ownership and Flow MVP](docs/ownership-and-flow.md)
- [Security Review Scope](docs/security-review-scope.md)
- [Contributing Guide](CONTRIBUTING.md)
- [MVP Scope Freeze](docs/mvp-scope-freeze.md)
- [External Review Index](docs/external-review-index.md)
- [Internal Review Index](docs/internal-review-index.md)
- [Internal Review Summary](docs/internal-review-summary.md)
- [Internal Review Findings](docs/internal-review-findings.md)
- [Negative Test Coverage Notes](docs/negative-test-coverage.md)

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

## Sample Fixtures

Sample proof data lives under:

```text
fixtures/
```

Fixtures are sample data only. They have no monetary value and contain no private keys.

## Risk Notice

ProofMoney is experimental. PRM may never have market value. Test units, if any, have no monetary value. Do not use experimental software with valuable assets.

## Author

Initial design authored by Vingen Motoki.
