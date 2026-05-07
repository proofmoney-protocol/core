# Negative Test Coverage Notes

## Purpose

This document records negative test coverage added or checked during the v0.7.0 internal review hardening pass.

This does not imply production readiness.

---

## Covered Areas

## Amount Model

Negative tests should cover:

- malformed decimal strings;
- negative amounts;
- too many fractional decimals;
- empty strings;
- overflow-like inputs where applicable.

## Ledger State

Negative tests should cover:

- invalid event hash;
- rule version mismatch;
- insufficient balance transfer;
- mutated transfer hash.

## Flow and Wallet

Negative tests should cover or document:

- zero transfer amount;
- invalid signature;
- address/public key mismatch;
- missing wallet;
- sender mismatch.

Some of these are CLI-level behaviors and may require future CLI integration tests.

---

## Added Test Files

```text
crates/proofmoney-types/tests/amount_negative.rs
crates/proofmoney-ledger/tests/internal_review_negative.rs
```

---

## Current Limitation

The negative tests added in this pass focus on stable crate-level behavior.

Future hardening can add full CLI integration tests once test isolation around local `~/.proofmoney` paths is improved.

---

## Safety Statement

Negative tests improve local MVP confidence but do not imply production security.
