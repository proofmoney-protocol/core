# ProofMoney Core v0.5.0-developer-release Change Log

## Implemented

This patch addresses the core development direction of the `v0.5.0-developer-release` milestone.

Implemented:

- developer quickstart guide;
- local demo script;
- sample proof data fixtures;
- architecture overview document;
- improved README examples;
- security review scope document;
- contributor guide v1.0;
- CI execution of local demo script.

## Added Files

```text
docs/developer-quickstart.md
docs/architecture-overview.md
docs/security-review-scope.md
scripts/demo-local.sh
fixtures/
CONTRIBUTING.md
```

## Developer Commands

```bash
cargo build --workspace --all-targets
cargo test --workspace --all-targets
bash scripts/demo-local.sh
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
