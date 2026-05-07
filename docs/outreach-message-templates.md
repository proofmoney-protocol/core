# ProofMoney Outreach Message Templates

## Purpose

Use these templates for targeted technical review outreach.

Do not use these for mass promotion.

---

## Short GitHub Message

Hi, I’m preparing ProofMoney Core for external review.

ProofMoney is an open protocol initiative for verifiable money integrity. The current Core release is a local Rust MVP prototype, not a public network, not a token sale, not an airdrop, not a yield product, and not a production wallet.

The review entry point is here:

https://github.com/proofmoney-protocol/core/blob/main/docs/external-review-index.md

Core repo:

https://github.com/proofmoney-protocol/core

If you have time, the most useful areas to review are amount handling, event hashing, ledger state transitions, wallet risk, signature verification, Proof of Flow, and proof export structure.

Any feedback as GitHub Issues, Discussions replies, or PRs would be appreciated.

---

## Short X / Social DM

Hi, I’m looking for technical review on ProofMoney Core v0.6.0.

It’s a local Rust MVP for verifiable money integrity — not a public network, not a token sale, not an airdrop, not a yield product, and not a production wallet.

Review entry point:

https://github.com/proofmoney-protocol/core/blob/main/docs/external-review-index.md

Most useful review areas: Rust structure, amount model, event hashing, ledger transitions, wallet risk, signatures, Proof of Flow, and proof export.

Feedback welcome via GitHub Issues / Discussions / PRs.

---

## Longer Email / Message

Subject:

```text
Request for technical review: ProofMoney Core local Rust MVP
```

Body:

Hi,

I’m preparing ProofMoney Core for external Rust, architecture, and security review.

ProofMoney is an open protocol initiative for verifiable money integrity:

> If money cannot be verified, it is only a promise.

The current release is `v0.6.0-external-review`, a local Rust MVP prototype prepared for review.

It includes:

- local ledger persistence;
- deterministic event hashing;
- supply verification;
- local wallet testing;
- transfer event validation;
- proof export;
- static local Proof Explorer prototype;
- developer quickstart;
- demo script;
- security review scope;
- MVP scope freeze.

Important boundary: this is not a public network, not a token sale, not an airdrop, not a yield product, not an exchange integration, not a claim page, and not a production wallet. Test units have no monetary value.

Review entry point:

https://github.com/proofmoney-protocol/core/blob/main/docs/external-review-index.md

Core repository:

https://github.com/proofmoney-protocol/core

Website:

https://proofmoney.org

The areas where feedback would be most valuable:

- amount model and decimal parsing;
- release curve logic;
- deterministic event hashing;
- ledger state transitions;
- supply verification;
- wallet risk;
- signature verification;
- Proof of Flow;
- proof export structure;
- safety wording.

If you spot anything, please open a GitHub Issue using the External Review Finding template, reply in Discussions, or submit a PR.

Thanks.

---

## Public Reply When Someone Asks “Is this a coin launch?”

No.

This is not a public network launch, token sale, airdrop, yield product, exchange integration, claim page, or investment opportunity.

ProofMoney Core is currently a local Rust MVP prototype prepared for external technical review.

The purpose of this phase is to find issues early, document risks clearly, and improve the integrity of the project before any future public-facing expansion.

---

## Public Reply When Someone Asks “Can PRM be bought?”

No.

ProofMoney does not offer, sell, or solicit the purchase of PRM.

PRM does not guarantee price, liquidity, yield, profit, or exchange access.

The current repository is a local MVP prototype for technical review only.

---

## Public Reply When Someone Reports a Finding

Thank you for the review.

Could you please open this as a GitHub Issue using the **External Review Finding** template?

Issue template:

https://github.com/proofmoney-protocol/core/issues/new/choose

Please include:

- affected file/function;
- severity estimate;
- reproduction steps if possible;
- expected behavior;
- actual behavior;
- risk explanation;
- suggested fix if available.

We will triage it and link it to the review response log.
