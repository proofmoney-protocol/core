# ProofMoney Core v0.8.0 CI Fix: release event hash

## Problem

CI fails on:

```text
simulate-release --interval 1 --append
```

Error:

```text
Error: event hash is invalid
```

## Fix

This patch rewrites the CLI release simulation to create the release event directly and hash it using the same ledger hashing function used by `apply_event`:

```text
proofmoney_ledger::hash_event
```

That removes the mismatch between release-event construction and ledger-event verification.

## File to Upload

Upload and overwrite:

```text
crates/proofmoney-cli/src/commands/release.rs
```

## Commit Message

```text
fix: hash simulated release events with ledger hasher
```

## After Commit

Run GitHub Actions again.

If CI still fails, send the final 30 lines of the failing step.
