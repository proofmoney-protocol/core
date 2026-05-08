# ProofMoney Testnet Network Message Model v1.1.0

## Purpose

This document defines the initial network message model for a future ProofMoney public testnet.

This is design only.

It does not define final mainnet consensus.

---

## Versioning

All messages should include or imply:

```text
protocol_version
network
node_version
```

Initial testnet network label:

```text
proofmoney-testnet
```

---

## Message Categories

Initial message categories:

- node status;
- event submission;
- event broadcast;
- ledger state query;
- proof query;
- transfer event query;
- release event query;
- error response;
- version negotiation.

---

## Node Status

Request:

```json
{
  "type": "node_status_request",
  "protocol_version": "v0.1",
  "network": "proofmoney-testnet"
}
```

Response:

```json
{
  "type": "node_status_response",
  "node_version": "v1.1.0-design",
  "network": "proofmoney-testnet",
  "ledger_height": 100,
  "last_event_hash": "hash",
  "status": "ok"
}
```

---

## Event Submission

Request:

```json
{
  "type": "event_submit_request",
  "protocol_version": "v0.1",
  "network": "proofmoney-testnet",
  "event": {}
}
```

Response:

```json
{
  "type": "event_submit_response",
  "accepted": true,
  "event_id": "event-id",
  "height": 101,
  "event_hash": "hash"
}
```

---

## Event Broadcast

Broadcast message:

```json
{
  "type": "event_broadcast",
  "protocol_version": "v0.1",
  "network": "proofmoney-testnet",
  "event": {}
}
```

---

## Ledger State Query

Request:

```json
{
  "type": "ledger_status_request",
  "network": "proofmoney-testnet"
}
```

Response:

```json
{
  "type": "ledger_status_response",
  "current_height": 100,
  "current_supply": "1000.00000000",
  "public_proof_fund_balance": "30.00000000",
  "event_count": 100,
  "last_event_hash": "hash"
}
```

---

## Proof Query

Request:

```json
{
  "type": "proof_query_request",
  "proof_type": "proof_of_supply"
}
```

Response:

```json
{
  "type": "proof_query_response",
  "proof_type": "proof_of_supply",
  "status": "Valid",
  "data": {},
  "summary": "Supply is valid."
}
```

---

## Transfer Event Query

Request:

```json
{
  "type": "transfer_events_request",
  "limit": 50
}
```

Response:

```json
{
  "type": "transfer_events_response",
  "events": []
}
```

---

## Release Event Query

Request:

```json
{
  "type": "release_events_request",
  "limit": 50
}
```

Response:

```json
{
  "type": "release_events_response",
  "events": []
}
```

---

## Error Response

Error response:

```json
{
  "type": "error_response",
  "code": "invalid_event_hash",
  "message": "Event hash is invalid.",
  "recoverable": false
}
```

Error codes may include:

```text
invalid_schema
invalid_event_hash
invalid_rule_version
invalid_signature
insufficient_balance
rate_limited
unsupported_protocol_version
node_not_synced
internal_error
```

---

## Version Negotiation

Request:

```json
{
  "type": "version_request",
  "supported_protocol_versions": ["v0.1"]
}
```

Response:

```json
{
  "type": "version_response",
  "selected_protocol_version": "v0.1",
  "network": "proofmoney-testnet"
}
```

---

## Safety Boundary

The message model is for testnet design only.

It does not define final mainnet consensus, token-sale infrastructure, exchange integration, or production custody behavior.
