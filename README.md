# ProofMoney Core

ProofMoney is an open protocol initiative for **verifiable money integrity**.

> If money cannot be verified, it is only a promise.

This repository contains the Rust local MVP prototype for the ProofMoney Integrity Stack.

## Current Status

Current development target:

```text
v0.6.0-external-review
```

This repository is a local MVP prototype prepared for external Rust, architecture, and security review.

It is not a public network.  
It does not create PRM with monetary value.  
It does not represent a token sale, investment opportunity, yield product, airdrop claim, exchange listing, hosted public API, or future allocation right.

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
- [Amount Model Review Notes](docs/review-amount-model.md)
- [Event Hashing and Ledger Review Notes](docs/review-event-hashing-ledger.md)
- [Wallet and Ownership Risk Review Notes](docs/review-wallet-ownership-risk.md)
- [Proof of Flow and Balance Review Notes](docs/review-proof-of-flow-balance.md)
- [Proof Export and Explorer Review Notes](docs/review-proof-export-explorer.md)
- [Public Developer Announcement Draft](docs/public-developer-announcement-draft.md)

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

## External Review Status

The v0.6.0 milestone prepares the repository for external review.

It does not mean ProofMoney has been audited.

It does not mean ProofMoney is production-ready.

## Risk Notice

ProofMoney is experimental. PRM may never have market value. Test units, if any, have no monetary value. Do not use experimental software with valuable assets.

## Author

Initial design authored by Vingen Motoki.
