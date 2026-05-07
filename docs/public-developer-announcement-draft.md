# Public Developer Announcement Draft

## Draft Status

```text
Draft only. Not yet published.
```

## Title

ProofMoney Core v0.5.0 Developer Release Is Ready for External Review

## Draft

ProofMoney is an open protocol initiative for verifiable money integrity.

The core idea is simple:

> If money cannot be verified, it is only a promise.

We have published ProofMoney Core v0.5.0-developer-release as a local Rust MVP prototype for external developer review.

This release is focused on making the project easier to run, inspect, and review.

## What Developers Can Run Locally

Developers can clone the repository and run:

```bash
cargo build --workspace --all-targets
cargo test --workspace --all-targets
bash scripts/demo-local.sh
```

The local MVP currently includes:

- Starting State Proof;
- Proof of Issuance;
- Proof of Supply;
- Proof of Ownership;
- Proof of Flow;
- Proof of Rule;
- local ledger persistence;
- local wallet testing;
- transfer event validation;
- proof snapshot export;
- static local Proof Explorer prototype;
- developer quickstart;
- security review scope;
- contributor guide.

## Repository

```text
https://github.com/proofmoney-protocol/core
```

## Documentation

```text
https://github.com/proofmoney-protocol/docs
```

## Website

```text
https://proofmoney.org
```

## What This Is Not

This is not a public network.

This is not a token sale.

This is not an airdrop.

This is not a yield product.

This is not an exchange integration.

This is not a production wallet.

This is not an audited release.

Test units, if any, have no monetary value.

## Review Invitation

We welcome review from Rust developers, protocol engineers, cryptography reviewers, security reviewers, and open-source contributors.

The most useful review areas are:

- amount model;
- release curve logic;
- deterministic event hashing;
- ledger state transitions;
- supply verification;
- wallet risk;
- signature verification;
- Proof of Flow;
- proof export structure;
- safety language.

## Contribution Boundary

Please do not propose or add:

- PRM sale functionality;
- claim pages;
- yield language;
- price charts;
- exchange integration;
- trading functionality;
- hidden allocation logic;
- privileged founder allocation;
- production custody claims.

## Final Statement

ProofMoney is experimental and may fail.

Verification reduces trust requirements, but it does not eliminate risk.
