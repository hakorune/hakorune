# Callable Loop Production Full-Demand Preflight S2

Status: `closed` (2026-08-08).
Parent: `CALLABLE-LOOP-PRODUCTION-PREPARED-INGRESS-S1`.

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

- [x] complete callable operation/effect coverage is verified with Builder
  effect equal to zero;
- [x] Recipe structure, not a sorted evidence index, owns operation order;
- [x] missing, duplicate, foreign, unsupported, or unconsumed evidence returns
  a typed `NoSafeSlice`/contract rejection before a physical session opens;
- [x] no `first`, `select`, `filter`, or partial-operation extraction API
  exists;
- [x] the prepared ingress remains move-only and is consumed exactly once;
- the raw host, production selector, physicalizer, Generic G0 parity,
  retry/fallback, and legacy edges remain unchanged;
- [x] focused tests, rustfmt/check, current-state guard, and replacement guard
  are green;
- [x] implementation closeout updates the applicable `docs/reference/**`
  contract only if a reference claim changes, and updates current/task mirrors
  in the same commit.

## Stop

Do not allocate physical blocks, emit MIR, open a function session, lower a
leaf operation, claim Tail/ABI/Completion, switch a production caller, enable
Generic G0 parity, add fallback/retry, or delete legacy edges. If the complete
demand cannot be prepared without re-reading AST/name or inventing a second
owner, return `NoSafeSlice` and reopen design.

## Implementation receipt

`PreparedCallableLoopIngressV1::prepare_full_demand` is now the single
callable full-demand entry. It consumes the ingress once, uses the existing
callable source/effect adapter to issue one neutral full-program demand, and
then calls `VerifiedLoopOperationPhysicalDemandV1::prepare_all`. The resulting
thin profile product retains the source receipt, input relation, Prelude, and
Tail while the common program owns Recipe/JoinSig, operation/effect evidence,
semantic context, and Loop continuation.

The S2 implementation adds no Builder/session effect and no physical IDs. The
existing callable fixture proves the seven-row Recipe-order schedule and exact
coverage with the source/context, input, Prelude, and Tail owners aligned.
The adapter's test wrapper remains `cfg(test)`; production exposes only the
one-shot parts issuer required by this assembler. Physicalization, ABI,
Completion, selector, Generic G0 parity, retry/fallback, module publication,
and legacy deletion remain closed.

Reference obligation: the implementation changes no user-facing language
semantics. The MIR reference receipt and current mirrors are nevertheless
updated in this closeout so the Builder-free full-demand boundary and next
physicalizer stop are discoverable.
