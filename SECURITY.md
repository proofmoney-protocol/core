# Security Policy

ProofMoney Core is experimental software.

Do not use this software with valuable assets.

## Current Security Status

This repository is a local MVP prototype.

It is not production wallet software.  
It is not a public network.  
It is not a custody system.  
It does not create PRM with monetary value.

## Never Share Secrets

ProofMoney will never ask for:

- private keys;
- seed phrases;
- recovery phrases;
- wallet files;
- remote access to user devices.

## MVP Wallet Warning

Any private key material generated or displayed by MVP commands is for local testing only.

Do not use MVP-generated keys for valuable assets.

## Reporting Security Issues

If you discover a serious vulnerability, do not publicly disclose exploitable details before coordination.

Until a dedicated reporting channel is published, open a GitHub issue with limited detail and request private coordination.

## Security Scope

Relevant issues include:

- private key exposure;
- signature verification failure;
- supply verification failure;
- invalid release acceptance;
- ownership proof failure;
- flow verification failure;
- rule verification failure;
- misleading proof status;
- unsafe CLI output.

Verification reduces trust requirements, but it does not eliminate risk.
