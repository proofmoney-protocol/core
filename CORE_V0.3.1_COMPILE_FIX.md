# ProofMoney Core v0.3.1 Compile Fix

## Purpose

This patch fixes the first v0.3.0 ownership-and-flow compile failure.

## Error 1

```text
no method named `hash_view` found for reference `&Event`
```

## Fix

Update:

```text
crates/proofmoney-types/src/event.rs
```

to include:

```rust
EventHashView
impl Event { pub fn hash_view(&self) -> EventHashView { ... } }
```

This allows deterministic event hashing to exclude the mutable `hash` field.

## Error 2

```text
unresolved import `proofmoney_ledger`
```

## Fix

Update:

```text
crates/proofmoney-proof/Cargo.toml
```

to include:

```toml
proofmoney-ledger = { path = "../proofmoney-ledger" }
```

This allows Proof of Flow to use local ledger helpers.

## Safety Boundary

This patch only fixes Rust type/module availability.

It does not change protocol rules, PRM issuance, release curve parameters, wallet custody claims, or any monetary claim.
