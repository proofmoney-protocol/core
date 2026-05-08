# ProofMoney Core v1.3.0-testnet-read-api-server Change Log

## Purpose

This release adds a local read-only API server skeleton to `proofmoney-node`.

It does not launch a public testnet.

It does not launch mainnet.

## Added

```text
crates/proofmoney-node/src/server.rs
docs/testnet/testnet-read-api-server-v1.3.0.md
```

## Updated

```text
crates/proofmoney-node/src/lib.rs
crates/proofmoney-node/src/api.rs
crates/proofmoney-node/src/config.rs
crates/proofmoney-node/src/service.rs
crates/proofmoney-node/README.md
README.md
```

## Added Local Read-Only Routes

```text
GET /health
GET /status
GET /ledger/status
GET /proofs/:proof_type
GET /events
GET /events/releases
GET /events/transfers
```

## Still Disabled

```text
POST /events
POST /faucet/request
```

## Safety Statement

This is a local read-only API server skeleton only.

It is not a live public testnet, mainnet, token sale, yield product, airdrop, exchange integration, hosted public API, completed audit, or production wallet.
