# ProofMoney Ownership and Flow MVP

## Purpose

This document describes the v0.3.0 ownership and flow MVP scope.

The goal is to make local wallet control, message signing, transfer event creation, local balance tracking, and Proof of Flow testable through CLI.

## Commands

```bash
proofmoney create-wallet
proofmoney new-address
proofmoney sign-message --message "verify ownership"
proofmoney verify-ownership --address <address> --message <message> --signature <signature> --public-key <public_key>
proofmoney create-transfer --from <address> --to <address> --amount 1.25
proofmoney create-transfer --from <address> --to <address> --amount 1.25 --append
proofmoney verify-flow --tx <transaction_id>
```

## Safety Boundary

This is local MVP validation only.

It is not a public network.  
It is not production wallet security.  
It does not create PRM with monetary value.  
It is not a token sale, airdrop, exchange integration, or yield product.
