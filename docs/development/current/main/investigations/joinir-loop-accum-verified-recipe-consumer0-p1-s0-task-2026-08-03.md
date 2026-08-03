---
Status: Closed caller-zero test-only task
Date: 2026-08-03
Decision: accepted — `JOINIR-LOOP-ACCUM-VERIFIED-RECIPE-CONSUMER0-P1-S0`
Scope: prove the physical allocation and PHI define-before-use seam for one
       DirectAccum recipe, without operation cutover
Related:
  - joinir-loop-accum-verified-recipe-consumer0-p1-d0-task-2026-08-03.md
  - joinir-loop-phi-materializer-m6b-design-2026-08-03.md
  - ../design/phi-lifecycle-ssot.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
---

# Accum verified-recipe consumer P1-S0

## Boundary

This is the first implementation slice after the design stop. It proves the
ordering and ownership seam in a caller-zero test module. It does not emit the
complete portable Loop body and does not switch `route_loop`.

```text
verified recipe + JoinSig + sealed physical paths
  -> Builder-free physical role/operation schedule
  -> candidate-only block/value reservations
  -> existing PHI lifecycle begin/finalize boundary
```

The legacy composer/PlanLowerer remains an oracle in a separate test helper.
Generic/D2, Retry, route selection, nested predicate loops, after-value
publication, and production candidate commit are outside this row.

## Result

P1-S0 is closed with a test-only role-plan child and candidate reservation
product. The DirectAccum witness now proves the explicit `P/H/B/S/A` edge
paths, alpha-stable candidate-only block/result reservation, and the
reserve -> provisional PHI define -> abort ordering. The two-phase handle
owns the existing `PhiTxn`; it does not add a PHI/SSA writer or a production
caller. Focused materializer tests (19/19), `cargo check`, the current-state
pointer guard, and the in-place replacement guard are green.

## Required products

1. `PhysicalRolePlanV1` (Builder-free, non-Clone): canonical `P/H/B/S/A`
   roles, explicit P1b edge paths, predecessor seal, and ordered recipe
   operation keys. It consumes verified products only and contains no AST,
   CorePlan, MIR IDs, or route names.
2. `PhysicalAllocationV1` (candidate-local, non-Clone): one reservation pass
   for physical blocks and operation result IDs. Reservation is the only
   candidate mutation in this product; it emits no instruction, PHI, Binding
   SSA fact, or publication.
3. `LoopPhiMaterializationHandleV1`: a bounded extension of the existing
   `LoopPhiMaterializerV1` that owns the same `PhiTxn` across begin/finalize.
   Within this caller-zero M6-B test seam, the handle is the only Loop-level
   PHI caller; `phi_lifecycle` remains the sole low-level writer. This is not
   a final semantic PHI authority: production Loop lowering must consume the
   function-owned Binding SSA owner before any cutover claim.

This does not establish a new PHI/SSA SSOT. The repository SSOTs remain
`../design/phi-lifecycle-ssot.md` and
`../design/binding-ssa-first-control-lowering-ssot.md`; this row only proves
that the verified-recipe seam can consume those owners without bypassing
them.

## Ordered slices

1. Verify the DirectAccum recipe/JoinSig/path witness can produce a complete
   role plan without reading AST/CorePlan or allocating physical IDs.
2. Reserve the candidate blocks/operation result IDs once and assert that the
   live external Builder is unchanged. Re-running from a fresh candidate must
   produce the same alpha-normalized role/allocation shape.
3. Begin the PHI handle, prove the carrier destinations are defined before a
   simulated `ReadBinding` consumption, and finalize/abort through the same
   `PhiTxn`. No direct `insert_phi`/`update_phi` call is allowed.
4. Add focused guards for caller-zero, non-Clone products, no raw IDs in the
   Builder-free role plan, and the 800-line limit.

## Acceptance

- one focused P1-S0 test proves reserve → provisional PHI define → abort or
  finalize ordering;
- all P1b/P4-S0 tests and PHI/SSA guards remain green;
- no complete operation lowerer, production physicalizer, `route_loop` caller,
  Retry/fallback, or candidate publication is added;
- every touched Rust file stays below 800 lines.

## Stop conditions

Stop and return to P1-D0 if this slice needs AST reconstruction, CorePlan
consumption, a second PHI/SSA writer, binding-map duplication, or instruction
emission before the allocation/PHI ownership contract is proven.
