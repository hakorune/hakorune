# DYNAMIC-OPERATOR-EXECUTION-CONTRACT-I0

Status: closed; canonical atomic Add/Less envelopes are live
Date: 2026-08-10
Reference: `docs/reference/language/dynamic-operators.md`

## Change

Add one profile-neutral, atomic Dynamic operator semantic issuer. Callers may
receive only the complete sealed envelope; no partial effect, operand,
outcome, or lifecycle constructor is public.

```text
DynamicAdd(Dynamic, I64):
  OpaqueObservable / SynchronousNonDetached / MaySuspend
  ExpressionBounded / BorrowedNoEscapeForOperation
  Normal(SelfContainedNonAliasingDynamicCarrier) | Fault(TypeError)
  EndExactlyOnceUnlessForwarded on Normal only

DynamicLess(Dynamic, Dynamic|I64):
  same execution and operand law
  Normal(TrivialBool) | Fault(TypeError)
  no carrier lifecycle
```

`SelfContainedNonAliasingDynamicCarrier` rejects a result that aliases either
operand. Fault publishes no result, changes no operand lifecycle, performs no
rebind, and never retries or falls back.

## Acceptance

- issuer and negative matrix cover wrong domain, partial axis, aliasing result,
  moved operand, Fault publication/mutation, lifecycle mismatch, and reuse of
  the Dynamic invocation envelope;
- source spelling plus verified operand classes are the semantic input;
- Recipe class, runtime tag, provider, selector, VM behavior, and `MirType` are
  non-authority;
- the exhaustive V2 operation/Fault projection is reused or closed before any
  new operation variant; no `_ => continue` classifier remains;
- no source/Recipe co-seal, V9/V17 destination row, Home, CFG, or physical
  projection is added in this row.

## Closeout

`dynamic_operator_contract` now issues three module-owned envelopes for exact
Add/Less domains. Unsupported domains reject without fallback. Add owns the
non-aliasing carrier result plus shared lifecycle; Less owns trivial Bool and
no carrier lifecycle. The V2 operation schema now exposes one exhaustive
execution-class projection, and the Fault catalog no longer has a wildcard
skip. Focused operator and semantic-program tests plus `cargo check --lib` are
green; unrelated unused-import warnings remain baseline evidence.

Classified parent-baseline red: at parent `271c73e0f7`, the exact test
`invocation_lifecycle_recipe_relations_reject_duplicates_and_wrong_boundaries`
already returns `RecipeRelation` before its expected `InvocationCoverage`.
The same isolated failure remains outside this row; the three Fault-catalog /
semantic-program tests selected by this change and the operator matrix are
green. Its owner is the existing invocation-lifecycle test-helper ordering,
not the operator envelope or exhaustive Fault projection.
