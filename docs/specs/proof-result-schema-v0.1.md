# ProofMoney Proof Result Schema Specification v0.1

## Status

```text
Spec version: v0.1
Scope: Local MVP proof result semantics
```

This specification documents the current local MVP proof result schema.

Proof results are local MVP verification outputs.

They do not imply external audit or public settlement finality.

---

## 1. Common Proof Result Structure

Current MVP proof results use a structure similar to:

```json
{
  "proof_type": "proof_of_supply",
  "status": "Valid",
  "rule_version": "v1",
  "data": {},
  "summary": "Supply is valid."
}
```

Common fields:

| Field | Type | Description |
|---|---|---|
| `proof_type` | string | Proof category |
| `status` | string | Proof status |
| `rule_version` | string | Rule version used |
| `data` | object | Proof-specific data |
| `summary` | string | Human-readable summary |

Some exported proof files also include:

| Field | Type | Description |
|---|---|---|
| `export_metadata` | object | Local export freshness metadata |
| `safety_notice` | string | Safety boundary statement |

---

## 2. Proof Status

Current status values include:

```text
Valid
Invalid
Warning
```

Status meanings:

- `Valid`: local MVP verification passed;
- `Invalid`: local MVP verification failed;
- `Warning`: local MVP verification produced cautionary status.

These statuses are local verification results only.

They are not external audit results.

---

## 3. Starting State Proof

Starting State Proof verifies initial local MVP conditions such as:

- zero initial supply;
- no founder allocation;
- no private sale allocation;
- no hidden address allocation;
- no Public Proof Fund preload.

---

## 4. Proof of Supply

Proof of Supply verifies:

- stored supply;
- computed supply;
- whether stored and computed supply match;
- maximum supply boundary status.

---

## 5. Proof of Rule

Proof of Rule verifies:

- active rule version;
- rule compatibility;
- local MVP rule assumptions.

---

## 6. Proof of Ownership

Proof of Ownership verifies local key-control semantics.

It may check:

- address;
- message;
- signature;
- public key;
- ownership verification result.

It does not imply monetary ownership, production custody, or public network control.

---

## 7. Proof of Flow

Proof of Flow verifies local transfer event validity.

It may check:

- sender;
- receiver;
- amount;
- fee;
- signature presence;
- signature validity;
- sufficient balance;
- event hash validity.

It does not imply public settlement finality.

---

## 8. Integrity Status

Integrity Status provides a local MVP overview across proof areas.

It may summarize:

- Starting State;
- Issuance;
- Supply;
- Ownership;
- Flow;
- Rule;
- Public Proof Fund;
- invalid release detection;
- MVP stage.

---

## 9. Safety Boundary

Proof results are local MVP verification outputs.

They are not:

- external audit reports;
- public consensus proofs;
- production custody guarantees;
- investment guarantees;
- exchange-listing proofs;
- claim eligibility proofs.
