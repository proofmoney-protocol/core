# ProofMoney Testnet Design Index v1.1.0

## Purpose

This index collects the ProofMoney public testnet design documents for v1.1.0.

This is design only.

It is not a public testnet launch.

It is not a mainnet launch.

It is not a token sale, airdrop, yield product, exchange integration, or production wallet release.

---

## Current Status

```text
Milestone: v1.1.0-testnet-design
Status: Public testnet architecture design
Local MVP baseline: v1.0.0-local-mvp-freeze
Public testnet: Not launched
Mainnet: Not launched
Audit status: Not audited
```

---

## Design Documents

| Area | File |
|---|---|
| Public testnet goals and scope | `docs/testnet/testnet-goals-and-scope-v1.1.0.md` |
| Testnet node architecture | `docs/testnet/testnet-node-architecture-v1.1.0.md` |
| Testnet network message model | `docs/testnet/testnet-network-message-model-v1.1.0.md` |
| Testnet ledger sync model | `docs/testnet/testnet-ledger-sync-model-v1.1.0.md` |
| Testnet faucet boundary | `docs/testnet/testnet-faucet-boundary-v1.1.0.md` |
| Public testnet explorer requirements | `docs/testnet/testnet-explorer-requirements-v1.1.0.md` |
| Testnet wallet safety policy | `docs/testnet/testnet-wallet-safety-policy-v1.1.0.md` |

---

## Design Goal

The goal is to prepare a safe and reviewable public testnet architecture before writing public-network implementation code.

The design should answer:

- what a testnet is for;
- what a node does;
- how events are submitted;
- how state is read;
- how ledger sync may work;
- what faucet boundaries are required;
- what explorer views are needed;
- what wallet safety warnings are required.

---

## Out of Scope

This milestone does not implement:

- public node;
- public API service;
- live faucet;
- hosted explorer;
- mainnet;
- token sale;
- airdrop claim;
- yield mechanism;
- exchange integration;
- production wallet.

---

## Safety Statement

Public testnet units, if any in a future milestone, must have no monetary value and must not imply future mainnet allocation.
