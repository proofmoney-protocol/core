# Event Hashing and Ledger State Transition Review Notes

## Review Status

```text
Prepared for external review
```

This document records current self-review notes and areas for external reviewers.

It is not an audit report.

---

## Current Event Hashing Model

Events are hashed through a stable event hash view.

The mutable `hash` field itself should be excluded from the hash input.

Intended rule:

```text
same event content = same event hash
changed event content = changed event hash
hash field itself is excluded
```

---

## Current Ledger Transition Model

The local ledger supports:

- release event append;
- transfer event append;
- current height update;
- current supply recomputation;
- Public Proof Fund balance recomputation;
- address balance recomputation;
- insufficient balance rejection.

The ledger is local only.

It is not public consensus.

---

## Areas Reviewed Internally

## 1. Event Hash View

Reviewers should inspect whether the hash input excludes mutable fields and includes all necessary event fields.

## 2. Determinism

Reviewers should inspect whether JSON serialization is sufficiently deterministic for the current MVP.

If a stronger canonical serialization is needed, it should be opened as a follow-up issue.

## 3. Release Event Append

Release event append should:

- validate rule version;
- validate event hash;
- validate release calculation;
- update supply;
- update Public Proof Fund balance;
- update height.

## 4. Transfer Event Append

Transfer event append should:

- validate rule version;
- validate event hash;
- check sufficient balance;
- update balances;
- update height.

## 5. Supply Recalculation

Supply should be computed from release events rather than trusted blindly.

---

## Known Risks and Questions

- Is JSON serialization sufficiently canonical for long-term protocol use?
- Are event timestamps part of the hash input intentionally?
- Should event IDs be generated deterministically in future versions?
- Does height validation cover all local replay cases?
- Can local event files be manually mutated to create inconsistent state?
- Should ledger state include stronger chain linkage between event hashes?

---

## Suggested External Review Tasks

1. Review `hash_event`.
2. Review `EventHashView`.
3. Review `apply_event`.
4. Review `compute_supply_from_events`.
5. Review `compute_balances_from_events`.
6. Review invalid mutation scenarios.
7. Add tests for tampered events.

---

## Safety Statement

This review covers local MVP behavior only.

It does not certify public consensus or production ledger safety.
