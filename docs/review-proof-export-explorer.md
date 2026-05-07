# Proof Export and Local Explorer Review Notes

## Review Status

```text
Prepared for external review
```

This document records current self-review notes and areas for external reviewers.

It is not an audit report.

---

## Current Export Scope

The MVP supports local proof export to:

```text
~/.proofmoney/export/
```

Expected files:

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

## Current Explorer Scope

The MVP supports preparing a static local Proof Explorer under:

```text
~/.proofmoney/explorer/
```

The explorer is a local static proof viewer only.

It is not a hosted public API.

It is not a public block explorer.

It is not a trading interface.

It is not a claim page.

---

## Areas Reviewed Internally

## 1. Proof Snapshot

Reviewers should inspect whether the snapshot includes enough context:

- project;
- snapshot version;
- generated timestamp;
- safety notice;
- starting state;
- ledger status;
- supply proof;
- rule proof;
- integrity status;
- release events;
- transfer events.

## 2. JSON Output Shape

Reviewers should inspect whether outputs are consistent and understandable.

Expected common fields:

- status;
- proof_type;
- rule_version;
- data;
- summary;
- safety_notice.

## 3. Release Event Listing

Reviewers should inspect whether listed release events include enough verification context.

## 4. Transfer Event Listing

Reviewers should inspect whether listed transfer events include hash validity and flow proof status.

## 5. Safety Language

Reviewers should inspect whether the explorer language clearly states local-only MVP status.

---

## Known Risks and Questions

- The explorer loads local files and may hit browser local-file restrictions.
- Export files may be stale after ledger changes.
- Exported JSON is not signed as an external artifact.
- The explorer is not production-ready.
- Public deployment would require a separate security and UX review.

---

## Suggested External Review Tasks

1. Review proof snapshot structure.
2. Review JSON consistency.
3. Review local file generation.
4. Review explorer safety copy.
5. Review misleading wording.
6. Recommend future public explorer requirements if needed.

---

## Safety Statement

The local explorer is not a public block explorer, hosted API, trading interface, or claim page.
