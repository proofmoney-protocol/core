# ProofMoney Core v0.8.0-cli-integration-hardening Change Log

## Purpose

This release hardens local CLI integration, local state validation, tamper detection, proof export freshness metadata, and local transfer demo flows.

It does not claim external audit completion.

It does not claim production readiness.

## Added

```text
proofmoney reset-ledger --yes
proofmoney validate-local-state
proofmoney detect-tampering
scripts/demo-transfer-local.sh
crates/proofmoney-cli/tests/cli_integration.rs
docs/cli-integration-hardening.md
docs/local-state-validation.md
docs/transfer-demo.md
docs/proof-export-freshness.md
```

## Changed

- proof exports now include freshness metadata;
- transfer CLI errors are clearer and more actionable;
- CI now runs local state validation and tamper detection;
- CI now runs the local transfer demo script.

## Review Status

```text
Founder-led internal review and CLI hardening
External audit status: Not audited
Production readiness: Not production-ready
Network status: Local MVP only
```

## Safety Statement

ProofMoney Core remains a local MVP prototype.

It is not a public network, token sale, yield product, airdrop, exchange integration, hosted public API, completed audit, or production wallet.
