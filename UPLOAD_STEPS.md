# ProofMoney Core Rustfmt Restore Pack v0.2.1

## Goal

Restore strict rustfmt check in GitHub Actions and close Issue 7 if CI passes.

## Target Repository

```text
https://github.com/proofmoney-protocol/core
```

## Files to Upload

Upload and overwrite these files:

```text
.github/workflows/rust-ci.yml
crates/proofmoney-cli/src/commands/verify.rs
CORE_V0.2.1_RUSTFMT_RESTORE.md
UPLOAD_STEPS.md
```

## Commit Message

```text
ci: restore strict rustfmt check
```

## After Commit

Open:

```text
https://github.com/proofmoney-protocol/core/actions
```

Wait for Rust CI.

If the CI passes:

1. close Issue 7;
2. close Issue 8 if not already closed;
3. close the `v0.2.0-local-ledger` milestone if all related issues are closed.

If the CI fails:

1. open the failed `Check formatting` step;
2. copy the full rustfmt diff;
3. send it for another patch.
