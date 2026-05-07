# ProofMoney Core Security Review Scope

## Purpose

This document defines the initial security review scope for ProofMoney Core.

This does not claim that ProofMoney has been audited.

It prepares the project for external Rust/security review.

---

## Current Review Target

Repository:

```text
https://github.com/proofmoney-protocol/core
```

Current scope:

```text
local MVP prototype
```

Out of scope:

- public network security;
- production custody;
- exchange integration;
- token sale systems;
- hosted API infrastructure;
- production explorer hosting;
- legal or regulatory review.

---

## Review Areas

## 1. Amount Model

Review:

- integer-only amount model;
- smallest unit handling;
- decimal parsing;
- overflow handling;
- invalid amount rejection.

Relevant crate:

```text
proofmoney-types
```

---

## 2. Release Curve

Review:

- release curve calculation;
- protection window logic;
- Public Proof Fund allocation;
- contributor reward calculation;
- supply boundary behavior.

Relevant crate:

```text
proofmoney-release
```

---

## 3. Event Hashing

Review:

- deterministic serialization;
- exclusion of mutable hash field;
- hash mismatch detection;
- event mutation risks.

Relevant crate:

```text
proofmoney-ledger
```

---

## 4. Ledger State Transition

Review:

- event append rules;
- height checks;
- supply recomputation;
- Public Proof Fund recomputation;
- balance computation;
- insufficient balance rejection.

Relevant crate:

```text
proofmoney-ledger
```

---

## 5. Proof Verification

Review:

- Proof of Supply;
- Proof of Rule;
- Proof of Ownership;
- Proof of Flow;
- Integrity Status.

Relevant crate:

```text
proofmoney-proof
```

---

## 6. Wallet Handling

Review:

- local key generation;
- private key storage warning;
- signing flow;
- address derivation;
- ownership verification.

Relevant crates:

```text
proofmoney-wallet
proofmoney-crypto
```

Known limitation:

The MVP wallet is not production custody software.

Private key material may be stored locally for MVP testing.

---

## 7. Signature Verification

Review:

- signing message consistency;
- public key to address relationship;
- signature verification;
- invalid signature rejection.

Relevant crates:

```text
proofmoney-crypto
proofmoney-proof
proofmoney-ledger
```

---

## 8. Local Storage

Review:

- local file paths;
- JSON read/write;
- overwrite behavior;
- wallet overwrite guard;
- error handling.

Relevant crate:

```text
proofmoney-storage
```

---

## 9. Proof Export

Review:

- proof snapshot correctness;
- release event proof listing;
- transfer event proof listing;
- safety notices;
- exported data shape.

Relevant crate:

```text
proofmoney-cli
```

---

## Known MVP Limitations

- no public network;
- no consensus;
- no mempool;
- no encrypted wallet storage;
- no production wallet security;
- no hosted API;
- no production explorer;
- no independent audit yet.

---

## Safety Statement

ProofMoney does not offer, sell, or solicit the purchase of PRM.

PRM does not guarantee price, liquidity, yield, profit, or exchange access.

ProofMoney is experimental and may fail.

Test units, if any, have no monetary value.
