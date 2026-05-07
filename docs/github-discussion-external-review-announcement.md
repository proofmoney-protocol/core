# ProofMoney Core v0.6.0 is ready for external review

ProofMoney is an open protocol initiative for **verifiable money integrity**.

> If money cannot be verified, it is only a promise.

We have published **ProofMoney Core v0.6.0-external-review**, a local Rust MVP prototype prepared for external Rust, architecture, and security review.

This release is focused on making the project easier to inspect, question, challenge, and improve before any future public-facing expansion.

---

## What is available now

The current local MVP includes:

- Starting State Proof
- Proof of Issuance
- Proof of Supply
- Proof of Ownership
- Proof of Flow
- Proof of Rule
- local ledger persistence
- deterministic event hashing
- local wallet testing
- transfer event validation
- local balance tracking
- proof snapshot export
- static local Proof Explorer prototype
- developer quickstart
- local demo script
- sample proof fixtures
- architecture overview
- security review scope
- contributor guide
- MVP scope freeze
- external review notes

---

## Repository

Core repository:

https://github.com/proofmoney-protocol/core

Documentation repository:

https://github.com/proofmoney-protocol/docs

Website:

https://proofmoney.org

Current release:

https://github.com/proofmoney-protocol/core/releases/tag/v0.6.0-external-review

---

## How to run locally

```bash
git clone https://github.com/proofmoney-protocol/core.git
cd core

cargo build --workspace --all-targets
cargo test --workspace --all-targets
bash scripts/demo-local.sh
```

The local demo runs the MVP flow, including build, tests, proof commands, local ledger state, wallet test flow, proof export, and local explorer preparation.

---

## Review entry points

If you want to review the project, these are the best starting points:

- `docs/external-review-index.md`
- `docs/mvp-scope-freeze.md`
- `docs/security-review-scope.md`
- `docs/architecture-overview.md`
- `docs/developer-quickstart.md`
- `CONTRIBUTING.md`

The most useful review areas are:

- amount model and decimal parsing
- release curve logic
- deterministic event hashing
- ledger state transitions
- supply verification
- wallet risk
- signature verification
- Proof of Flow
- proof export structure
- static local explorer output
- safety language and misleading wording

---

## What this is not

This is not a public network.

This is not a token sale.

This is not an airdrop.

This is not a yield product.

This is not an exchange integration.

This is not a claim page.

This is not a production wallet.

This is not an audited release.

Test units, if any, have no monetary value.

ProofMoney does not offer, sell, or solicit the purchase of PRM. PRM does not guarantee price, liquidity, yield, profit, or exchange access.

---

## Contribution boundary

Please do not propose or add:

- PRM sale functionality
- claim pages
- yield language
- price charts
- exchange integration
- trading functionality
- hidden allocation logic
- privileged founder allocation
- production custody claims

The current goal is review, not promotion.

---

## Feedback welcome

We welcome feedback from:

- Rust engineers
- protocol engineers
- cryptography reviewers
- security reviewers
- open-source maintainers
- documentation reviewers

Useful feedback can be posted as:

- GitHub Issues
- GitHub Discussions replies
- pull requests
- review notes referencing specific files or functions

ProofMoney is experimental and may fail. The purpose of this review phase is to find problems early, document risks clearly, and improve the integrity of the project before moving further.

If money cannot be verified, it is only a promise.
