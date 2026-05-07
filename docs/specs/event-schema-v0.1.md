# ProofMoney Event Schema Specification v0.1

## Status

```text
Spec version: v0.1
Scope: Local MVP event semantics
```

This specification documents the current local MVP event schema.

It does not define public network consensus.

---

## 1. Common Event Fields

Each event currently includes:

| Field | Type | Description |
|---|---|---|
| `id` | string | Event identifier |
| `event_type` | enum/string | Event type |
| `height` | integer | Local ledger event height |
| `timestamp` | integer | Unix timestamp |
| `rule_version` | string | Active rule version at event creation |
| `payload` | object | Event-specific payload |
| `hash` | string | Deterministic event hash |

---

## 2. Event Types

Current MVP event types include:

```text
StartingState
Release
Transfer
PublicFund
Rule
```

The active CLI flow primarily uses:

```text
Release
Transfer
```

---

## 3. Event Height

Event height is local ledger height.

Rules:

- first appended event should have height `1`;
- event height should increase over time;
- local validation should flag non-increasing height order;
- height is local MVP state, not public consensus height.

---

## 4. Rule Version

Each event includes a `rule_version`.

The MVP currently uses:

```text
v1
```

Local state validation should flag events whose rule version does not match the ledger rule version.

---

## 5. Event Hash View

The event hash should be calculated from a stable event hash view.

The mutable `hash` field itself must be excluded from the hash input.

Rule:

```text
same event content = same hash
changed event content = changed hash
hash field itself is excluded
```

---

## 6. Release Event Payload

Current release event payload includes:

| Field | Description |
|---|---|
| `interval` | Local release simulation interval |
| `recipient` | Local MVP recipient address |
| `actual_release` | Human-readable PRM amount |
| `public_proof_fund` | Human-readable Public Proof Fund allocation |
| `proof_contributor_reward` | Human-readable contributor reward |
| `actual_release_proof` | Integer proof units released |
| `public_proof_fund_proof` | Integer proof units allocated to Public Proof Fund |
| `proof_contributor_reward_proof` | Integer proof units allocated to contributor |
| `mvp_scope` | Local MVP scope marker |

---

## 7. Transfer Event Payload

Current transfer event payload includes:

| Field | Description |
|---|---|
| `from` | Sender local MVP address |
| `to` | Receiver local MVP address |
| `amount` | Human-readable PRM amount |
| `amount_proof` | Integer proof units transferred |
| `fee` | Human-readable fee amount |
| `fee_proof` | Integer proof units fee |
| `signature` | Signature where applicable |
| `public_key` | Public key where applicable |
| `mvp_scope` | Local transfer event marker |

---

## 8. Known MVP Limitations

Current event schema limitations:

- JSON serialization is MVP-level and should be externally reviewed before public consensus use;
- local event files can be manually mutated;
- previous-event chain linkage may need stronger future design;
- timestamps are local MVP timestamps;
- event IDs are not final public network transaction IDs.

---

## 9. Safety Boundary

This specification documents local MVP event semantics only.

It does not define:

- public consensus;
- final transaction format;
- production chain format;
- exchange integration format;
- audited serialization standard.
