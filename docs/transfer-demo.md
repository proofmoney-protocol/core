# ProofMoney Local Transfer Demo

## Purpose

The local transfer demo provides a repeatable end-to-end CLI flow for local MVP testing.

It is not a public network transaction.

It does not create PRM with monetary value.

---

## Script

```bash
bash scripts/demo-transfer-local.sh
```

---

## Flow

The script:

1. resets local ledger;
2. creates local MVP wallet;
3. reads sender address;
4. appends release event to sender;
5. creates a sample receiver address;
6. creates and appends a transfer;
7. validates local state;
8. detects obvious tampering;
9. lists transfer events;
10. exports proof snapshot;
11. prepares local Proof Explorer.

---

## Safety Boundary

This is a local demo only.

It does not broadcast transactions.

It does not interact with any public network.

Test units have no monetary value.
