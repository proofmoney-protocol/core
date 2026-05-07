# ProofMoney MVP Scope Freeze

## Version

```text
v0.6.0-external-review
```

## Purpose

This document freezes the current ProofMoney Core MVP scope before external Rust, architecture, and security review.

The goal is to prevent uncontrolled feature expansion while reviewers inspect the existing local MVP.

This scope freeze does not imply production readiness.

This scope freeze does not imply public network readiness.

This scope freeze does not imply that ProofMoney has been audited.

---

## In Scope

The current MVP includes:

- Rust Cargo workspace;
- local command-line interface;
- Starting State Proof;
- Proof of Issuance;
- Proof of Supply;
- Proof of Ownership;
- Proof of Flow;
- Proof of Rule;
- local ledger JSON persistence;
- deterministic event hashing;
- release event creation and append flow;
- computed supply verification;
- local wallet persistence for MVP testing;
- address inspection;
- message signing;
- local transfer event creation;
- local balance tracking;
- insufficient balance rejection;
- local proof JSON export;
- proof snapshot export;
- release event proof listing;
- transfer event proof listing;
- static local Proof Explorer prototype;
- developer quickstart;
- local demo script;
- sample proof fixtures;
- architecture overview;
- security review scope;
- contributor guide.

---

## Out of Scope

The current MVP explicitly does not include:

- public network;
- public consensus;
- peer-to-peer networking;
- mempool;
- mining;
- staking;
- token sale;
- presale;
- private allocation;
- airdrop claim;
- yield product;
- exchange integration;
- price charts;
- trading interface;
- hosted public API;
- public production explorer;
- production wallet security;
- encrypted wallet storage;
- mobile wallet;
- browser wallet;
- custody service;
- final economic policy;
- regulatory approval;
- completed external audit.

---

## Freeze Rules

During external review preparation:

1. Do not add new protocol features.
2. Do not add promotional financial language.
3. Do not add PRM sale, claim, yield, price, or exchange functionality.
4. Do not present MVP wallet functionality as production custody.
5. Do not present local ledger state as public network state.
6. Do not present sample fixtures as real balances or real transactions.
7. Do not change release curve parameters unless opened as a separate review issue.
8. Do not change amount units unless opened as a separate review issue.
9. Do not change event hashing behavior without a review issue.
10. Do not change supply logic without review notes and tests.

---

## Review Objective

External reviewers should focus on:

- correctness of local proof logic;
- code clarity;
- amount safety;
- event hashing determinism;
- ledger transition correctness;
- wallet risk;
- signature verification;
- balance validation;
- proof export correctness;
- misleading language removal;
- unsafe assumptions.

---

## Safety Statement

ProofMoney Core remains a local MVP prototype.

It is not a public network.

It does not create PRM with monetary value.

It is not a token sale, yield product, airdrop claim, exchange integration, hosted public API, or production wallet.
