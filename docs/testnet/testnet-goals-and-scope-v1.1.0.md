# ProofMoney Public Testnet Goals and Scope v1.1.0

## Purpose

This document defines the goals and scope of a future ProofMoney public testnet.

This is design only.

It is not a public testnet launch.

---

## Testnet Purpose

A ProofMoney public testnet should allow developers and reviewers to test ProofMoney protocol behavior in a shared, non-monetary environment.

The public testnet should make it possible to inspect:

- event submission;
- proof verification;
- ledger state visibility;
- release event behavior;
- transfer event behavior;
- proof export behavior;
- explorer visibility;
- wallet safety boundaries.

---

## What Developers Can Test

Developers should be able to test:

- submitting local-style release events where allowed;
- submitting transfer events;
- querying ledger height;
- querying current supply;
- querying Public Proof Fund balance;
- querying event lists;
- querying release event proofs;
- querying transfer event proofs;
- verifying event hashes;
- verifying Proof of Supply;
- verifying Proof of Rule;
- verifying Proof of Flow;
- exporting proof snapshots;
- viewing testnet data in explorer.

---

## Proof Types To Expose

The public testnet should expose or support:

- Starting State Proof;
- Proof of Issuance;
- Proof of Supply;
- Proof of Ownership;
- Proof of Flow;
- Proof of Rule;
- Integrity Status.

---

## Expected CLI / API Flows

Expected testnet flows may include:

```text
node status
submit event
submit transfer
query ledger status
query proof result
query release events
query transfer events
export proof snapshot
view explorer
request test units from faucet
```

---

## Local MVP vs Public Testnet vs Future Mainnet

## Local MVP

- local files;
- local CLI;
- local wallet testing;
- no public node;
- no network consensus;
- no public explorer;
- no monetary value.

## Public Testnet

- shared test environment;
- testnet nodes;
- public testnet data;
- test units only;
- no monetary value;
- no mainnet allocation promise;
- may be reset or broken.

## Future Mainnet

- not defined in this milestone;
- would require separate architecture, security review, compliance review, wallet policy, and launch readiness.

---

## Explicitly Out of Scope

The public testnet design does not include:

- mainnet launch;
- token sale;
- airdrop;
- yield;
- exchange integration;
- price chart;
- trading interface;
- production wallet;
- legal classification;
- guaranteed future allocation.

---

## Test Unit Safety Boundary

Future public testnet units must be described as:

```text
test units with no monetary value
```

They must not be described as:

```text
free tokens
airdrop
early allocation
claimable PRM
future mainnet PRM
investment opportunity
```

---

## Safety Statement

Public testnet units have no monetary value and do not imply future mainnet allocation.
