# ProofMoney Reviewer Outreach Playbook

## Purpose

This playbook guides the first targeted external review outreach for ProofMoney Core.

This is not mass promotion.

This is not a token launch.

This is not a public network launch.

This is a targeted technical review phase.

---

## Goal

Reach out to 5-10 technical reviewers across:

- Rust engineering;
- protocol engineering;
- cryptography review;
- security review;
- open-source maintenance;
- documentation review.

The goal is to receive meaningful feedback, not attention.

---

## Current Review Entry Points

Core repository:

```text
https://github.com/proofmoney-protocol/core
```

External Review Index:

```text
https://github.com/proofmoney-protocol/core/blob/main/docs/external-review-index.md
```

Security Review Scope:

```text
https://github.com/proofmoney-protocol/core/blob/main/docs/security-review-scope.md
```

Website:

```text
https://proofmoney.org
```

Current release:

```text
https://github.com/proofmoney-protocol/core/releases/tag/v0.6.0-external-review
```

---

## Reviewer Types

## 1. Rust Engineer

Best for:

- workspace structure;
- error handling;
- amount parsing;
- serialization;
- tests;
- CLI structure.

## 2. Security Reviewer

Best for:

- wallet risk;
- signature verification;
- unsafe assumptions;
- misleading safety wording;
- local storage risk.

## 3. Protocol Engineer

Best for:

- issuance logic;
- supply calculation;
- event hashing;
- ledger state transition;
- balance validation.

## 4. Documentation Reviewer

Best for:

- onboarding clarity;
- safety language;
- public claims;
- review flow clarity.

---

## Outreach Rules

Use precise, non-hype language.

Do not say:

- coin launch;
- token opportunity;
- early investor;
- airdrop;
- yield;
- exchange;
- mainnet soon;
- price potential.

Say:

- local Rust MVP;
- external review;
- verifiable money integrity;
- developer feedback;
- security review scope;
- no monetary value;
- experimental prototype.

---

## First Round Target

Initial target count:

```text
5-10 reviewers
```

Expected conversion:

```text
1-3 meaningful responses
```

That is enough to start `v0.7.0-review-response`.

---

## When Feedback Arrives

1. Ask reviewer to open a GitHub Issue if not already done.
2. Apply `review` and `needs-triage` labels.
3. Add severity label.
4. Ask for reproduction steps if missing.
5. Record the finding in `docs/review-response-log-template.md`.
6. Create follow-up issues where needed.
7. Do not start feature work until high-priority review findings are understood.

---

## Safety Boundary

External review does not mean audit completion.

ProofMoney Core remains a local MVP prototype.

Test units, if any, have no monetary value.
