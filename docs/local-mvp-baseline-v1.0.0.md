# ProofMoney Local MVP Baseline Manifest v1.0.0

## Status

```text
Version: v1.0.0-local-mvp-freeze
Baseline type: Stable local MVP baseline
Network status: Local MVP only
Audit status: Not audited
Wallet status: Experimental local MVP wallet only
```

## Purpose

This manifest defines the first stable local MVP baseline for ProofMoney Core.

This baseline consolidates the current implemented local MVP scope, command surface, proof types, demo flows, protocol specs, documentation, and safety boundaries.

This is not a public network launch.

This is not a token launch.

This is not an exchange listing.

This is not an external audit.

---

## Current Core Version

```text
v1.0.0-local-mvp-freeze
```

---

## Local MVP Scope

The v1.0.0 local MVP baseline includes:

- Rust Cargo workspace;
- ProofMoney CLI;
- local ledger JSON persistence;
- local ledger reset;
- local state validation;
- local tamper detection;
- local release event simulation and append flow;
- local wallet generation;
- local address inspection;
- local message signing;
- local ownership proof;
- local transfer event creation;
- local Proof of Flow;
- local balance recomputation;
- proof JSON export;
- proof snapshot export with freshness metadata;
- static local Proof Explorer preparation;
- local demo scripts;
- isolated CLI integration tests;
- protocol specification documents.

---

## Supported Proof Types

The local MVP supports:

- Starting State Proof;
- Proof of Issuance;
- Proof of Supply;
- Proof of Ownership;
- Proof of Flow;
- Proof of Rule;
- Integrity Status.

---

## Supported CLI Commands

The local MVP command surface includes:

```bash
proofmoney starting-state
proofmoney simulate-release --interval 1
proofmoney simulate-release --interval 1 --append
proofmoney ledger-status
proofmoney reset-ledger --yes
proofmoney validate-local-state
proofmoney detect-tampering
proofmoney verify-supply
proofmoney verify-rule
proofmoney integrity-status
proofmoney create-wallet --force
proofmoney new-address
proofmoney sign-message --message "hello"
proofmoney verify-ownership
proofmoney create-transfer
proofmoney verify-flow
proofmoney export-proof-snapshot
proofmoney export-proof-site-data
proofmoney list-release-events
proofmoney list-transfer-events
proofmoney prepare-explorer
```

---

## Supported Local Demo Scripts

```bash
bash scripts/demo-local.sh
bash scripts/demo-transfer-local.sh
```

---

## Protocol Specification Files

```text
docs/specs/protocol-spec-index-v0.1.md
docs/specs/amount-model-v0.1.md
docs/specs/event-schema-v0.1.md
docs/specs/proof-result-schema-v0.1.md
docs/specs/proof-release-curve-v0.1.md
docs/specs/local-ledger-state-v0.1.md
docs/specs/proof-export-json-v0.1.md
docs/specs/wallet-ownership-safety-v0.1.md
```

---

## Local Storage Paths

Local ledger:

```text
~/.proofmoney/ledger.json
```

Local test wallet:

```text
~/.proofmoney/wallets/default.json
```

Proof export data:

```text
~/.proofmoney/export/
```

Local Proof Explorer:

```text
~/.proofmoney/explorer/
```

---

## Known Limitations

The v1.0.0 local MVP does not include:

- public network;
- public consensus;
- peer-to-peer networking;
- mempool;
- production wallet security;
- encrypted wallet storage;
- hosted public API;
- production explorer;
- external audit;
- token sale;
- airdrop claim;
- yield product;
- exchange integration;
- monetary value.

---

## Safety Boundary

This baseline is a local MVP freeze only.

It is not:

- a public network;
- a mainnet;
- a testnet token launch;
- a token sale;
- an airdrop;
- a yield product;
- an exchange integration;
- a production wallet;
- an external audit.

ProofMoney is experimental and may fail.

Test units, if any, have no monetary value.
