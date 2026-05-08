# ProofMoney Core v1.1.0 Testnet Design Pack

## Goal

Upload this pack to the `core` repository to implement the v1.1.0 public testnet design milestone.

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
docs: add public testnet design v1.1.0
```

## After Upload

GitHub Actions should run automatically.

Expected checks:

```bash
cargo fmt --all -- --check
cargo build --workspace --all-targets
cargo test --workspace --all-targets
bash scripts/demo-local.sh
bash scripts/demo-transfer-local.sh
```

If CI fails, open the failed step and send the final error log.

## After CI Passes

You can close Issues 1-7 under:

```text
v1.1.0-testnet-design
```

Do not close Issue 8 until the v1.1.0 report is published to the docs repository.
