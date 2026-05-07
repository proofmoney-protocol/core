# ProofMoney External Review Launch Checklist

## Purpose

This checklist helps launch the ProofMoney external review phase in a controlled, non-promotional way.

This is not a token launch.

This is not a public network launch.

This is not a fundraising campaign.

This is a developer and security review phase.

---

## 1. Repository Readiness

Confirm:

- [ ] Core repository is public.
- [ ] Docs repository is public.
- [ ] Website is live.
- [ ] Latest Core release is `v0.6.0-external-review`.
- [ ] CI is green.
- [ ] `SECURITY.md` exists.
- [ ] `CONTRIBUTING.md` exists.
- [ ] Issue templates are visible.
- [ ] PR template is visible.
- [ ] `docs/external-review-index.md` exists.
- [ ] `docs/security-review-scope.md` exists.
- [ ] `docs/mvp-scope-freeze.md` exists.

---

## 2. Discussion Announcement

Create a GitHub Discussion in the `core` repository.

Suggested title:

```text
ProofMoney Core v0.6.0 is ready for external review
```

Use the announcement in:

```text
docs/github-discussion-external-review-announcement.md
```

Recommended category:

```text
Announcements
```

If unavailable, use:

```text
General
```

---

## 3. GitHub Labels

Create or verify the labels listed in:

```text
docs/github-review-labels.md
```

Important labels:

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
follow-up
```

---

## 4. Reviewer Outreach

Use:

```text
docs/reviewer-outreach-message.md
```

Recommended reviewer types:

- Rust engineers
- protocol engineers
- cryptography reviewers
- security reviewers
- open-source maintainers
- documentation reviewers

Initial target count:

```text
5-10 reviewers
```

Do not mass promote.

Start with targeted technical review.

---

## 5. What Not To Say

Do not say:

- token launch
- public network launch
- airdrop
- presale
- investment opportunity
- yield
- early access to coins
- exchange listing
- price potential
- claim page
- mainnet soon

Use:

- local Rust MVP
- external review
- verifiable money integrity
- developer feedback
- security review scope
- experimental prototype
- no monetary value

---

## 6. Intake Flow

When feedback arrives:

1. Ask reviewer to open a GitHub Issue if they have not.
2. Apply labels.
3. Classify severity.
4. Ask for reproduction steps if missing.
5. Link to affected file/function.
6. Do not immediately argue.
7. Create follow-up issue if needed.
8. Add accepted finding to future review response log.

---

## 7. When to Start v0.7.0

Do not start `v0.7.0-review-response` until at least one of the following is true:

- at least 3 meaningful external findings received;
- one high-severity issue discovered;
- reviewer feedback requires code or docs changes;
- a security reviewer provides structured feedback.

---

## Safety Statement

ProofMoney Core remains a local MVP prototype.

External review does not mean audit completion.

External review does not mean public network readiness.

Test units, if any, have no monetary value.
