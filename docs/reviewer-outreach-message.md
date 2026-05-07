# Reviewer Outreach Message

## Short Message

Hi, I’m preparing ProofMoney Core for external review.

ProofMoney is an open protocol initiative for verifiable money integrity. The current Core release is a local Rust MVP prototype, not a public network, not a token sale, not an airdrop, not a yield product, and not a production wallet.

The review entry point is here:

https://github.com/proofmoney-protocol/core/blob/main/docs/external-review-index.md

Core repository:

https://github.com/proofmoney-protocol/core

Website:

https://proofmoney.org

If you have time, the most useful areas to review are amount handling, event hashing, ledger state transitions, wallet risk, signature verification, Proof of Flow, and proof export structure.

Any feedback is welcome as GitHub Issues, Discussions replies, or PRs.

## Longer Message

Hi, I’m looking for external review on ProofMoney Core v0.6.0-external-review.

ProofMoney is an open protocol initiative for verifiable money integrity:

> If money cannot be verified, it is only a promise.

The current release is a local Rust MVP prototype prepared for review. It includes local ledger persistence, deterministic event hashing, supply verification, local wallet testing, transfer validation, proof export, a static local explorer prototype, developer quickstart, demo script, security review scope, and an MVP scope freeze.

Important boundary: this is not a public network, not a token sale, not an airdrop, not a yield product, not an exchange integration, not a claim page, and not a production wallet. Test units have no monetary value.

Review entry point:

https://github.com/proofmoney-protocol/core/blob/main/docs/external-review-index.md

Core repository:

https://github.com/proofmoney-protocol/core

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
