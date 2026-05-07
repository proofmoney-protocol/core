# ProofMoney Core v0.7.0 Internal Review Hardening Pack

## Goal

Upload this pack to the `core` repository to implement the v0.7.0 internal review hardening milestone.

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
core: add internal review hardening v0.7.0
```

## After Upload

GitHub Actions should run automatically.

Expected checks:

```bash
cargo fmt --all -- --check
cargo build --workspace --all-targets
cargo test --workspace --all-targets
bash scripts/demo-local.sh
```

If CI fails, open the failed step and send the final error log.

## After CI Passes

You can close Issues 1-7 under:

```text
v0.7.0-internal-review-hardening
```

Do not close Issue 8 until the v0.7.0 report is published to the docs repository.
