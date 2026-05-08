# ProofMoney Core v1.0.0 Local MVP Public Summary

## Summary

ProofMoney Core has reached its first stable local MVP baseline:

```text
v1.0.0-local-mvp-freeze
```

ProofMoney is an open protocol initiative for verifiable money integrity.

> If money cannot be verified, it is only a promise.

The current release is a local Rust MVP baseline that developers can run, inspect, test, and review locally.

---

## What Core v1.0.0 Includes

The local MVP includes:

- local ledger;
- local wallet testing;
- Starting State Proof;
- Proof of Issuance;
- Proof of Supply;
- Proof of Ownership;
- Proof of Flow;
- Proof of Rule;
- local release simulation;
- local transfer demo;
- local state validation;
- local tamper detection;
- proof export JSON;
- static local Proof Explorer;
- protocol specifications;
- documentation and safety boundaries.

---

## What Developers Can Run

```bash
git clone https://github.com/proofmoney-protocol/core.git
cd core

cargo build --workspace --all-targets
cargo test --workspace --all-targets

bash scripts/demo-local.sh
bash scripts/demo-transfer-local.sh
```

---

## Protocol Specs

The v1.0.0 local MVP baseline includes protocol specifications for:

- Amount Model;
- Event Schema;
- Proof Result Schema;
- Proof Release Curve;
- Local Ledger State;
- Proof Export JSON;
- Wallet and Ownership Safety.

Entry point:

```text
docs/specs/protocol-spec-index-v0.1.md
```

---

## Repositories

Core:

```text
https://github.com/proofmoney-protocol/core
```

Docs:

```text
https://github.com/proofmoney-protocol/docs
```

Website:

```text
https://proofmoney.org
```

---

## What This Is Not

This is not:

- a public network;
- a mainnet launch;
- a token sale;
- an airdrop;
- a yield product;
- an exchange integration;
- a production wallet;
- an external audit;
- an investment opportunity.

Test units, if any, have no monetary value.

---

## What Comes Next

Possible next directions:

- public testnet design;
- SDK design;
- external security review;
- protocol audit preparation;
- node architecture design;
- public explorer architecture;
- wallet safety research.

No future stage should be interpreted as guaranteed.

---

## Safety Statement

ProofMoney does not offer, sell, or solicit the purchase of PRM.

PRM does not guarantee price, liquidity, yield, profit, or exchange access.

ProofMoney is experimental and may fail.
