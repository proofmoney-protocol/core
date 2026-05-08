# ProofMoney Local MVP Command Reference v1.0.0

## Purpose

This document is the stable local MVP command reference for ProofMoney Core v1.0.0.

All commands operate on local MVP state only.

No command interacts with a public network.

No command creates PRM with monetary value.

---

## Build and Test

```bash
cargo build --workspace --all-targets
cargo test --workspace --all-targets
```

---

## Starting State

```bash
cargo run -p proofmoney-cli -- starting-state
cargo run -p proofmoney-cli -- starting-state --json
```

Purpose:

- verifies local MVP starting state assumptions;
- confirms no initial supply, no private allocation, and no hidden founder allocation.

---

## Simulate Release

```bash
cargo run -p proofmoney-cli -- simulate-release --interval 1
cargo run -p proofmoney-cli -- simulate-release --interval 1 --append
cargo run -p proofmoney-cli -- simulate-release --interval 1 --recipient <address> --append
```

Purpose:

- simulates a local MVP release event;
- optionally appends it to the local ledger.

Safety:

- local release simulation only;
- not a token sale;
- not a public network issuance.

---

## Ledger Status

```bash
cargo run -p proofmoney-cli -- ledger-status
cargo run -p proofmoney-cli -- ledger-status --json
```

Purpose:

- prints local ledger version;
- current height;
- current supply;
- Public Proof Fund balance;
- rule version;
- event count;
- last event hash;
- local storage path.

---

## Reset Ledger

```bash
cargo run -p proofmoney-cli -- reset-ledger --yes
cargo run -p proofmoney-cli -- reset-ledger --json --yes
```

Purpose:

- resets local MVP ledger state;
- does not reset local wallet.

Safety:

- affects local MVP files only;
- no public network exists.

---

## Validate Local State

```bash
cargo run -p proofmoney-cli -- validate-local-state
cargo run -p proofmoney-cli -- validate-local-state --json
```

Purpose:

- validates event hashes;
- checks rule versions;
- checks height order;
- recomputes supply;
- recomputes Public Proof Fund balance;
- recomputes local balances;
- checks last event hash.

---

## Detect Tampering

```bash
cargo run -p proofmoney-cli -- detect-tampering
cargo run -p proofmoney-cli -- detect-tampering --json
```

Purpose:

- detects obvious local ledger inconsistencies;
- helps identify local file mutation.

Limitation:

- not production forensic security.

---

## Verify Supply

```bash
cargo run -p proofmoney-cli -- verify-supply
cargo run -p proofmoney-cli -- verify-supply --json
```

Purpose:

- compares stored local supply with computed local supply.

---

## Verify Rule

```bash
cargo run -p proofmoney-cli -- verify-rule
cargo run -p proofmoney-cli -- verify-rule --json
```

Purpose:

- checks current local rule version and rule compatibility.

---

## Integrity Status

```bash
cargo run -p proofmoney-cli -- integrity-status
cargo run -p proofmoney-cli -- integrity-status --json
```

Purpose:

- summarizes local MVP proof integrity status.

---

## Create Wallet

```bash
cargo run -p proofmoney-cli -- create-wallet --force
cargo run -p proofmoney-cli -- create-wallet --force --json
```

Purpose:

- creates a local experimental MVP wallet for testing.

Safety:

- not production custody;
- do not use with valuable assets.

---

## New Address

```bash
cargo run -p proofmoney-cli -- new-address
cargo run -p proofmoney-cli -- new-address --json
```

Purpose:

- displays local MVP wallet address.

---

## Sign Message

```bash
cargo run -p proofmoney-cli -- sign-message --message "hello"
```

Purpose:

- signs a local message for MVP ownership testing.

---

## Verify Ownership

```bash
cargo run -p proofmoney-cli -- verify-ownership --address <address> --message <message> --signature <signature> --public-key <public_key>
```

Purpose:

- verifies local key-control semantics.

Safety:

- not legal ownership;
- not production custody;
- not exchange-recognized ownership.

---

## Create Transfer

```bash
cargo run -p proofmoney-cli -- create-transfer --from <sender> --to <receiver> --amount 1.25
cargo run -p proofmoney-cli -- create-transfer --from <sender> --to <receiver> --amount 1.25 --append
```

Purpose:

- creates a local MVP transfer event;
- optionally appends it to local ledger.

Safety:

- no public network broadcast;
- no monetary value created.

---

## Verify Flow

```bash
cargo run -p proofmoney-cli -- verify-flow --tx <transaction_id>
cargo run -p proofmoney-cli -- verify-flow --tx <transaction_id> --json
```

Purpose:

- verifies local Proof of Flow for a local transfer event.

---

## Export Proof Snapshot

```bash
cargo run -p proofmoney-cli -- export-proof-snapshot --json
cargo run -p proofmoney-cli -- export-proof-snapshot --output /tmp/proof-snapshot.json
```

Purpose:

- exports local proof snapshot;
- includes freshness metadata.

---

## Export Proof Site Data

```bash
cargo run -p proofmoney-cli -- export-proof-site-data
```

Purpose:

- exports local proof JSON data for local explorer.

---

## List Release Events

```bash
cargo run -p proofmoney-cli -- list-release-events
cargo run -p proofmoney-cli -- list-release-events --json
```

Purpose:

- lists local release event proofs.

---

## List Transfer Events

```bash
cargo run -p proofmoney-cli -- list-transfer-events
cargo run -p proofmoney-cli -- list-transfer-events --json
```

Purpose:

- lists local transfer event proofs.

---

## Prepare Explorer

```bash
cargo run -p proofmoney-cli -- prepare-explorer
```

Purpose:

- prepares static local Proof Explorer files.

Safety:

- local explorer only;
- not hosted public block explorer;
- not claim page;
- not trading interface.

---

## Safety Statement

All commands operate on local MVP state only.

They do not interact with any public network.

They do not create PRM with monetary value.
