# ProofMoney Core v0.5.0 Developer Release Pack

## Goal

Upload this pack to the `core` repository to implement the v0.5.0 developer release milestone.

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
docs: prepare developer release v0.5.0
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
