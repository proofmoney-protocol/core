# ProofMoney Testnet Node Architecture v1.1.0

## Purpose

This document defines the initial architecture direction for a future ProofMoney public testnet node.

This is design only.

It is not a public network launch.

---

## Node Responsibilities

A testnet node should be responsible for:

- accepting valid testnet events;
- rejecting invalid events;
- storing testnet ledger state;
- verifying event hashes;
- applying ledger state transitions;
- exposing read APIs;
- optionally exposing write APIs;
- exporting proof data;
- reporting node status;
- supporting explorer data queries.

---

## Local Ledger vs Testnet Ledger

## Local MVP Ledger

- stored at local user path;
- single-user local file state;
- CLI-driven;
- no network sync.

## Testnet Ledger

- node-managed state;
- shared test environment;
- queryable by public clients;
- may require sync or replication;
- must clearly be testnet only.

---

## Node Storage Model

Initial testnet node storage may include:

- ledger events;
- ledger metadata;
- current supply;
- Public Proof Fund balance;
- balances;
- last event hash;
- proof export snapshots;
- node configuration.

Storage can begin with a simple persistent store before moving to more advanced databases.

---

## Event Acceptance Flow

A future testnet node should accept an event through a flow similar to:

```text
receive event
validate schema
validate rule version
validate event hash
validate event type
validate payload
validate proof-specific rules
check current ledger state
apply event if valid
persist event
update derived state
return result
```

---

## Proof Verification Flow

The node should support proof verification for:

- supply;
- rule;
- ownership where applicable;
- flow;
- integrity status.

Proof verification should be deterministic and reproducible.

---

## Read API Responsibilities

Read APIs may expose:

- node status;
- ledger status;
- event by id;
- event list;
- release event list;
- transfer event list;
- proof snapshot;
- supply proof;
- rule proof;
- integrity status.

---

## Write API Responsibilities

Write APIs may support:

- transfer event submission;
- faucet request;
- test-only release event where permitted;
- testnet event submission.

Write APIs must enforce:

- input validation;
- rate limits where needed;
- no monetary claims;
- no mainnet claim implication.

---

## Node Operator Assumptions

Early testnet node operation may assume:

- trusted bootstrap configuration;
- limited validator set;
- simple sync;
- frequent reset possibility;
- unstable API;
- testnet-only state.

---

## MVP Simplifications

Initial testnet design may simplify:

- consensus;
- peer discovery;
- mempool;
- fee market;
- validator economics;
- slashing;
- governance.

These simplifications must be documented and not hidden.

---

## Safety Boundary

This is a testnet architecture design only, not a public network launch or mainnet specification.
