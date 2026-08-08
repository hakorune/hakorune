---
Status: active — bounded parser implementation
Date: 2026-08-09
Decision: implement typed `CallableContract(query)` syntax carriage only
Parent: `callable-contract-typed-syntax-carriage-d0-design-task-2026-08-09.md`
---

# CALLABLE-CONTRACT-TYPED-SYNTAX-CARRIAGE-I0

## Scope

Carry the parser-validated source spelling
`@rune CallableContract(query)` from the ordinary Box method source seal as a
parser-private typed syntax row. The explicit method source site remains the
identity; the typed row carries only the declaration-local rune ordinal and
the `Query` variant.

```text
Rust parser rune validation
  -> explicit method source relation
  -> CallableContractSyntaxV1::Query
  -> parser-owned source seal
```

The Hako parser keeps the same accepted name/value contract in its normalized
rune helper. It does not become a resolver or semantic-contract issuer in
this row.

## Allowed changes

1. Add `CallableContract` to Rust and Hako parser syntax validation.
2. Reject wrong placement, duplicate rune, and unknown value at the parser
   boundary. `Contract`/`Profile` semantic conflicts remain a later issuer
   responsibility and are not inferred by this syntax row.
3. Add a small parser-private typed syntax module; do not grow the large source
   authority module with a second semantic owner.
4. Carry the typed row through explicit method source relations and the rich
   parser seal without using inventory ordinals as source identity.
5. Add focused parser/source-seal tests and update the parser README and
   `docs/reference/language/callable-contracts.md` in the same closeout.

## Acceptance

```text
positive: one instance method preserves Query + exact method source site
negative: unknown value, duplicate CallableContract, wrong placement
parity: Rust and Hako rune helpers accept only query
guard: source seal remains the only parser authority
budget: every changed Rust source file stays below 760 lines
```

## Nonclaims

```text
no VerifiedDeclaredCallableContract
no semantic signature or I64 meaning
no Home ABI / effect / suspension / control / physical ABI
no resolver target or source-bound call relation
no Recipe/CallSlot, Builder/MIR/CFG/PHI, ScanWithInit, or lowering
no provider/runtime, production selection, fallback, retry, or retirement
```

## Closeout requirement

Implementation, focused tests, parser README, language reference, current
pointer, and this card's landed receipt must be closed in the same commit and
pushed. If the rich parser seal cannot carry the exact method/rune structural
site without reconstructing it from inventory placement, stop with
`NoSafeSlice` and do not synthesize a fallback.
