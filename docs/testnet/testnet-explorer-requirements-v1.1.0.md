# ProofMoney Public Testnet Explorer Requirements v1.1.0

## Purpose

This document defines requirements for a future ProofMoney public testnet Proof Explorer.

This is design only.

It is not a hosted explorer launch.

---

## Explorer Purpose

The testnet explorer should make testnet proof state visible and understandable.

It should help users inspect:

- ledger height;
- current supply;
- Public Proof Fund balance;
- release events;
- transfer events;
- proof results;
- invalid state indicators;
- testnet safety notices.

---

## Required Views

Initial explorer views may include:

- Overview;
- Ledger Status;
- Release Events;
- Transfer Events;
- Event Detail;
- Supply Proof;
- Rule Proof;
- Integrity Status;
- Faucet Status where applicable;
- Safety Notice.

---

## Ledger Height Display

The explorer should show:

- current height;
- event count;
- last event hash;
- node status;
- network label.

Network label must clearly say:

```text
Testnet
```

---

## Event Listing

Event lists should show:

- event id;
- height;
- event type;
- timestamp;
- event hash;
- hash validity;
- status.

---

## Release Event View

Release event view should show:

- recipient;
- interval;
- actual release amount;
- Public Proof Fund allocation;
- contributor reward;
- event hash;
- proof status.

---

## Transfer Event View

Transfer event view should show:

- transaction id;
- sender;
- receiver;
- amount;
- fee;
- event hash;
- flow status.

---

## Proof Result Display

Proof views should show:

- proof type;
- status;
- rule version;
- summary;
- data;
- safety notice.

---

## Tamper / Invalid State Indicators

The explorer should display warnings for:

- invalid event hash;
- inconsistent supply;
- mismatched last event hash;
- unsupported rule version;
- node not synced;
- stale proof export.

---

## API Data Source

The explorer should read from:

- testnet node read API; or
- exported testnet proof JSON where appropriate.

The data source should be visible to users.

---

## Safety Notices

Required safety notices:

```text
This is testnet data only.
Testnet units have no monetary value.
This is not a trading interface.
This is not a claim page.
This is not a production wallet.
```

---

## Out of Scope

The explorer must not include:

- price chart;
- trading button;
- swap interface;
- claim page;
- airdrop eligibility;
- investment language;
- yield display;
- exchange listing language.

---

## Safety Boundary

The testnet explorer is for testnet proof visibility only.

It is not a trading interface, wallet, or claim page.
