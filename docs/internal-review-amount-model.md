# Internal Review: Amount Model and Supply Boundary

## Review Type

Founder-led internal review.

This is not an external audit.

---

## Reviewed Areas

- smallest unit definition;
- `1 PRM = 100,000,000 proof`;
- decimal parsing;
- invalid amount rejection;
- zero amount behavior;
- overflow protection;
- maximum supply boundary;
- release amount calculation;
- supply verification consistency.

---

## Internal Assessment

The current MVP uses integer-only accounting for monetary-like amounts.

This is the correct direction for a protocol that wants verifiable supply and transfer logic.

Floating-point monetary logic should remain forbidden.

---

## Positive Checks

- Amounts are represented as integer proof units.
- Decimal parsing rejects more than 8 decimal places.
- Negative string amounts should be rejected.
- Malformed string amounts should be rejected.
- Zero transfer amounts should be rejected at CLI / flow boundary.
- Supply verification compares stored and computed supply.

---

## Risks

- All future monetary-like code must continue using integer units.
- Reviewers should inspect any new code path that parses human decimal strings.
- Release curve arithmetic should remain deterministic and overflow-aware.
- Maximum supply boundary should remain enforced consistently.

---

## Internal Outcome

```text
Status: acceptable for local MVP, requires external review before public network design
```

---

## Safety Statement

This review does not certify monetary correctness or production readiness.
