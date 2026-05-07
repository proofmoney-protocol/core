# ProofMoney Proof Explorer and Local Proof API

## Purpose

The v0.4.0-proof-explorer-api milestone adds local proof JSON exports and a static local Proof Explorer prototype.

This is not a hosted public API.

This is not a public block explorer.

It is a local MVP proof viewer.

## Commands

```bash
proofmoney export-proof-snapshot
proofmoney export-proof-snapshot --json
proofmoney export-proof-snapshot --output proof-snapshot.json
proofmoney export-proof-site-data
proofmoney list-release-events
proofmoney list-release-events --json
proofmoney list-transfer-events
proofmoney list-transfer-events --json
proofmoney prepare-explorer
```

## Local Export Directory

```text
~/.proofmoney/export/
```

Expected files:

```text
starting-state.json
ledger-status.json
supply.json
rule.json
integrity-status.json
release-events.json
transfer-events.json
proof-snapshot.json
```

## Local Explorer Directory

```text
~/.proofmoney/explorer/
```

Expected files:

```text
index.html
styles.css
data/
```

## Safety Boundary

This is local MVP proof data only.

It is not a public network, public API, hosted explorer, token sale, airdrop, yield product, exchange integration, or production wallet.
