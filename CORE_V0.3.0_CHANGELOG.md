# ProofMoney Core v0.3.0-ownership-and-flow Change Log

## Implemented

This patch addresses the core development direction of the `v0.3.0-ownership-and-flow` milestone.

Implemented:

- local wallet persistence;
- no silent wallet overwrite without `--force`;
- `new-address` CLI command;
- local wallet message signing;
- transfer event creation;
- deterministic transfer event hashing;
- local balance tracking;
- insufficient balance rejection;
- stronger Proof of Flow;
- `create-transfer` CLI command;
- `verify-flow` CLI command;
- README and documentation updates.

## Commands

```bash
cargo run -p proofmoney-cli -- create-wallet
cargo run -p proofmoney-cli -- new-address
cargo run -p proofmoney-cli -- sign-message --message "verify ownership"
cargo run -p proofmoney-cli -- create-transfer --from <address> --to <address> --amount 1.25
cargo run -p proofmoney-cli -- verify-flow --tx <transaction_id>
```

## Still Limited

- no public network;
- no production wallet security;
- no full consensus;
- no independent audit;
- no exchange integration;
- no real PRM value.

## Safety Statement

This remains a local MVP prototype only. It is not a PRM sale, public network, airdrop, exchange integration, or yield product.
