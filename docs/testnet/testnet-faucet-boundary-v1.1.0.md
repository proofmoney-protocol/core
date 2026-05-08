# ProofMoney Testnet Faucet Boundary v1.1.0

## Purpose

This document defines the safety boundary for a future ProofMoney public testnet faucet.

This is design only.

There is no live faucet in this milestone.

---

## Faucet Purpose

A testnet faucet may distribute test units for:

- testing transfers;
- testing Proof of Flow;
- testing wallet flows;
- testing explorer visibility;
- testing proof export;
- testing API behavior.

---

## Test Unit Rules

Future testnet units must be described as:

```text
test units with no monetary value
```

They must not be described as:

```text
free PRM
airdrop
reward
allocation
claimable token
investment opportunity
future mainnet token
```

---

## Distribution Rules

Initial faucet rules may include:

- one request per address per time window;
- maximum test amount per request;
- cooldown period;
- IP / address rate limiting;
- abuse monitoring;
- faucet reset possibility.

---

## Abuse Prevention

Potential abuse controls:

- address-based rate limits;
- IP-based rate limits;
- captcha or manual gate if needed;
- request logs;
- temporary blocklist;
- faucet balance cap;
- reset policy.

---

## No Monetary Value Statement

Required statement:

```text
Testnet units have no monetary value.
```

---

## No Airdrop Statement

Required statement:

```text
Testnet faucet distribution is not an airdrop and does not imply future PRM allocation.
```

---

## No Test-to-Main Conversion Promise

Required statement:

```text
Testnet units do not convert to mainnet PRM.
```

---

## Faucet Logging Expectations

A future faucet may log:

- timestamp;
- testnet address;
- requested amount;
- request IP hash or abuse signal;
- status;
- error reason.

Privacy and retention should be considered before implementation.

---

## Safety Statement

The faucet distributes test units only.

Test units have no monetary value and do not imply future PRM allocation.
