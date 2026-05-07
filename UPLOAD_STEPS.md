# ProofMoney Core v0.8.0 CI Fix v2: reset-ledger

## Problem

CI still failed with:

```text
Error: event hash is invalid
```

The previous patch still reinitialized the ledger immediately after reset.

## Fix

This patch changes `reset-ledger` to only remove the local ledger file.

It does not call `load_or_init_ledger("v1")` during reset.

The next command that actually needs ledger state will initialize through the existing normal path.

## File to Upload

Upload and overwrite:

```text
crates/proofmoney-cli/src/commands/local_state.rs
```

## Commit Message

```text
fix: make reset ledger remove local ledger only
```

## After Commit

Run GitHub Actions again.

If CI still fails, send the final 30 lines of the failing step.
