# ProofMoney Core Rustfmt Fix v0.1.1

## What this fixes

The first CI failure was caused by `cargo fmt --all -- --check`.

This patch only applies rustfmt formatting changes.

## Target repository

```text
https://github.com/proofmoney-protocol/core
```

## Files to upload and overwrite

```text
crates/proofmoney-release/src/release_event.rs
crates/proofmoney-release/src/validation.rs
crates/proofmoney-types/src/amount.rs
crates/proofmoney-wallet/src/signing.rs
```

## Upload steps

1. Open the `core` repository.
2. Click `Add file → Upload files`.
3. Upload the extracted files into the repository root.
4. Confirm GitHub shows these files as changed, not added inside a parent folder.
5. Commit with:

```text
core: apply rustfmt fix
```

## After commit

GitHub Actions will run again automatically.

If it fails again, open the failed step and send the new error log.
