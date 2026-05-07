# ProofMoney Internal Review Index

## Purpose

This index collects founder-led internal review and hardening materials for ProofMoney Core.

This is not an external audit.

This is not a production certification.

This is an internal review pass intended to make the local MVP safer, clearer, and more reviewable before broader outside review.

---

## Current Status

```text
Version: v0.7.0-internal-review-hardening
Review type: Founder-led internal review
External audit status: Not audited
Network status: Local MVP only
Wallet status: Experimental MVP wallet only
```

---

## Internal Review Documents

- [Internal Review Summary](internal-review-summary.md)
- [Internal Review Findings](internal-review-findings.md)
- [Amount Model Internal Review](internal-review-amount-model.md)
- [Event Hashing and Ledger Internal Review](internal-review-event-hashing-ledger.md)
- [Wallet and Ownership Internal Review](internal-review-wallet-ownership.md)
- [Proof of Flow and Balance Internal Review](internal-review-proof-of-flow.md)
- [Proof Export and Explorer Internal Review](internal-review-proof-export.md)
- [Documentation Safety Wording Review](internal-review-docs-safety-wording.md)
- [Negative Test Coverage Notes](negative-test-coverage.md)

---

## Review Areas

The internal review focuses on:

- amount model and decimal parsing;
- supply boundary;
- event hashing;
- local ledger state transitions;
- MVP wallet risk;
- ownership proof language;
- Proof of Flow;
- local balance validation;
- proof export structures;
- static local explorer wording;
- documentation safety boundaries;
- negative test coverage.

---

## What This Review Is Not

This review is not:

- an external audit;
- a public network readiness certificate;
- a production wallet security review;
- legal or regulatory advice;
- economic model validation;
- exchange-readiness validation;
- token launch approval.

---

## Safety Statement

ProofMoney Core remains a local MVP prototype.

Test units, if any, have no monetary value.

The MVP wallet must not be used with valuable assets.
