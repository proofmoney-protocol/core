# ProofMoney Local State Validation

## Purpose

Local state validation checks the consistency of local MVP ledger state.

It validates local files only.

It does not validate public consensus because no public network exists.

---

## Commands

```bash
proofmoney validate-local-state
proofmoney validate-local-state --json
proofmoney detect-tampering
proofmoney detect-tampering --json
```

---

## Checks

The commands check:

- event hash validity;
- rule version consistency;
- event height order;
- computed supply;
- Public Proof Fund recomputation;
- balance recomputation;
- last event hash consistency.

---

## Limitations

Local tamper detection is not production forensic security.

Local JSON files can still be modified by the local user.

These commands only detect obvious local inconsistencies.

---

## Safety Boundary

This validates local MVP state only.

It does not imply public settlement finality or public consensus.
