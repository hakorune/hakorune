# Callable Loop Production Prepared-Ingress Assembly S1

Status: `closed` (2026-08-08).

## Change

Assemble one move-only prepared callable ingress from the closed source-loan
receipt and the already-closed callable logical products. Keep the product
Builder-free and profile-neutral; this row only proves that the exact source,
logical Recipe/JoinSig/After, and existing callable boundary facts can be
carried together once.

## Contract

```text
source receipt
  + existing callable logical issuer products
  -> PreparedCallableLoopIngressV1
```

The prepared product may borrow or move only existing verified owners. It must
not rewalk AST, resolve names/arity, synthesize an ABI/header, issue a second
semantic owner, or create `ValueId`/`BasicBlockId`/Builder/session state.
Callable index/header and target ABI remain optional profile companions; they
are not required by the common source ingress. The product is single-use and
must reject foreign owner/source/function/Recipe/JoinSig/After combinations
before any Builder effect.

## Done

- one non-`Clone` prepared ingress is assembled from the source-loan receipt
  and one existing callable logical product;
- owner, origin/source-kind, Loop site, frame, scope, Recipe component, and
  JoinSig/After component ownership are checked at the assembly boundary;
- the S0 source receipt retains the exact function/forest/projection proof;
  S1 consumes that receipt without re-walking or re-resolving it;
- common ingress succeeds without an index/header companion, while the
  optional profile companion remains outside this row;
- a foreign logical owner is rejected before any Builder effect;
- no Builder, physicalizer, selector, production caller, Generic G0,
  retry/fallback, or legacy edge changes occur;
- focused tests, rustfmt/check, current-state guard, and replacement guard are
  green;
- the same commit updates the applicable `docs/reference/**` contract only if
  a reference claim changes, and updates diagnostics/migration/current task
  pointers as required. No language/reference claim changed in this row.

## Stop

Do not emit CFG/SSA/PHI/MIR, open a physical session, create a physical block
receipt, lower an operation, claim Tail/Completion, select a RoutePlan, switch
a production caller, enable Generic G0 parity, add fallback/retry, or delete
legacy edges. If the existing logical products cannot be joined by exact
identity, return `NoSafeSlice` and reopen design; do not add an AST adapter or
resolver.
