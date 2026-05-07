# ProofMoney Core v0.8.0 CI Fix: Serialize CLI Integration Tests

## Problem

CI still failed in:

```text
crates/proofmoney-cli/tests/cli_integration.rs
```

Error:

```text
Error: event hash is invalid
```

Likely cause:

Rust integration tests run concurrently by default. Multiple CLI tests perform local state operations such as reset, wallet creation, and ledger updates. If any underlying local path is shared or not fully isolated by the OS/path library, tests can collide.

## Fix

This patch:

- serializes CLI integration tests with a global mutex;
- adds XDG path isolation;
- improves failure output by printing the exact failed CLI args.

## File to Upload

Upload and overwrite:

```text
crates/proofmoney-cli/tests/cli_integration.rs
```

## Commit Message

```text
test: serialize cli integration tests
```

## After Commit

Run GitHub Actions again.

If CI still fails, the new log will show the exact CLI command that failed.
Send the final 30 lines if it fails again.
