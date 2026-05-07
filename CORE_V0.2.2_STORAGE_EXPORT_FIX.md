# ProofMoney Core v0.2.2 Storage Export Fix

## Purpose

This patch fixes the CI error:

```text
unresolved import `proofmoney_storage::load_or_init_ledger`
```

## Cause

The CLI imports:

```rust
use proofmoney_storage::load_or_init_ledger;
```

But the storage crate root must export this function through:

```rust
pub mod ledger_store;
pub use ledger_store::*;
```

## Files Updated

```text
crates/proofmoney-storage/src/lib.rs
crates/proofmoney-storage/src/ledger_store.rs
```

## Expected Result

After this patch, the following imports should resolve:

```rust
use proofmoney_storage::{ledger_path, load_or_init_ledger, save_ledger};
```

## Safety Boundary

This patch only fixes Rust module exports and local ledger storage access.

It does not change protocol rules, PRM issuance, release curve parameters, wallet behavior, or any monetary claim.
