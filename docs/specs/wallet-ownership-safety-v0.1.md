# ProofMoney Wallet and Ownership Safety Specification v0.1

## Status

```text
Spec version: v0.1
Scope: Local MVP wallet and ownership safety semantics
```

This specification documents the current MVP wallet and ownership safety boundary.

The MVP wallet is experimental and must not be used with valuable assets.

---

## 1. MVP Wallet Purpose

The MVP wallet exists to support local testing of:

- address generation;
- message signing;
- ownership proof verification;
- local transfer demo flows.

It is not production custody software.

---

## 2. Local Wallet Path

Current local MVP wallet path:

```text
~/.proofmoney/wallets/default.json
```

This file may contain local MVP key material.

---

## 3. Address Format

Current local MVP addresses use a test prefix such as:

```text
tprm
```

Address format is MVP-level and may change.

---

## 4. Message Signing

The MVP wallet may sign local messages for proof testing.

Message signing is used to demonstrate local key control.

It does not imply:

- monetary ownership;
- production custody;
- public network account ownership;
- exchange-recognized address control.

---

## 5. Ownership Proof Boundary

Proof of Ownership verifies local key-control semantics.

It does not verify:

- legal ownership;
- market value;
- exchange custody;
- claim eligibility;
- production asset ownership.

---

## 6. Private Key Exposure Risk

The MVP wallet may store local key material in a local file.

Risks:

- local file exposure;
- accidental sharing;
- unsafe backup behavior;
- user misunderstanding;
- no production-grade custody protection.

---

## 7. Forbidden Claims

Do not describe the MVP wallet as:

```text
secure wallet
production wallet
safe custody
real PRM wallet
asset wallet
mainnet wallet
exchange wallet
```

Allowed wording:

```text
experimental local MVP wallet
local testing wallet
local key-control test wallet
```

---

## 8. User Safety Warning

Required warning:

```text
The MVP wallet is experimental and must not be used with valuable assets.
```

---

## 9. Safety Boundary

The MVP wallet is experimental.

It is not:

- production custody;
- hardware wallet integration;
- insured custody;
- audited wallet software;
- exchange-compatible wallet;
- public mainnet wallet.
