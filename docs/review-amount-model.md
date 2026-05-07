# Amount Model and Decimal Parsing Review Notes

## Review Status

```text
Prepared for external review
```

This document records current self-review notes and areas for external reviewers.

It is not an audit report.

---

## Current Model

Current unit model:

```text
1 PRM = 100,000,000 proof
```

The smallest unit is:

```text
proof
```

Amounts are represented with integer arithmetic.

Floating-point math should not be used for monetary amounts.

---

## Areas Reviewed Internally

## 1. Smallest Unit

The MVP defines `proof` as the smallest integer unit.

Internal arithmetic should use integer proof values.

## 2. Decimal Parsing

Decimal parsing should:

- accept valid PRM strings;
- reject negative values;
- reject malformed values;
- reject more than 8 decimal places;
- avoid floating-point conversion;
- check overflow.

## 3. Zero Amount

Zero amount behavior should be explicit.

For transfers, zero amount should be rejected.

For initial supply, zero amount is expected.

## 4. Maximum Supply Boundary

The current maximum supply boundary is:

```text
100,000,000 PRM
```

Reviewers should inspect whether this value is enforced consistently.

---

## Known Risks and Questions

- Are all amount inputs parsed through integer-only logic?
- Are any CLI or JSON paths accidentally accepting floating-point values?
- Are overflow checks sufficient?
- Are display strings and internal units always consistent?
- Are release amount calculations fully deterministic?
- Are there edge cases around maximum supply boundary comparisons?

---

## Suggested External Review Tasks

1. Review `Amount` parsing.
2. Review `checked_add` and `checked_sub` usage.
3. Review release curve calculations.
4. Review supply verification behavior.
5. Add edge-case tests if gaps are found.

---

## Follow-Up Issue Policy

Any discovered issue should be opened as a new GitHub issue with:

- affected crate;
- reproduction steps;
- expected behavior;
- actual behavior;
- risk level;
- suggested fix.

---

## Safety Statement

This review does not certify monetary correctness or production readiness.

ProofMoney Core remains a local MVP prototype.
