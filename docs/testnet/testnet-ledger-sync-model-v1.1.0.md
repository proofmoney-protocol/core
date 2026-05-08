# ProofMoney Testnet Ledger Sync Model v1.1.0

## Purpose

This document defines an initial ledger sync model for a future ProofMoney public testnet.

This is design only.

Ledger sync is not the same as public consensus.

---

## Sync Purpose

Ledger sync allows nodes or clients to align local testnet state with a peer or reference node.

Sync should help detect:

- missing events;
- mismatched event height;
- mismatched last event hash;
- invalid event hashes;
- invalid rule versions;
- divergent local state.

---

## Event Ordering Assumptions

Initial sync may assume:

- events are ordered by height;
- each event has a unique id;
- event height increases;
- last event hash identifies current ledger tip;
- invalid events are rejected.

---

## Height-Based Sync

Basic sync flow:

```text
query peer status
compare local height with peer height
if local height < peer height, request missing events
verify each event hash
verify rule version
apply events in order
update derived state
verify final last event hash
```

---

## Last Event Hash Checking

A node should compare:

```text
local last_event_hash
peer last_event_hash
```

If heights match but hashes differ, the node should treat the state as divergent.

---

## Event Hash Verification

Every received event should pass event hash verification before being applied.

Invalid hash behavior:

```text
reject event
record error
do not update local state
```

---

## Invalid Event Rejection

Events should be rejected for:

- invalid schema;
- invalid event hash;
- invalid rule version;
- non-increasing height;
- invalid transfer signature;
- insufficient balance;
- invalid amount;
- unsupported event type.

---

## Resync Behavior

If state divergence is detected, possible responses include:

- stop sync;
- report divergence;
- request checkpoint;
- request full event list;
- require manual reset;
- trigger future recovery flow.

The first testnet version may keep this simple and conservative.

---

## MVP Sync Limitations

Initial testnet sync may not include:

- Byzantine fault tolerance;
- full consensus;
- fork choice rule;
- validator set;
- slashing;
- finality gadget;
- fraud proofs;
- light client proofs.

---

## Future Consensus Gap

Ledger sync is not consensus.

A future mainnet design would need a separate consensus model, including:

- validator / node role definitions;
- finality assumptions;
- fork choice;
- network adversary model;
- economic or non-economic security model;
- state commitment design.

---

## Safety Boundary

Ledger sync is not the same as public consensus.

This design does not launch mainnet.
