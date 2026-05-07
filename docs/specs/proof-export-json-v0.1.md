# ProofMoney Proof Export JSON Specification v0.1

## Status

```text
Spec version: v0.1
Scope: Local MVP proof export semantics
```

This specification documents current local MVP proof export JSON structures.

Proof exports are local snapshots.

They are not signed public audit artifacts or public consensus.

---

## 1. Export Directory

Current local proof export directory:

```text
~/.proofmoney/export/
```

Current local explorer data directory:

```text
~/.proofmoney/explorer/data/
```

---

## 2. Exported Files

Expected export files:

```text
starting-state.json
ledger-status.json
supply.json
rule.json
integrity-status.json
release-events.json
transfer-events.json
proof-snapshot.json
```

---

## 3. Proof Snapshot Structure

A proof snapshot includes:

- project name;
- snapshot version;
- generated timestamp;
- export metadata;
- safety notice;
- Starting State proof;
- Ledger Status;
- Supply proof;
- Rule proof;
- Integrity Status;
- Release Events;
- Transfer Events.

---

## 4. Export Metadata

v0.8.0 and later exports include freshness metadata:

```json
{
  "exported_at": "timestamp",
  "ledger_height_at_export": 1,
  "last_event_hash_at_export": "hash",
  "event_count_at_export": 1,
  "local_ledger_path": "~/.proofmoney/ledger.json",
  "freshness_warning": "This proof export is a local snapshot..."
}
```

---

## 5. Freshness Warning

Required warning:

```text
This proof export is a local snapshot. It can become stale after local ledger changes. Regenerate exports after state changes.
```

---

## 6. Individual Proof Files

Individual proof files may include:

- status;
- proof type;
- rule version;
- data;
- summary;
- export metadata;
- safety notice.

---

## 7. Release Events Export

Release events export includes:

- event id;
- height;
- timestamp;
- recipient;
- actual release;
- Public Proof Fund allocation;
- proof contributor reward;
- event hash;
- hash validity;
- proof status.

---

## 8. Transfer Events Export

Transfer events export includes:

- transaction id;
- height;
- timestamp;
- sender;
- receiver;
- amount;
- fee;
- event hash;
- hash validity;
- flow status;
- MVP scope.

---

## 9. Local Explorer Expectations

The local static explorer reads local proof export data.

It is:

- local only;
- static;
- MVP-level;
- not hosted by default;
- not a public block explorer;
- not a trading interface;
- not a claim page.

---

## 10. Safety Boundary

Proof exports are local MVP snapshots.

They are not:

- signed audit artifacts;
- public consensus;
- exchange data;
- legal accounting records;
- investment disclosures;
- airdrop claim proofs;
- production explorer API responses.
