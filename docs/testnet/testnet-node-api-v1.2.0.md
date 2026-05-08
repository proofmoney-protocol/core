# ProofMoney Testnet Node/API Model v1.2.0

## Purpose

This document describes the first minimal ProofMoney testnet node/API skeleton.

This is not a live public testnet.

It is not mainnet.

It is not a hosted public API.

---

## Added Crate

```text
crates/proofmoney-node/
```

The crate provides model structures and placeholder service logic for future testnet node/API work.

---

## Current Crate Scope

The `proofmoney-node` crate includes:

- node config model;
- node status model;
- ledger status API model;
- proof query API model;
- event list API model;
- disabled-by-default event submission model;
- disabled-by-default faucet API model;
- API error model;
- node service placeholder;
- unit tests.

---

## Proposed Endpoint Models

The crate models future endpoint responses for:

```text
GET /status
GET /ledger/status
GET /proofs/:proof_type
GET /events
GET /events/releases
GET /events/transfers
POST /events
POST /faucet/request
```

These endpoints are model definitions only.

No live server is exposed by default.

---

## Node Status Model

The status model includes:

- node version;
- network name;
- protocol version;
- mode;
- ledger height;
- event count;
- last event hash;
- sync status;
- write API enabled flag;
- faucet enabled flag;
- safety notice.

---

## Ledger Status Model

The ledger status model includes:

- ledger version;
- current height;
- current supply;
- Public Proof Fund balance;
- rule version;
- event count;
- last event hash;
- network label;
- safety notice.

---

## Proof Query Model

The proof query model supports future queries for:

- Starting State Proof;
- Proof of Supply;
- Proof of Rule;
- Proof of Flow;
- Integrity Status.

Proof outputs remain testnet/local MVP verification results and do not imply external audit or public settlement finality.

---

## Event List Model

The event list model supports:

- all events;
- release events;
- transfer events;
- pagination fields;
- event hash validity;
- event status;
- safety notice.

Event listing is for proof visibility only.

It is not a trading, claim, or exchange interface.

---

## Disabled Event Submission Draft

The event submission model supports a future:

```text
POST /events
```

In v1.2.0 this is disabled by default.

No public write API is launched.

---

## Disabled Faucet Draft

The faucet model supports a future:

```text
POST /faucet/request
```

In v1.2.0 this is disabled by default.

The faucet model includes:

- no monetary value notice;
- no airdrop notice;
- no test-to-main conversion notice;
- safety notice.

---

## Safety Statement

This is a testnet node/API skeleton only.

It is not:

- live public testnet;
- mainnet;
- token sale;
- airdrop;
- yield product;
- exchange integration;
- hosted public API;
- production wallet;
- completed audit.
