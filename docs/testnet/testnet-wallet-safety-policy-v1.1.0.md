# ProofMoney Testnet Wallet Safety Policy v1.1.0

## Purpose

This document defines wallet safety policy for future ProofMoney public testnet usage.

This is design only.

It is not production wallet guidance.

---

## Testnet Wallet Purpose

A testnet wallet may support:

- testnet address generation;
- message signing;
- ownership proof;
- transfer signing;
- testnet demo flows.

It is for testnet use only.

---

## Testnet Address Format

A future testnet address should use a clearly testnet-specific prefix.

Current local MVP uses:

```text
tprm
```

A future public testnet may continue using a test prefix or define a new one.

The prefix must not be confused with mainnet.

---

## Private Key Warning

Required warning:

```text
Do not use testnet private keys for valuable assets.
```

---

## Test Unit Warning

Required warning:

```text
Testnet units have no monetary value.
```

---

## No Production Custody Statement

Required statement:

```text
The testnet wallet is experimental and is not production custody software.
```

---

## No Valuable Asset Usage

Users should not:

- import real private keys;
- store valuable assets;
- reuse production wallet seed phrases;
- treat testnet balances as monetary balances;
- assume recovery guarantees.

---

## Key Backup Warning

Users should be told:

- testnet keys may be lost;
- testnet state may be reset;
- testnet units may disappear;
- testnet accounts may not carry over to mainnet.

---

## Wallet Reset Behavior

Future wallet reset behavior should clearly state:

- what files are deleted;
- whether ledger state is affected;
- whether testnet state is affected;
- whether keys can be recovered.

---

## Future Production Wallet Gap

A production wallet would require separate:

- threat model;
- secure storage design;
- key backup and recovery design;
- UX review;
- external security audit;
- platform-specific hardening;
- legal and compliance review.

---

## Forbidden Claims

Do not describe a testnet wallet as:

```text
secure wallet
production wallet
safe custody
real PRM wallet
mainnet wallet
exchange wallet
investment wallet
```

Allowed wording:

```text
experimental testnet wallet
testnet-only wallet
local/testnet key-control wallet
```

---

## Safety Boundary

The testnet wallet is experimental and must not be used with valuable assets.
