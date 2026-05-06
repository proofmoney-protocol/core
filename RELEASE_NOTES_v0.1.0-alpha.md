# ProofMoney Core v0.1.0-alpha Release Notes

## Release Type

Local MVP scaffold.

## Status

Experimental.

This release is not a public network.  
It does not create PRM with monetary value.  
It does not represent a token sale, investment opportunity, yield product, exchange listing, airdrop claim, or future allocation right.

## Included

- Rust Cargo workspace
- `proofmoney-cli`
- `proofmoney-types`
- `proofmoney-crypto`
- `proofmoney-ledger`
- `proofmoney-release`
- `proofmoney-proof`
- `proofmoney-wallet`
- `proofmoney-storage`
- integer-only amount model
- Starting State Proof
- Proof Release Curve simulation
- Proof of Supply
- Proof of Rule
- Proof of Ownership MVP
- CLI smoke commands
- test foundation
- CI workflow

## Core Commands

```bash
cargo run -p proofmoney-cli -- starting-state
cargo run -p proofmoney-cli -- starting-state --json
cargo run -p proofmoney-cli -- simulate-release --interval 1
cargo run -p proofmoney-cli -- verify-supply
cargo run -p proofmoney-cli -- verify-rule
cargo run -p proofmoney-cli -- integrity-status
cargo run -p proofmoney-cli -- create-wallet
```

## Known Limitations

- local prototype only;
- no public node consensus;
- no production wallet security;
- simplified flow verification;
- no full balance engine;
- no independent audit;
- no public network;
- no real PRM value.

## Required Safety Statement

ProofMoney does not offer, sell, or solicit the purchase of PRM. PRM does not guarantee price, liquidity, yield, profit, or exchange access. ProofMoney is experimental and may fail. Test units, if any, have no monetary value.

## Final Statement

If money cannot be verified, it is only a promise.
