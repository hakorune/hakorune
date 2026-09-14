---
Status: open__design_stop__2026-09-14
Task: MIR-CALL-RESOLVER-IF-EXPRESSION-CONTRACT-D0
Date: 2026-09-14
Priority: define the missing source-to-Recipe contract for expression If results
Parent: mir-call-resolver-if-expression-expressivity-d0-2026-09-14.md
NextCard: TBD after source/result contract decision
Implementation permission: false until condition, branch values, result class, and one Recipe/JoinSig issuer are co-sealed
---

# Expression If contract design stop

## Six-line brief

```text
Decision: keep all five resolver expression-If rows at NoSafeSlice while defining one typed contract for condition, both branch values, result class, and source body identity.
Source authority + canonical issuer: ShadowResolverV0 supplies source/site facts; a future expression-If Recipe producer must co-seal the result relation with the existing source-backed callable package.
Non-authority: AST re-reading, ternary text rewriting, names/arity, MIR inspection, VM compatibility, fallback, and a second expression matcher.
Fail-fast boundary: missing condition, branch, type, result, JoinSig, or source-site relation remains SelectedCallableResolverDeferredBatchV1 before Recipe issuance.
Smallest next slice: enumerate i64, f64, and String branch/result shapes and determine whether one existing If control owner can issue a typed result without changing statement-If semantics.
Non-claims: resolver acceptance, MIR lowering, package publication, A3 handoff, StringBox fixes, VM parity, or phase14 completion.
```

## Required co-sealed relation

The contract must carry, in one source-backed issuance:

1. the callable/body identity and exact expression source path;
2. the If condition fact and its boolean result relation;
3. both branch value facts, including their type/ABI class;
4. the result destination/consumer relation (`Return.Value`, initializer, or
   RHS) without inferring it from a later MIR ValueId; and
5. one continuation/JoinSig that preserves branch exit and result merge.

The result classes observed by the finite inventory are i64, f64, and String.
String must remain explicit: no integer zero/default or opaque text fallback
can stand in for a missing branch result.

## Ordered design tasks

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | Source census | Record exact source path, condition shape, then/else value shape, result class, and consumer position for all five rows. |
| 2 | Owner map | Compare the existing `resolved_region_flow`/`if_control` statement owner and source-backed callable package; identify one producer or prove that a new owner is required. |
| 3 | Recipe/JoinSig decision | Name the existing Recipe/JoinSig/result issuer or keep `NoSafeSlice` with the missing capability explicitly recorded. |
| 4 | Negative boundary | Fix rejects for missing else/value, mixed result classes, unsupported nested/control shapes, and non-source-backed bodies. |
| 5 | Guard plan | Design positive/negative/terminal guards only after the issuer exists; do not add a resolver-positive test during this design stop. |
| 6 | Follow-up selection | Select a bounded i64 subset only if it shares the same co-sealed contract; otherwise split by result class without widening the five-row inventory. |

## Stop conditions and nonclaims

Remain at `NoSafeSlice` if the contract needs AST rewrite, by-name matching,
later MIR re-inference, a default branch value, a second resolver, fallback,
or a compatibility downgrade. This card does not authorize changes to the
five Hako helpers, statement-If lowering, StringBox, publication, VM, or C.
