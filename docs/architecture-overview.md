# ProofMoney Core Architecture Overview

## Purpose

This document gives Rust engineers and reviewers a concise overview of the ProofMoney Core local MVP architecture.

ProofMoney Core is currently a local MVP prototype.

It is not a public network, token sale, yield product, exchange integration, airdrop claim, hosted public API, or production wallet.

---

## Workspace Structure

The Rust workspace contains:

| Crate | Responsibility |
|---|---|
| `proofmoney-cli` | Command-line interface and local command orchestration |
| `proofmoney-types` | Shared domain types: amounts, events, rules, proof result models |
| `proofmoney-crypto` | Hashing, key generation, signing, signature verification, address helpers |
| `proofmoney-ledger` | Local ledger state, event hashing, event application, balance computation |
| `proofmoney-release` | Proof Release Curve logic and release event validation |
| `proofmoney-proof` | Proof Engine modules: Supply, Rule, Ownership, Flow, Integrity Status |
| `proofmoney-wallet` | Experimental local wallet model and message signing helpers |
| `proofmoney-storage` | Local JSON storage and path helpers |

---

## High-Level Data Flow

```text
CLI command
  ↓
Local storage load/init
  ↓
Domain operation
  ↓
Proof validation
  ↓
Ledger mutation or proof output
  ↓
JSON/human-readable output
```

---

## CLI Layer

The CLI is the primary MVP interface.

It supports:

- starting state verification;
- release event simulation;
- local ledger status;
- supply verification;
- rule verification;
- wallet creation;
- address inspection;
- message signing;
- transfer event creation;
- Proof of Flow verification;
- proof snapshot export;
- local Proof Explorer preparation.

---

## Local Ledger Layer

The ledger stores local MVP state.

Local ledger path:

```text
~/.proofmoney/ledger.json
```

The ledger contains:

- ledger version;
- current height;
- current supply;
- Public Proof Fund balance;
- rule version;
- last event hash;
- event list.

Ledger state is local only.

It is not public network consensus.

---

## Event Model

Current MVP event types:

- StartingState
- Release
- Transfer
- PublicFund
- Rule

Event hashing uses a stable event hash view and excludes the mutable `hash` field itself.

The rule is:

```text
same event content = same hash
changed event content = changed hash
hash field itself is excluded
```

---

## Proof Engine Layer

The Proof Engine validates:

- Proof of Supply;
- Proof of Rule;
- Proof of Ownership;
- Proof of Flow;
- Integrity Status.

Proof outputs use structured `ProofResult` models with:

- proof type;
- status;
- rule version;
- data;
- summary.

---

## Wallet Layer

The wallet layer is experimental.

Local wallet path:

```text
~/.proofmoney/wallets/default.json
```

The MVP wallet can:

- generate local test keypair;
- derive local address;
- sign messages;
- support ownership verification testing.

The wallet is not production custody software.

Do not use it with valuable assets.

---

## Proof Export Layer

The proof export layer generates local JSON proof files.

Export path:

```text
~/.proofmoney/export/
```

Explorer path:

```text
~/.proofmoney/explorer/
```

The static explorer is a local proof viewer only.

It is not a hosted public explorer or trading interface.

---

## MVP Limitations

The MVP does not include:

- public network;
- peer-to-peer networking;
- consensus;
- mempool;
- production wallet security;
- encrypted wallet storage;
- public hosted API;
- production explorer;
- independent security audit;
- exchange integration;
- token sale;
- airdrop;
- yield product.

---

## Safety Boundary

All current behavior is local MVP behavior.

No command should be interpreted as creating monetary value, public network finality, investment return, exchange access, or custody safety.
