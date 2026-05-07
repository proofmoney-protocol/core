# ProofMoney Core v0.3.1 Compile Fix Upload Steps

## Target Repository

```text
https://github.com/proofmoney-protocol/core
```

## Files to Upload

Upload and overwrite:

```text
crates/proofmoney-types/src/event.rs
crates/proofmoney-proof/Cargo.toml
CORE_V0.3.1_COMPILE_FIX.md
UPLOAD_STEPS.md
```

## Commit Message

```text
core: fix v0.3 ownership flow compile errors
```

## After Commit

GitHub Actions will run again automatically.

If CI fails again, open the failed step and send the final error log.
