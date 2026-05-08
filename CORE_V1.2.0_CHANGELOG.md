# ProofMoney Core v1.2.0-testnet-node-api Change Log

## Purpose

This release adds the first minimal ProofMoney testnet node/API skeleton.

It does not launch a public testnet.

It does not launch mainnet.

It does not claim external audit completion.

It does not claim production readiness.

## Added

```text
crates/proofmoney-node/
crates/proofmoney-node/Cargo.toml
crates/proofmoney-node/src/lib.rs
crates/proofmoney-node/src/config.rs
crates/proofmoney-node/src/api.rs
crates/proofmoney-node/src/service.rs
crates/proofmoney-node/README.md
docs/testnet/testnet-node-api-v1.2.0.md
```

## Added Models

- node config model;
- node status API model;
- ledger status API model;
- proof query API model;
- event list API model;
- disabled-by-default event submission model;
- disabled-by-default faucet model;
- API error model.

## Safety Statement

ProofMoney Core remains local MVP software with testnet node/API skeleton materials.

This is not a live public testnet, mainnet, token sale, yield product, airdrop, exchange integration, hosted public API, completed audit, or production wallet.
