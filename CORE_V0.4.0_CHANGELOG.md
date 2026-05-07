# ProofMoney Core v0.4.0-proof-explorer-api Change Log

## Implemented

This patch addresses the core development direction of the `v0.4.0-proof-explorer-api` milestone.

Implemented:

- proof snapshot export command;
- stable local Proof API JSON structures;
- local proof export directory;
- release event proof listing;
- transfer event proof listing;
- static local Proof Explorer prototype;
- explorer data preparation command;
- README and documentation updates;
- CI smoke tests for proof export commands.

## Commands

```bash
cargo run -p proofmoney-cli -- export-proof-snapshot
cargo run -p proofmoney-cli -- export-proof-snapshot --json
cargo run -p proofmoney-cli -- export-proof-snapshot --output proof-snapshot.json
cargo run -p proofmoney-cli -- export-proof-site-data
cargo run -p proofmoney-cli -- list-release-events
cargo run -p proofmoney-cli -- list-transfer-events
cargo run -p proofmoney-cli -- prepare-explorer
```

## Still Limited

- no public network;
- no hosted API;
- no production explorer;
- no production wallet security;
- no independent audit;
- no exchange integration;
- no real PRM value.

## Safety Statement

This remains a local MVP prototype only. It is not a PRM sale, public network, airdrop, exchange integration, or yield product.
