---
Status: active design stop after full-body source I0
Date: 2026-08-10
Row: `SOURCE-BOUND-DYNAMIC-METHOD-DISPATCH-D0`
Parent: `generic-loop-dynamic-full-body-closure-d0-task-2026-08-10.md`
Mode: BoxShape / compiler acceptance repair
---

# Source-bound Dynamic method dispatch contract

## Decision boundary

The prerequisite `GENERIC-LOOP-DYNAMIC-FULL-BODY-COVERAGE-I0` is closed. Its
source inventory retains both exact MethodCall syntax rows without assigning
receiver/result semantics. This D0 is now the only open boundary; no Recipe
or physical implementation proceeds until the five questions below are
closed.

The unchanged production `ParserScanLoopBox.skip_while/4` contains:

```hako
src.substring(i, i + 1)
pred_chars.indexOf(ch)
```

Both receivers are source-backed `Dynamic`. The current resolver target
catalog admits exact declared instance/static targets only; it cannot honestly
mint an exact declaration target for either call. Treating the method spelling
as a Box/type classifier, fabricating an instance target, or falling through to
the legacy Builder writer is forbidden.

The missing compiler authority is a route-disjoint arm in the one source-call
target catalog:

```text
exact declared receiver
  -> exact declaration target capability

source-backed Dynamic receiver
  -> opaque DynamicMemberTarget
  -> exact source-bound Dynamic dispatch contract
```

This row designs the second arm. It does not weaken the first arm, create a
second target catalog, or put runtime dispatch identity into the neutral
Recipe wire.

## Required semantic product

The selected design must issue one non-`Clone`, AST-free relation per exact
source call site. Conceptually:

```text
VerifiedSourceBoundDynamicMethodDispatchV1
  exact function owner / source call site
  source-backed Dynamic receiver relation
  ordered source argument relations
  source method symbol + arity as the runtime dispatch key
  Dynamic result class
  declared effect/control envelope for this dispatch form
  same source/resolver provenance
```

The method spelling is only the as-written runtime dispatch key. It is not
authority for receiver Box/type, result refinement, Home, provider identity,
physical ABI, backend route, or special compiler behavior.

The Recipe `CallSlot` keeps only receiver/argument/result logical keys. The
source-bound relation set, co-sealed beside the Recipe, owns the exact dispatch
contract for each CallSlot.

## Source authority inventory

Existing reusable evidence:

```text
call syntax and exact source sites:
  ResolvedFunctionBodyShapeProductV1

receiver/argument BindingRefs and source membership:
  CallableSourceLedgerV1

source-backed Dynamic origin:
  SourceBackedDynamicCallableIssuerV1
  CallableDynamicOriginLoweringStateV1 (migration physical witness only)
```

Missing canonical issuer:

```text
source call site
+ Dynamic receiver origin
+ ordered argument source relations
+ method symbol/arity dispatch key
-> one verified Dynamic dispatch relation
```

The issuer belongs in a neutral pre-Builder source/semantic layer. Builder
modules may consume the product later but must not become its semantic owner.

## Outcome rules

```text
Candidate:
  exact supported source MethodCall, Dynamic receiver origin, complete ordered
  argument relations, exact source membership, and one result relation

Declined:
  completely observed call is not the Dynamic-dispatch family

Unresolved:
  source/resolver evidence required to decide the source call is unavailable

Rejected:
  foreign owner/provenance, duplicate call site, missing/duplicate argument,
  receiver-origin mismatch, or contradictory result relation

NoSafeSlice:
  the canonical issuer itself is not implemented; this is a development state,
  not a source disposition
```

## D0 questions to close

1. Which existing resolver source product owns ordered argument membership for
   an arbitrary direct method-call expression?
2. Which neutral semantic type owns the Dynamic result without using
   `MirType::Unknown` as reverse inference?
3. Which effect/control envelope is honest for generic Dynamic dispatch before
   an exact runtime target is selected?
4. How is one relation set co-sealed to Recipe CallSlots without copying
   owner/block placement already derivable from Recipe membership?
5. Which runtime plan consumes the dispatch key exactly once without call-time
   compiler fallback or provider reselection?

If any answer lacks a canonical issuer, the implementation row stops
`NoSafeSlice`; the production source is not narrowed.

## Implementation row

`SOURCE-BOUND-DYNAMIC-METHOD-DISPATCH-I0` must:

```text
issue exact relations for substring/2 and indexOf/1 in unchanged skip_while/4
retain source-backed Dynamic receiver/argument/result lineage
reject missing / duplicate / foreign / reordered source rows
perform zero Builder/MIR effect
expose no name-based semantic classifier
update owner README and public MIR reference in the same commit
keep every new source file below 800 lines
```

## Nonclaims

```text
no exact declaration target for a Dynamic receiver
no Text/String refinement
no method-name special case
no Recipe or CallSlot implementation
no Builder/MIR/CFG/PHI
no runtime invocation or provider selection
no retry/fallback
no source annotation/rewrite/narrow fixture
```
