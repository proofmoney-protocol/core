# ProofMoney Core v0.8.0 CLI Integration Hardening Pack

## Goal

Upload this pack to the `core` repository to implement the v0.8.0 CLI integration hardening milestone.

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
core: harden cli integration v0.8.0
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
v0.8.0-cli-integration-hardening
```

Do not close Issue 8 until the v0.8.0 report is published to the docs repository.
