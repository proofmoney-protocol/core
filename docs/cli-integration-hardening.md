# ProofMoney CLI Integration Hardening

## Purpose

The v0.8.0 CLI integration hardening milestone improves repeatable local testing and local state inspection.

This is local MVP hardening only.

It is not public network readiness, production wallet readiness, or an external audit.

---

## Added Commands

```bash
proofmoney reset-ledger --yes
proofmoney validate-local-state
proofmoney detect-tampering
```

---

## Integration Test Strategy

CLI integration tests use isolated temporary home directories.

The goal is to avoid modifying the real user home directory during automated tests.

Covered flow:

```text
reset ledger
starting state
append release
ledger status
validate local state
detect tampering
export proof data
prepare explorer
```

---

## Safety Boundary

CLI integration tests improve local MVP confidence but do not imply production readiness.
