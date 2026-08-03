# JOINIR-LOOP-RECIPE-D3-CANONICAL-PHYSICAL-PILOT

Status: design stop opened after D2-D semantic parity closure.
Date: 2026-08-03

## Decision

The first physical pilot is the existing singleton DirectAccum/AccumConstLoop
product, not the newly produced Nested Predicate Recipe. D2-D remains a
semantic parity witness until a separate nested physical topology is sealed.

```text
VerifiedDirectAccumRecipeProductV1
  -> VerifiedLoopPhysicalInputV1(recipe + JoinSig)
  -> owner-issued binding/input projections
  -> Standard5 VerifiedLoopPhysicalRolePlanV1
  -> existing direct-accum physicalizer
```

This is an owner-reuse proof for the existing canonical physical session, not
a PHI writer rewrite or an all-route cutover.

## Existing SSOT chain

The caller opens one unpublished `ModuleBuilderInvocationSessionV1` candidate
and creates one `CanonicalSsaFunctionSessionV2`. That owner contains or lends
exactly one:

```text
CanonicalCfgSessionV1
BindingSsaBuilderV1<PhiToken>
PhiTxn
```

- CFG edges, block seals, and predecessor witnesses belong to the canonical
  CFG session.
- ReadBinding/WriteBinding use the function-owned Binding SSA owner.
- PHI Reserve/Define/Populate/Seal/abort/commit use the existing `PhiTxn`
  lifecycle adapter only.
- `LoopPhiMaterializerV1` and route-specific materializers remain
  mechanical-observer or legacy surfaces; the pilot must not call them.

On success the caller orders identity finish, CFG/semantic/if finish,
`PhiTxn.commit`, and candidate completion. On any failure, `PhiTxn.abort` is
followed by dropping the whole unpublished compile candidate; the live
`MirBuilder` is unchanged.

## Non-goals

- Do not physicalize D2-D Nested Predicate yet. Its root/child ports,
  child-after/resume path, predecessor witnesses, carrier destinations, and
  Standard5 topology are not sealed by JoinSig alone.
- Do not infer blocks or PHI destinations from semantic JoinSig rows.
- Do not add route selection, `Option`/Retry continuation, Generic handling,
  legacy composer/CorePlan/PlanLowerer calls, candidate publish, final-result
  mapping, or raw PHI-writer retirement.
- Do not create a second PHI/SSA owner or modify the existing SSOT docs.

## Caller-zero gate

The pilot must keep these production counts at zero:

```text
route_loop / scheduler callers       = 0
legacy composer/CorePlan/PlanLowerer = 0
Retry/Option/fallback                 = 0
raw route PHI writers                 = 0
phi_input_materializer                = 0
live MirBuilder direct callers        = 0
```

Tests may create local canonical owners and explicitly exercise finish/abort.

## Acceptance gates

1. Existing DirectAccum recipe/JoinSig product reaches the physical input
   projection and Standard5 role plan without source reread or route choice.
2. A success test proves the candidate-local CFG/SSA/PhiTxn path and semantic
   result parity; the live builder is published only by the existing outer
   candidate contract.
3. Failure injection after first PHI/CFG effect proves `PhiTxn.abort` plus
   candidate drop and fresh-request reuse; no live builder mutation remains.
4. Existing `phi_lifecycle`, DirectAccum candidate hardening, Recipe, and
   scope guards remain green. No Nested Predicate production caller is added.
5. All touched Rust/test files remain below 800 lines.

After this pilot closes, open a separate design card for Nested Predicate
physical topology. Do not use the pilot as evidence that all Loop families are
physically cut over.
