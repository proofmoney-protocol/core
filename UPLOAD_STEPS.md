# ProofMoney Core v1.2.0 Testnet Node API Pack

## Goal

Upload this pack to the `core` repository to implement the v1.2.0 testnet node/API skeleton milestone.

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
core: add testnet node API skeleton v1.2.0
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
v1.2.0-testnet-node-api
```

Do not close Issue 8 until the v1.2.0 report is published to the docs repository.
