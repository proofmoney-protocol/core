# ProofMoney Proof Release Curve Specification v0.1

## Status

```text
Spec version: v0.1
Scope: Local MVP release simulation semantics
```

This specification documents the current local MVP Proof Release Curve behavior.

It is not a sale schedule.

It is not a fundraising plan.

It is not an airdrop plan.

It is not a yield model.

It is not a price narrative.

---

## 1. Purpose

The Proof Release Curve exists to make issuance behavior inspectable and bounded.

Every local MVP release event should have a verifiable origin and computed allocation breakdown.

---

## 2. Local MVP Release Simulation

The current CLI supports local release simulation:

```bash
proofmoney simulate-release --interval 1
proofmoney simulate-release --interval 1 --append
```

The command may also specify a local recipient:

```bash
proofmoney simulate-release --interval 1 --recipient <address> --append
```

This creates local MVP release events only.

It does not create PRM with monetary value.

---

## 3. Allocation Components

A release event includes:

- actual release amount;
- Public Proof Fund allocation;
- proof contributor reward.

In the current MVP simulation:

```text
Public Proof Fund allocation = 3%
Proof contributor reward = 97%
```

These are MVP parameters and not a fundraising promise.

---

## 4. Supply Boundary

The release curve must respect the MVP maximum supply boundary.

Current maximum supply boundary:

```text
100,000,000 PRM
```

---

## 5. Release Event Fields

Release event payload should include:

- interval;
- recipient;
- actual release;
- Public Proof Fund allocation;
- proof contributor reward;
- integer proof-unit values;
- MVP scope marker.

---

## 6. What Is Not Included

The current MVP release curve does not include:

- token sale;
- private sale;
- presale;
- airdrop;
- yield distribution;
- staking reward;
- exchange listing;
- price support;
- mainnet issuance;
- guaranteed future allocation.

---

## 7. Safety Boundary

The Proof Release Curve specification documents local MVP release simulation semantics only.

It must not be presented as:

- sale schedule;
- fundraising plan;
- investor allocation plan;
- exchange listing plan;
- yield model;
- airdrop eligibility model.
