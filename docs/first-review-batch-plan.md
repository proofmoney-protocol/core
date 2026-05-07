# ProofMoney First Review Batch Plan

## Purpose

This plan defines the first batch of external reviewers to contact.

The goal is not publicity.

The goal is technical feedback.

---

## Batch Size

```text
5-10 reviewers
```

---

## Reviewer Mix

Recommended first batch:

| Type | Count | Purpose |
|---|---:|---|
| Rust engineer | 2-3 | Code structure, error handling, tests |
| Security reviewer | 1-2 | Wallet risk, signature verification, unsafe assumptions |
| Protocol engineer | 1-2 | Issuance, supply, hashing, state transitions |
| Documentation reviewer | 1 | Safety wording, onboarding clarity |
| Open-source maintainer | 1 | Contribution flow, templates, project hygiene |

---

## Priority Review Areas

Ask each reviewer to focus on a narrow area.

Do not ask everyone to review everything.

## Rust engineer focus

- workspace structure;
- CLI flow;
- error handling;
- tests;
- rustfmt / CI;
- JSON serialization.

## Security reviewer focus

- wallet warning clarity;
- private key exposure risk;
- signature verification;
- misleading claims;
- unsafe assumptions.

## Protocol engineer focus

- amount model;
- release curve;
- event hashing;
- ledger transitions;
- supply verification;
- balance validation.

## Documentation reviewer focus

- quickstart clarity;
- review index clarity;
- safety boundaries;
- public announcement wording;
- contributor rules.

---

## Outreach Tracking

Use:

```text
docs/reviewer-tracking-sheet.md
```

Track:

- reviewer handle;
- area;
- status;
- date contacted;
- feedback link;
- notes.

---

## Success Criteria

The first review batch is successful if any of the following happens:

- 3 meaningful findings;
- 1 high-severity issue;
- 1 useful PR;
- 1 structured review note;
- 1 reviewer says the project is clear enough to review.

---

## Next Step After Success

Create:

```text
v0.7.0-review-response
```

Then:

- classify findings;
- create follow-up issues;
- fix priority issues;
- publish review response log;
- update docs and safety wording.

---

## Safety Boundary

Do not convert technical review into promotional marketing.

Do not mention sale, airdrop, yield, exchange, price, or investment opportunity.
