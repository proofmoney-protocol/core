# Reviewer Follow-Up Templates

## After Reviewer Agrees

Thank you. The best starting point is the external review index:

https://github.com/proofmoney-protocol/core/blob/main/docs/external-review-index.md

If you prefer a focused review, the most useful areas are:

- amount model and decimal parsing;
- event hashing;
- ledger state transitions;
- wallet risk;
- signature verification;
- Proof of Flow;
- proof export structure.

Any finding can be opened here:

https://github.com/proofmoney-protocol/core/issues/new/choose

Please use the External Review Finding template if possible.

---

## After Reviewer Sends Informal Feedback

Thank you, this is useful.

Could you open it as a GitHub Issue so it can be tracked and triaged properly?

Issue templates:

https://github.com/proofmoney-protocol/core/issues/new/choose

Useful details:

- affected file/function;
- severity estimate;
- reproduction steps;
- expected vs actual behavior;
- why it matters;
- suggested fix if available.

---

## After Reviewer Reports a Possible Security Issue

Thank you for flagging this.

If the details are sensitive or exploit-ready, please avoid posting full details publicly.

Could you share a minimal description first, including:

- affected area;
- estimated severity;
- whether it is reproducible;
- whether it affects local MVP assumptions or user safety wording.

We will triage it carefully.

---

## If Reviewer Says It Is Too Early

That is fair. This is intentionally early.

The current goal is not public launch, but to catch architectural, correctness, wallet-risk, and wording issues before further expansion.

Even a short note on what is unclear or risky would be useful.

---

## If Reviewer Asks About Incentives

At this stage, this is an open technical review request, not a bounty program.

There is no token reward, airdrop, yield, allocation, or investment benefit.

If a formal bounty or paid review program is created in the future, it will be documented separately with clear terms.

---

## If Reviewer Asks Whether This Is Audited

No.

The current release is prepared for external review but has not completed an external audit.

The review phase exists to find problems before any public-facing expansion.
