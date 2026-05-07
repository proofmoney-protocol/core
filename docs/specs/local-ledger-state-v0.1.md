# ProofMoney Local Ledger State Specification v0.1

## Status

```text
Spec version: v0.1
Scope: Local MVP ledger state semantics
```

This specification documents the current local MVP ledger state.

Local ledger state is not public network consensus.

---

## 1. Local Storage Path

Current local ledger path:

```text
~/.proofmoney/ledger.json
```

This path is local MVP state only.

---

## 2. Ledger Fields

Current local ledger state includes:

| Field | Description |
|---|---|
| `ledger_version` | Local ledger version |
| `current_height` | Current local event height |
| `current_supply` | Current stored supply |
| `public_fund_balance` | Public Proof Fund balance |
| `rule_version` | Active local MVP rule version |
| `events` | Local event list |
| `last_event_hash` | Hash of the latest local event |

---

## 3. Reset Behavior

The CLI supports:

```bash
proofmoney reset-ledger --yes
proofmoney reset-ledger --json --yes
```

Rules:

- requires explicit `--yes`;
- removes local MVP ledger state;
- does not reset local wallet;
- does not affect any public network because no public network exists.

---

## 4. Validation Behavior

The CLI supports:

```bash
proofmoney validate-local-state
proofmoney validate-local-state --json
```

Validation checks include:

- event hash validity;
- rule version consistency;
- event height order;
- computed supply;
- Public Proof Fund recomputation;
- balance recomputation;
- last event hash consistency.

---

## 5. Tamper Detection Behavior

The CLI supports:

```bash
proofmoney detect-tampering
proofmoney detect-tampering --json
```

Tamper detection checks obvious local inconsistencies such as:

- invalid event hash;
- mutated event payload;
- mismatched current supply;
- mismatched Public Proof Fund balance;
- mismatched last event hash;
- invalid rule version;
- invalid event height order.

---

## 6. Known Local-Only Limitations

The local ledger:

- is not public consensus;
- can be manually modified by the local user;
- does not provide network-level finality;
- does not provide peer-to-peer replication;
- does not include mempool behavior;
- does not provide production forensic security.

---

## 7. Safety Boundary

Local ledger state is local MVP state only.

It is not:

- public network state;
- exchange-recognized state;
- audited financial state;
- custody state;
- claim eligibility state.
