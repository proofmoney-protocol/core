# ProofMoney Protocol Specification Index v0.1

## Purpose

This index collects the current local MVP protocol specifications for ProofMoney Core.

This is a protocol specification freeze for the local MVP semantics.

It is not a public network specification.

It is not an external audit.

It is not a production wallet specification.

---

## Current Status

```text
Spec version: v0.1
Core milestone: v0.9.0-protocol-spec-freeze
Network status: Local MVP only
Audit status: Not audited
Wallet status: Experimental local MVP wallet only
```

---

## Specification Documents

| Specification | File |
|---|---|
| Amount Model Specification | `docs/specs/amount-model-v0.1.md` |
| Event Schema Specification | `docs/specs/event-schema-v0.1.md` |
| Proof Result Schema Specification | `docs/specs/proof-result-schema-v0.1.md` |
| Proof Release Curve Specification | `docs/specs/proof-release-curve-v0.1.md` |
| Local Ledger State Specification | `docs/specs/local-ledger-state-v0.1.md` |
| Proof Export JSON Specification | `docs/specs/proof-export-json-v0.1.md` |
| Wallet and Ownership Safety Specification | `docs/specs/wallet-ownership-safety-v0.1.md` |

---

## Frozen MVP Semantics

The v0.1 specification freezes the current local MVP semantics for:

- amount unit representation;
- event structure;
- event hashing boundary;
- proof result structure;
- local release simulation;
- local ledger state;
- local state validation;
- local proof export;
- MVP wallet safety boundary.

This freeze means that future changes to these semantics should be intentional, reviewed, documented, and versioned.

---

## What Is Not Frozen

This specification does not freeze:

- public network consensus;
- peer-to-peer behavior;
- mempool behavior;
- production wallet custody model;
- exchange integration;
- token sale mechanics;
- legal or regulatory classification;
- final economic policy;
- mainnet behavior.

---

## Safety Statement

ProofMoney Core remains a local MVP prototype.

Test units, if any, have no monetary value.

The MVP wallet must not be used with valuable assets.
