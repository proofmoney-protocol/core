# Internal Review: Wallet and Ownership Proof Risk

## Review Type

Founder-led internal review.

This is not an external audit.

---

## Reviewed Areas

- local wallet file storage;
- private key exposure warning;
- overwrite behavior;
- address derivation;
- signing message consistency;
- signature verification;
- ownership proof output;
- safety language.

---

## Internal Assessment

The wallet module is the highest-risk area for user misunderstanding.

The MVP wallet is useful for local proof testing, but it must not be described as production custody software.

---

## Positive Checks

- Wallet output includes experimental warning language.
- Contributor guide forbids production custody claims.
- Security docs warn against valuable asset usage.
- Ownership proof is described as local key-control proof only.

---

## Risks

- Local key material may be stored in local files.
- Users may misunderstand address output as real asset custody.
- MVP wallet does not include production key management.
- No encrypted wallet storage is guaranteed at this stage.

---

## Internal Outcome

```text
Status: acceptable for local MVP testing only, not production custody
```

---

## Required Wording

Use:

```text
experimental local MVP wallet
```

Do not use:

```text
secure wallet
production wallet
safe custody
real PRM wallet
```

---

## Safety Statement

The MVP wallet must not be used with valuable assets.
