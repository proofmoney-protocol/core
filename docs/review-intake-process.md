# ProofMoney Review Intake Process

## Purpose

This document explains how external review feedback should be collected, triaged, and converted into follow-up issues.

This is for the v0.6.0 external review phase.

---

## Feedback Channels

Preferred feedback channels:

1. GitHub Issues
2. GitHub Discussions
3. Pull requests
4. Review notes referencing specific files or functions

---

## Issue Templates

The repository includes templates for:

- External Review Finding
- Documentation Feedback

Use **External Review Finding** for:

- correctness issues;
- protocol logic concerns;
- security concerns;
- state transition concerns;
- wallet risk;
- signature verification;
- Proof of Flow;
- proof export concerns.

Use **Documentation Feedback** for:

- unclear docs;
- misleading wording;
- missing safety language;
- onboarding confusion;
- developer quickstart issues.

---

## Severity Classification

Initial severity levels:

## Critical

A finding that could invalidate core MVP proof logic or create a severe false claim.

Examples:

- supply verification can be bypassed;
- event hash can be forged or ignored;
- invalid transfer can be accepted;
- private key handling is dangerously misrepresented.

## High

A serious correctness or safety issue that should be fixed before broader review.

Examples:

- insufficient balance logic has a gap;
- signature verification mismatch;
- release calculation bug;
- misleading public-facing safety language.

## Medium

A meaningful issue that affects reviewability, correctness clarity, or developer confidence.

Examples:

- unclear proof output;
- missing negative tests;
- ambiguous error handling;
- incomplete documentation.

## Low

Minor issue, wording improvement, or non-blocking implementation cleanup.

## Informational

Design question, future recommendation, or reviewer note.

---

## Triage Process

For each finding:

1. Confirm the affected area.
2. Confirm whether it is reproducible.
3. Assign severity.
4. Decide whether it is in current MVP scope.
5. Add or update labels.
6. Create a follow-up issue if needed.
7. Link the finding to the future review response log.

---

## Labels

Recommended labels:

```text
review
needs-triage
security
correctness
documentation
wallet-risk
ledger
proof-flow
proof-export
amount-model
good-first-review
follow-up
```

---

## Response Policy

Do not dismiss review findings quickly.

For each meaningful finding, respond with:

- acknowledgement;
- current understanding;
- whether it is accepted, rejected, or needs more investigation;
- next action;
- linked follow-up issue where applicable.

---

## Safety Boundary

Review intake does not mean ProofMoney has been audited.

Review intake does not mean ProofMoney is production-ready.

ProofMoney remains a local MVP prototype.
