---
Status: open bounded resolver-only implementation
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-callable-body-conformance-evidence-d0-design-task-2026-08-09.md`
Authority: `docs/reference/language/callable-contracts.md`
---

# CALLABLE-BODY-EFFECT-CONTROL-I0

## Decision

The general body-execution evidence D0 is accepted.  The next implementation
is deliberately one finite resolver-only witness; it is not general
conformance and does not open the target/Recipe/Builder/MIR lane.

```text
ResolvedFunctionBodyShapeProductV1
  -> private BodyEffectControlCoverageIssuerV1
  -> non-Clone BodyEffectControlCoverageReceiptV1
```

The issuer has one canonical entry:

```rust
BodyEffectControlCoverageIssuerV1::issue(
    product: &ResolvedFunctionBodyShapeProductV1,
)
```

Function and body shape are never passed as separate arguments and are never
re-paired by name, ordinal, or owner number.  The receipt borrows the existing
product and does not copy shadow vectors or create a second semantic owner.

## Finite admitted cohort

The only positive shape is an exact root-direct body equivalent to:

```hako
return me.invoke()
```

It must contain exactly:

```text
MethodCall + Me + ordinary Return
one Call effect at the MethodCall site
one ExplicitReturn exit targeting the same function region
the exact Return->value and MethodCall->receiver relations
receiver BindingRef owned by the same function and typed as Receiver
```

The method name is not semantic authority; `invoke` is only the fixture name.

## Fail-fast boundary

```text
NoSafeSlice:
  Other/opaque expression, SequenceItem/Print, Await, Break, Continue,
  If/Loop, field access, write, allocation, nested owner, or unsupported
  transfer because this bounded issuer does not cover that vocabulary

Rejected:
  foreign product/owner/root/site, duplicate or missing relation/effect/exit,
  wrong return target region, foreign receiver binding, or cardinality mismatch
```

No public Candidate/Declined/Unresolved source disposition is issued by this
private finite receipt.  `NoSafeSlice` is development state only.

## Required tests

1. Positive exact `return me.invoke()` issues one receipt with Call+Return.
2. `return 0` remains `NoSafeSlice` because its expression is `Other`.
3. Print/empty/field/write/alloc/await and unsupported exits remain
   `NoSafeSlice`; they are not silently treated as no-effect.
4. A foreign or mismatched product is `Rejected`.
5. Existing bounded `return me` Query evidence and conformance tests remain
   unchanged.

## Non-claims

This slice does not issue or prove Home flow, Pure, Query conformance,
general effect/control completeness, target lookup, source-bound calls,
Recipe/CallSlot, FunctionOwner creation, ABI, Builder, MIR, publication,
retry, fallback, or production selection.

## Closeout

The same implementation commit must update this card, the body evidence D0
receipt, `src/mir/resolved_semantics/README.md`, and
`docs/reference/language/callable-contracts.md`, then run the focused resolver
tests and the current-state pointer guard.
