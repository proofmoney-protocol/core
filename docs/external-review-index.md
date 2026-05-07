# ProofMoney External Review Index

## Purpose

This index collects the documents prepared for external Rust, architecture, and security review.

This is not an audit report.

This is review preparation material.

---

## Core Review Documents

- [MVP Scope Freeze](mvp-scope-freeze.md)
- [Security Review Scope](security-review-scope.md)
- [Architecture Overview](architecture-overview.md)
- [Developer Quickstart](developer-quickstart.md)
- [Contributor Guide](../CONTRIBUTING.md)

---

## Review Notes

- [Amount Model and Decimal Parsing Review](review-amount-model.md)
- [Event Hashing and Ledger State Transition Review](review-event-hashing-ledger.md)
- [Wallet and Ownership Proof Risk Review](review-wallet-ownership-risk.md)
- [Proof of Flow and Balance Validation Review](review-proof-of-flow-balance.md)
- [Proof Export and Local Explorer Review](review-proof-export-explorer.md)

---

## Developer Communication

- [Public Developer Announcement Draft](public-developer-announcement-draft.md)

---

## Current Review Status

```text
Status: Prepared for external review
Audit Status: Not audited
Network Status: Local MVP only
Wallet Status: Experimental local MVP wallet only
```

---

## What Reviewers Should Not Assume

Reviewers should not assume:

- a public network exists;
- PRM has monetary value;
- local wallet is production safe;
- local proof outputs are public consensus;
- the local Proof Explorer is a hosted block explorer;
- this repository has been audited;
- test units convert to future units.

---

## Safety Statement

ProofMoney is experimental and may fail.

Test units, if any, have no monetary value.
