# ProofMoney Core Next Steps v1.1

After v0.1.0-alpha scaffold is committed and CI passes, the next engineering work should focus on correctness.

## Priority 1: Make CI Pass

- ensure `cargo fmt --all -- --check` passes
- ensure `cargo build --workspace --all-targets` passes
- ensure `cargo test --workspace --all-targets` passes
- ensure CLI smoke tests pass

## Priority 2: Strengthen Event Validation

- deterministic event hash
- reject missing payload fields
- reject invalid event types
- reject mismatched rule versions
- reject supply boundary violation

## Priority 3: Improve Ledger State

- append release events to local ledger
- compute current supply from events
- compute Public Proof Fund balance from events
- persist ledger to local JSON

## Priority 4: Improve Ownership Proof

- create wallet
- sign message
- verify ownership with address + public key + signature
- avoid unsafe default private key output in later versions

## Priority 5: Flow Verification

- define transfer event model
- validate amount > 0
- validate receiver address
- validate signature
- implement simple account balance tracking
- reject insufficient balance

## Priority 6: MVP Report

Publish:

```text
ProofMoney Core MVP Report v0.1.0-alpha
```

Include:

- implemented modules
- proof outputs
- test results
- known limitations
- no-sale statement
- no-value statement
