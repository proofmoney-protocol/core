# ProofMoney Node

## Status

```text
Version: v1.3.0-testnet-read-api-server
Status: Local read-only API server skeleton
Public testnet: Not launched
Mainnet: Not launched
```

## Purpose

`proofmoney-node` is the first minimal node/API model and local read-only API server skeleton for future ProofMoney public testnet development.

It currently provides:

- node config model;
- node status response model;
- ledger status response model;
- proof query request / response model;
- event list model;
- disabled-by-default event submission model;
- disabled-by-default faucet model;
- local read-only API server route dispatcher;
- unit tests for endpoint JSON responses and safety boundaries.

## Local Read-Only Routes

Modeled local read-only routes:

```text
GET /health
GET /status
GET /ledger/status
GET /proofs/:proof_type
GET /events
GET /events/releases
GET /events/transfers
```

Supported proof path values:

```text
starting-state
supply
rule
flow
integrity-status
```

## Disabled APIs

These remain disabled by default:

```text
POST /events
POST /faucet/request
```

## What This Is Not

This crate is not:

- a live public testnet node;
- a mainnet node;
- a hosted public API server;
- a consensus implementation;
- a production wallet;
- a faucet launch;
- a token sale;
- an airdrop;
- a yield product;
- an exchange integration.

## Safety Boundary

This crate is a local read-only testnet API server skeleton only.

Testnet units, if any in a future milestone, have no monetary value and do not imply future PRM allocation.
