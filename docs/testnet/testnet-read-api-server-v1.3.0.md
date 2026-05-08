# ProofMoney Testnet Read API Server v1.3.0

## Purpose

This document describes the local read-only API server skeleton added in v1.3.0.

This is not a live public testnet.

It is not a hosted public API.

It is not mainnet.

---

## Scope

The v1.3.0 local read-only API server skeleton provides route handling models for:

```text
GET /health
GET /status
GET /ledger/status
GET /proofs/:proof_type
GET /events
GET /events/releases
GET /events/transfers
```

The server defaults to local read-only semantics.

Write APIs remain disabled.

The faucet remains disabled.

---

## Default Bind Policy

Default bind configuration:

```text
host: 127.0.0.1
port: 8787
```

This keeps the design local-first.

---

## GET /health

Response includes:

- status;
- service name;
- node version;
- mode;
- safety notice.

---

## GET /status

Response includes:

- node version;
- network name;
- protocol version;
- mode;
- ledger height;
- event count;
- last event hash;
- sync status;
- write API enabled;
- faucet enabled;
- safety notice.

---

## GET /ledger/status

Response includes:

- ledger version;
- current height;
- current supply;
- Public Proof Fund balance;
- rule version;
- event count;
- last event hash;
- network label;
- safety notice.

---

## GET /proofs/:proof_type

Supported proof path values:

```text
starting-state
supply
rule
flow
integrity-status
```

Unsupported proof types return structured errors.

Proof outputs are local/testnet verification results only and do not imply external audit.

---

## GET /events

Returns event list model.

In the current skeleton, empty event lists are valid.

---

## GET /events/releases

Returns release event list model.

---

## GET /events/transfers

Returns transfer event list model.

---

## Disabled Write APIs

The following remain disabled by default:

```text
POST /events
POST /faucet/request
```

No public write API is launched.

No faucet is live.

---

## Smoke Tests

The `proofmoney-node` crate includes unit tests for:

- localhost bind config;
- health route;
- status route;
- ledger status route;
- proof query route;
- unsupported proof error;
- event list routes;
- unsupported route error.

---

## Safety Statement

This is a local read-only API server skeleton only.

It is not:

- live public testnet;
- mainnet;
- token sale;
- airdrop;
- yield product;
- exchange integration;
- hosted public API;
- production wallet;
- completed audit.
