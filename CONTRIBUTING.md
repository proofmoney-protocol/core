# Contributing to ProofMoney Core

ProofMoney Core is the Rust local MVP prototype for verifiable money integrity.

Before contributing, read:

- `README.md`
- `docs/mvp-limitations.md`
- ProofMoney docs repository: https://github.com/proofmoney-protocol/docs

## Development Rules

- Do not use floating-point numbers for monetary amounts.
- Use integer smallest units.
- Add tests for valid and invalid cases.
- Keep the MVP local.
- Do not add token sale, airdrop, yield, exchange, or trading features.
- Do not overclaim security.

## Required Local Checks

Before opening a pull request, run:

```bash
cargo fmt --all
cargo build --workspace --all-targets
cargo test --workspace --all-targets
```

Run CLI smoke tests:

```bash
cargo run -p proofmoney-cli -- starting-state
cargo run -p proofmoney-cli -- simulate-release --interval 1
cargo run -p proofmoney-cli -- verify-supply
cargo run -p proofmoney-cli -- verify-rule
cargo run -p proofmoney-cli -- integrity-status
```

## Pull Request Expectations

A PR should explain:

- what changed;
- which proof area is affected;
- which commands were tested;
- whether risk or user expectations changed.

## Forbidden Changes

Do not add:

- PRM sale language;
- airdrop claim logic;
- presale logic;
- yield logic;
- exchange integration;
- price or trading features;
- test-to-main conversion promises.

If money cannot be verified, it is only a promise.
