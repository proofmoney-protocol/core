# GitHub Review Labels

## Purpose

These labels support external review intake, triage, and follow-up tracking.

Create or verify these labels in the `core` repository.

Repository:

```text
https://github.com/proofmoney-protocol/core
```

Go to:

```text
Issues → Labels
```

---

## Core Review Labels

| Label | Color | Description |
|---|---:|---|
| `review` | `#5319e7` | External review finding or review-related issue |
| `needs-triage` | `#fbca04` | Needs maintainer classification |
| `security` | `#d73a4a` | Security-sensitive issue or review note |
| `correctness` | `#d93f0b` | Logic correctness concern |
| `documentation` | `#0075ca` | Documentation or wording issue |
| `wallet-risk` | `#b60205` | Wallet, key handling, or ownership proof risk |
| `ledger` | `#1d76db` | Ledger state transition or event application |
| `proof-flow` | `#0e8a16` | Proof of Flow or transfer validation |
| `proof-export` | `#006b75` | Proof JSON export or local explorer |
| `amount-model` | `#c5def5` | Amount parsing, unit model, or supply boundary |
| `release-curve` | `#bfdadc` | Release curve or issuance logic |
| `follow-up` | `#cfd3d7` | Follow-up issue from review |
| `accepted-finding` | `#0e8a16` | Finding accepted and scheduled for action |
| `needs-reproduction` | `#fef2c0` | Reproduction steps needed |
| `wontfix-current-mvp` | `#ffffff` | Valid concern but outside current MVP scope |

---

## Severity Labels

| Label | Color | Description |
|---|---:|---|
| `severity-critical` | `#b60205` | Severe issue that may invalidate core MVP claims |
| `severity-high` | `#d93f0b` | Serious issue requiring priority response |
| `severity-medium` | `#fbca04` | Meaningful issue requiring follow-up |
| `severity-low` | `#c2e0c6` | Minor issue or cleanup |
| `severity-informational` | `#bfd4f2` | Informational note or design question |

---

## Recommended Triage Rules

## Critical

Use when a finding could invalidate core proof claims.

Examples:

- supply verification can be bypassed;
- event hash validation fails open;
- invalid transfer accepted;
- wallet risk is materially misrepresented.

## High

Use when a serious correctness or safety issue exists.

Examples:

- insufficient balance validation gap;
- signature verification mismatch;
- release calculation bug;
- misleading public-facing safety statement.

## Medium

Use when the issue affects reviewability or correctness clarity.

Examples:

- ambiguous proof output;
- missing test;
- unclear error handling;
- incomplete docs.

## Low

Use for cleanup, wording, or minor non-blocking improvements.

## Informational

Use for design discussion, future recommendation, or reviewer note.

---

## Notes

Labels do not need to be perfect on the first pass.

Initial triage can change after investigation.

Do not mark a finding as fixed until the fix is merged and CI is green.
