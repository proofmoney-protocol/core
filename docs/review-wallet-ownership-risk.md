# Wallet and Ownership Proof Risk Review Notes

## Review Status

```text
Prepared for external review
```

This document records current self-review notes and areas for external reviewers.

It is not an audit report.

---

## Current Wallet Model

The MVP wallet is local and experimental.

Local wallet path:

```text
~/.proofmoney/wallets/default.json
```

The MVP wallet can:

- generate a local test keypair;
- derive an MVP address;
- sign messages;
- support ownership proof testing.

---

## Explicit Limitation

The MVP wallet is not production custody software.

It must not be used with valuable assets.

Private key handling is intentionally flagged as unsafe for production.

---

## Areas Reviewed Internally

## 1. Wallet File Storage

The wallet is stored locally for MVP testing.

Reviewers should inspect:

- file creation behavior;
- overwrite behavior;
- `--force` behavior;
- private key exposure risk;
- safety warnings.

## 2. Address Derivation

Reviewers should inspect:

- public key to address relationship;
- address prefix usage;
- address validation;
- mismatch handling.

## 3. Message Signing

Reviewers should inspect:

- signing message consistency;
- signature encoding;
- public key verification;
- invalid signature handling.

## 4. Ownership Proof

Ownership proof should show local key control only.

It should not imply investment value, custody safety, or network ownership.

---

## Known Risks and Questions

- Private key material may be stored unencrypted in MVP context.
- Local wallet should not be reused for valuable assets.
- Address format is MVP-level, not final production format.
- Ownership proof is local key-control proof only.
- Wallet overwrite behavior must remain explicit.

---

## Suggested External Review Tasks

1. Review local wallet generation.
2. Review address derivation.
3. Review message signing.
4. Review signature verification.
5. Review CLI safety warnings.
6. Review private key exposure language.
7. Recommend production wallet requirements for future scope.

---

## Safety Statement

The MVP wallet is experimental and must not be used with valuable assets.

This review does not certify custody safety.
