# ProofMoney Core v0.3.0 Ownership and Flow Pack

## Goal

Upload this pack to the `core` repository to implement the v0.3.0 ownership and flow milestone.

Repository:

```text
https://github.com/proofmoney-protocol/core
```

## Upload

Upload all files in this pack into the repository root.

Important:

Do not upload the parent folder itself.

## Commit Message

```text
core: implement ownership and flow milestone v0.3.0
```

## After Upload

GitHub Actions should run automatically.

Expected commands covered by CI:

```bash
cargo fmt --all -- --check
cargo build --workspace --all-targets
cargo test --workspace --all-targets
cargo run -p proofmoney-cli -- starting-state
cargo run -p proofmoney-cli -- starting-state --json
cargo run -p proofmoney-cli -- simulate-release --interval 1
cargo run -p proofmoney-cli -- simulate-release --interval 1 --append
cargo run -p proofmoney-cli -- ledger-status
cargo run -p proofmoney-cli -- verify-supply
cargo run -p proofmoney-cli -- verify-rule
cargo run -p proofmoney-cli -- integrity-status
cargo run -p proofmoney-cli -- create-wallet
cargo run -p proofmoney-cli -- new-address
```

If CI fails, open the failed step and send the final error log.
