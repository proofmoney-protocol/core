# ProofMoney Core v0.8.0 CI Fix: reset-ledger

## Problem

CI failed in:

```text
crates/proofmoney-cli/tests/cli_integration.rs
```

Error:

```text
Error: event hash is invalid
```

The failure happened after `reset-ledger`, when the integration flow tried to append a release event.

## Fix

This patch changes `reset-ledger` to remove the local ledger file and reinitialize it through the existing `load_or_init_ledger("v1")` storage path.

That keeps reset behavior aligned with the same initialization path used by the rest of the MVP.

## File to Upload

Upload and overwrite:

```text
crates/proofmoney-cli/src/commands/local_state.rs
```

## Commit Message

```text
fix: align reset ledger with storage initialization
```

## After Commit

Run GitHub Actions again.

If CI still fails, send the final 30 lines of the failing step.
