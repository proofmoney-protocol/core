# ProofMoney Node

## Status

```text
Version: v1.2.0-testnet-node-api
Status: Minimal testnet node/API skeleton
Public testnet: Not launched
Mainnet: Not launched
```

## Purpose

`proofmoney-node` is the first minimal node/API model skeleton for future ProofMoney public testnet development.

It currently provides:

- node config model;
- node status response model;
- ledger status response model;
- proof query request / response model;
- event list model;
- disabled-by-default event submission model;
- disabled-by-default faucet model;
- node service placeholder;
- unit tests for JSON serialization and safety boundaries.

## What This Is Not

This crate is not:

- a live public testnet node;
- a mainnet node;
- a public API server;
- a consensus implementation;
- a production wallet;
- a faucet launch;
- a token sale;
- an airdrop;
- a yield product;
- an exchange integration.

## Safety Boundary

This crate is a testnet node/API skeleton only.

Testnet units, if any in a future milestone, have no monetary value and do not imply future PRM allocation.
