# Proof Export Freshness

## Purpose

Proof exports are local snapshots.

They can become stale after local ledger changes.

v0.8.0 adds freshness metadata to proof exports.

---

## Metadata

Proof exports include:

- export timestamp;
- ledger height at export;
- last event hash at export;
- event count at export;
- local ledger path;
- freshness warning.

---

## Freshness Warning

```text
This proof export is a local snapshot. It can become stale after local ledger changes. Regenerate exports after state changes.
```

---

## Safety Boundary

Freshness metadata does not make exports public consensus or signed audit artifacts.
