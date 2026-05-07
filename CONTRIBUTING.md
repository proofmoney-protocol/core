# Contributing to ProofMoney Core

ProofMoney Core is a local MVP prototype for verifiable money integrity.

> If money cannot be verified, it is only a promise.

Thank you for considering a contribution.

---

## Project Purpose

ProofMoney explores a verification-first standard for money-like digital systems.

The current repository is focused on local MVP proof logic:

- Starting State Proof
- Proof of Issuance
- Proof of Supply
- Proof of Ownership
- Proof of Flow
- Proof of Rule
- local proof export
- local proof explorer prototype

---

## Safety Boundary

Do not add or promote:

- token sale;
- presale;
- private allocation;
- airdrop claim;
- yield product;
- price prediction;
- exchange listing;
- trading interface;
- investment return language;
- test-to-main conversion promises.

ProofMoney Core is not production wallet software.

Do not present MVP wallet functionality as safe custody.

---

## Local Setup

```bash
git clone https://github.com/proofmoney-protocol/core.git
cd core
cargo build --workspace --all-targets
cargo test --workspace --all-targets
```

Run the local demo:

```bash
bash scripts/demo-local.sh
```

---

## Code Style

Before opening a pull request, run:

```bash
cargo fmt --all
cargo build --workspace --all-targets
cargo test --workspace --all-targets
```

CI requires:

```bash
cargo fmt --all -- --check
```

---

## Pull Request Expectations

A PR should explain:

- what changed;
- which proof area is affected;
- whether local ledger state changes;
- whether wallet behavior changes;
- which tests were run;
- whether risk or user expectations changed.

---

## Issue Workflow

Use GitHub Issues for:

- feature requests;
- bug reports;
- proof logic concerns;
- documentation updates;
- security review notes.

For larger changes, create or reference a milestone.

---

## Forbidden Changes

Do not add:

- PRM sale functionality;
- claim pages;
- yield language;
- price charts;
- exchange integration;
- trading functionality;
- hidden allocation logic;
- privileged founder allocation;
- production custody claims.

---

## Security Review

Security-related concerns should be handled carefully.

Do not publicly post exploit-ready details if a vulnerability could put users at risk.

Current security review scope is documented in:

```text
docs/security-review-scope.md
```

---

## Final Statement

Verification reduces trust requirements, but it does not eliminate risk.

ProofMoney is experimental and may fail.
