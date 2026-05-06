# ProofMoney Core Repository Settings Checklist

Repository:

```text
https://github.com/proofmoney-protocol/core
```

## About Section

Set repository description:

```text
Rust local MVP prototype for ProofMoney.
```

Set website:

```text
https://proofmoney.org
```

Set topics:

```text
rust
protocol
cryptography
ledger
verification
proofs
cli
money-integrity
proofmoney
```

## Features

Recommended:

- Issues: On
- Projects: Off for now
- Wiki: Off
- Discussions: Off for now
- Sponsorships: Off

## Branch Protection

After CI passes once, add branch protection for `main`:

- Require a pull request before merging
- Require status checks to pass before merging
- Require Rust CI
- Do not allow force pushes
- Do not allow deletions

## GitHub Actions

Confirm Actions tab shows:

```text
Rust CI
```

Expected checks:

- cargo fmt
- cargo build
- cargo test
- CLI smoke tests

## Safety

Do not add:

- Buy PRM
- Claim PRM
- Airdrop
- Presale
- Yield
- Price
- Exchange
- Countdown
- Trading links
