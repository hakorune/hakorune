# Callable Loop Production Full-Demand Preflight S2

Status: next implementation row after
`CALLABLE-LOOP-PRODUCTION-PREPARED-INGRESS-S1` (2026-08-08).

## Change

Add the Builder-free full-demand preflight that consumes one prepared callable
ingress and verifies the complete existing logical operation/effect product
before any physical session opens. This row is the whole-program counterpart
to the already-closed S1 assembly; it must not introduce a single-operation
selection API or a second Recipe owner.

## Contract

```text
PreparedCallableLoopIngressV1
  + existing callable operation/effect product
  -> PreparedCallableLoopOperationProgramV1
```

The preflight consumes every operation exactly once according to the logical
Recipe structure and checks the existing source/effect evidence, value classes,
logical placement, and continuation compatibility. It produces no `ValueId`,
`BasicBlockId`, CFG/SSA/PHI state, ABI/Completion claim, selector, RoutePlan,
retry, fallback, or module publication.

## Acceptance

- complete callable operation/effect coverage is verified with Builder effect
  equal to zero;
- Recipe structure, not a sorted evidence index, owns operation order;
- missing, duplicate, foreign, unsupported, or unconsumed evidence returns a
  typed `NoSafeSlice`/contract rejection before a physical session opens;
- no `first`, `select`, `filter`, or partial-operation extraction API exists;
- the prepared ingress remains move-only and is consumed exactly once;
- the raw host, production selector, physicalizer, Generic G0 parity,
  retry/fallback, and legacy edges remain unchanged;
- focused tests, rustfmt/check, current-state guard, and replacement guard are
  green;
- implementation closeout updates the applicable `docs/reference/**` contract
  only if a reference claim changes, and updates current/task mirrors in the
  same commit.

## Stop

Do not allocate physical blocks, emit MIR, open a function session, lower a
leaf operation, claim Tail/ABI/Completion, switch a production caller, enable
Generic G0 parity, add fallback/retry, or delete legacy edges. If the complete
demand cannot be prepared without re-reading AST/name or inventing a second
owner, return `NoSafeSlice` and reopen design.
