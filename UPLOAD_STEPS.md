# ProofMoney Core CLI Verify Name Fix v0.1.3

## What this fixes

CI failed because `crates/proofmoney-cli/src/commands/verify.rs` imported:

```rust
use proofmoney_proof::{verify_rule, verify_supply};
```

and also defined local CLI functions with the same names:

```rust
pub fn verify_supply(...)
pub fn verify_rule(...)
```

Rust does not allow the imported function names and local function names to exist in the same value namespace.

This patch aliases the imported proof functions:

```rust
verify_supply as proof_verify_supply
verify_rule as proof_verify_rule
```

Then the CLI wrapper functions can keep their original names.

## Target repository

```text
https://github.com/proofmoney-protocol/core
```

## File to upload and overwrite

```text
crates/proofmoney-cli/src/commands/verify.rs
```

## Commit message

```text
core: fix CLI verify command name conflict
```

## After commit

GitHub Actions will run again automatically.

If it fails again, open the failed step and send the new error log.
