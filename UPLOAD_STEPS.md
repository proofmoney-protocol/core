# ProofMoney Core Wallet Dependency Fix v0.1.2

## What this fixes

CI failed because `proofmoney-wallet` imports:

```rust
use ed25519_dalek::SigningKey;
```

but `ed25519-dalek` was not listed in:

```text
crates/proofmoney-wallet/Cargo.toml
```

This patch adds:

```toml
ed25519-dalek.workspace = true
```

## Target repository

```text
https://github.com/proofmoney-protocol/core
```

## File to upload and overwrite

```text
crates/proofmoney-wallet/Cargo.toml
```

## Commit message

```text
core: add wallet ed25519 dependency
```

## After commit

GitHub Actions will run again automatically.

If it fails again, open the failed step and send the new error log.
