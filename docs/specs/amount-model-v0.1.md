# ProofMoney Amount Model Specification v0.1

## Status

```text
Spec version: v0.1
Scope: Local MVP amount semantics
```

This specification documents the current ProofMoney Core local MVP amount model.

It does not imply public network readiness or monetary value.

---

## 1. Unit Model

The smallest unit is:

```text
proof
```

The display unit is:

```text
PRM
```

The current MVP conversion rule is:

```text
1 PRM = 100,000,000 proof
```

All monetary-like calculations in the MVP should use integer `proof` units.

Floating-point monetary logic is not allowed.

---

## 2. Decimal Display Format

A `proof` amount may be displayed as PRM with 8 fractional decimal places.

Examples:

| Proof units | PRM display |
|---:|---:|
| `1` | `0.00000001` |
| `100000000` | `1.00000000` |
| `123456789` | `1.23456789` |

---

## 3. Decimal Parsing Rules

A decimal PRM string may be parsed into integer proof units.

Valid examples:

```text
0.00000001
1
1.0
1.00000000
12.34567890
```

Invalid examples:

```text
""
abc
1.2.3
-1
1.123456789
```

Rules:

- negative values are invalid;
- malformed strings are invalid;
- more than 8 decimal places are invalid;
- floating-point conversion must not be used;
- parsing must result in integer proof units.

---

## 4. Zero Amount Behavior

Zero amount is valid for some system state values.

Examples:

- initial supply may be zero;
- initial Public Proof Fund balance may be zero.

Zero amount is invalid for local transfer amount.

A transfer amount must be greater than zero.

---

## 5. Maximum Supply Boundary

The current MVP maximum supply boundary is:

```text
100,000,000 PRM
```

Equivalent in proof units:

```text
10,000,000,000,000,000 proof
```

Supply verification should ensure that computed supply does not violate the configured boundary.

---

## 6. Overflow Safety Expectations

Amount arithmetic should use checked or safe integer operations where appropriate.

Future protocol changes must preserve:

- integer-only accounting;
- no floating-point money math;
- explicit overflow handling;
- deterministic calculation behavior.

---

## 7. Safety Boundary

This specification documents local MVP amount semantics only.

It does not imply:

- public network readiness;
- monetary value;
- exchange access;
- yield;
- token sale;
- production accounting certification.

ProofMoney remains experimental and may fail.
