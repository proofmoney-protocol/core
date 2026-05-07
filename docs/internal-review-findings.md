# ProofMoney Internal Review Findings

## Purpose

This document records founder-led internal review findings for ProofMoney Core.

This is not an external audit report.

---

## Finding Status Legend

```text
Open
Accepted
Fixed
Documented Risk
Out of Scope Current MVP
Needs External Review
```

---

## Findings

| ID | Severity | Area | Status | Summary |
|---|---|---|---|---|
| IR-001 | Medium | Wallet | Documented Risk | MVP wallet stores local test key material and must not be described as production custody. |
| IR-002 | Medium | Ledger | Needs External Review | Long-term protocol use may require stronger canonical serialization and event chain linking. |
| IR-003 | Low | Proof Export | Documented Risk | Exported JSON can become stale after local ledger changes unless regenerated. |
| IR-004 | Low | Documentation | Fixed | Wording must consistently distinguish internal review from external audit. |
| IR-005 | Medium | Flow | Documented Risk | Proof of Flow is local MVP validation only and does not represent settlement finality. |
| IR-006 | Medium | Tests | Fixed | Negative tests should cover invalid hashes, rule mismatch, amount parsing, and insufficient balance. |

---

## IR-001: MVP Wallet Risk

Severity:

```text
Medium
```

Status:

```text
Documented Risk
```

The MVP wallet is intentionally experimental.

Risk:

- local test key material may be stored in local files;
- users could misunderstand wallet output as production custody;
- private key handling is not hardened.

Response:

- documentation repeats that the MVP wallet is not production custody software;
- `SECURITY.md` and review docs highlight wallet risk;
- contributor guide forbids production custody claims.

---

## IR-002: Event Serialization and Chain Linking

Severity:

```text
Medium
```

Status:

```text
Needs External Review
```

The MVP hashes an event hash view that excludes the mutable `hash` field.

Risk:

- JSON serialization may not be sufficient as a long-term canonical protocol format;
- long-term event chain linking may need stronger previous-hash linkage;
- local file mutation remains possible.

Response:

- documented as a known risk;
- future external review should inspect canonical serialization requirements.

---

## IR-003: Proof Export Staleness

Severity:

```text
Low
```

Status:

```text
Documented Risk
```

Exported proof files are snapshots.

Risk:

- local ledger may change after export;
- exported files may become stale.

Response:

- documentation states exported files are local MVP data only;
- users should regenerate exports after state changes.

---

## IR-004: Internal Review vs External Audit Wording

Severity:

```text
Low
```

Status:

```text
Fixed
```

Risk:

- users may confuse internal review with external audit.

Response:

- README and review docs now clearly state founder-led internal review;
- docs explicitly state this is not an external audit.

---

## IR-005: Proof of Flow Scope

Severity:

```text
Medium
```

Status:

```text
Documented Risk
```

Proof of Flow is local MVP validation.

Risk:

- users may interpret it as public settlement finality.

Response:

- documentation states Proof of Flow does not imply public settlement finality;
- docs clarify no public network or consensus exists.

---

## IR-006: Negative Test Coverage

Severity:

```text
Medium
```

Status:

```text
Fixed
```

Risk:

- invalid local states should be tested explicitly.

Response:

- added negative tests for amount parsing and local ledger invalid states;
- documented coverage boundaries.

---

## Safety Statement

These findings are internal review findings.

They do not represent an external audit.

ProofMoney remains experimental and may fail.
