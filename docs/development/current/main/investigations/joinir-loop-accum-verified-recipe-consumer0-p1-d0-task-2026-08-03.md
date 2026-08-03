---
Status: Accepted design / caller-zero task ready
Date: 2026-08-03
Decision: accepted after worker consultation — `JOINIR-LOOP-ACCUM-VERIFIED-RECIPE-CONSUMER0-P1-D0`
Scope: caller-zero Accum physicalizer seam after M6-A/B and P4-S0 evidence
Related:
  - joinir-loop-accum-verified-recipe-consumer-p1-design-2026-08-03.md
  - joinir-loop-phi-materializer-m6b-design-2026-08-03.md
  - joinir-loop-accum-mir-physical-snapshot0-m5-p4-s0-task-2026-08-03.md
  - ../design/joinir-loop-selfhost-recipe-pipeline-ssot.md
  - ../design/phi-lifecycle-ssot.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
---

# Accum verified-recipe consumer: physicalizer design stop

## Why this is the next hard boundary

P4-S0 now observes the existing legacy Standard5 MIR without creating a second
producer. The remaining clean-pipeline step is a caller-zero physicalizer that
consumes the verified portable recipe and mutates only an unpublished compile
candidate. This is the first point at which a new operation/CFG producer could
accidentally become a second PlanLowerer or a second PHI authority, so it is a
design stop before implementation.

The target flow is:

```text
VerifiedLoopRecipeV1
  -> VerifiedLoopJoinSigV1
  -> explicit physical CFG/path skeleton
  -> verified operation/value schedule
  -> candidate MIR emission
  -> existing PhiTxn / Binding SSA bridge
  -> terminal LoopPhysicalSuccessV1 or Freeze
```

No production `route_loop` caller is authorized by this card. The existing
`RecipeComposer`/`CorePlan`/`PlanLowerer` path remains a legacy parity oracle.

## Design questions that must be closed

### 1. Physical skeleton ownership

The portable JoinSig keeps the logical `Body -> Header` backedge. The physical
owner must consume the sealed P1b path witness and issue `Body -> Step ->
Header` blocks/terminators without adding a `Step` field to the semantic recipe.
It must also seal predecessor rows before the first Builder effect.

### 2. Logical operation/value mapping

`ReadBinding` is an alias to the current Binding SSA value, not a new MIR
definition. `WriteBinding` updates the physicalizer's local binding schedule.
Operation results are allocated exactly once in recipe block order. The map for
operation results is distinct from the M6-B edge payload/destination map; it
must not silently grow that PHI-only map into a general second plan.

### 3. PHI define-before-use ordering

Loop body operations need the current carrier PHI values before the body is
emitted, while PHI inputs are known only after the body/step values exist. The
consultation must choose one ownership-preserving solution:

- extend `LoopPhiMaterializerV1` with a two-phase begin/finalize handle that
  owns the same `PhiTxn`; or
- prove a different ordering that never exposes a reserve-only PHI and does
  not call PHI lifecycle APIs outside the existing materializer.

Direct physicalizer calls to low-level PHI insertion are forbidden. A second
PHI/SSA writer is a hard stop.

### 4. Candidate and failure boundary

The physicalizer may dirty the current compile candidate after qualification.
Any lower/verification error is terminal `Freeze`; it must never return
`Option`, Retry, or a next-route callback. The whole unpublished candidate is
dropped, and the already-existing compile-session fresh-reuse proof owns the
rollback evidence.

## Required consultation output

Before code, a worker must return:

1. one selected owner for the physical skeleton and operation schedule;
2. the exact two-phase PHI lifecycle boundary, or a proof that it is not needed;
3. the smallest test-only API/product list, with each product's input/output
   and forbidden dependencies;
4. a slice order that keeps every Rust file below 800 lines;
5. caller-zero gates and the first permitted M10a production bridge;
6. explicit non-claims for Generic/D2, nested predicate loops, full all-route
   parity, and legacy retirement.

## Selected design (closed 2026-08-03)

The worker consultation selected one ownership-preserving sequence:

```text
VerifiedRecipe / JoinSig / sealed paths
  -> Builder-free PhysicalRolePlanV1
  -> candidate-local PhysicalAllocationV1
       (blocks and ValueIds reserved; no instructions/PHI/Binding writes)
  -> LoopPhiMaterializerV1 two-phase handle
       (the handle owns the existing PhiTxn; no low-level caller escapes)
  -> Binding SSA-backed operation emission
       (ReadBinding aliases the current binding; WriteBinding updates it)
  -> handle patch + commit
  -> CFG/SSA/type/result verification
```

The M6-B PHI adapter remains the only Loop-level PHI entry inside the
caller-zero test seam. Its next bounded extension is a begin/finalize handle
around the same `PhiTxn`; the physicalizer does not call `phi_lifecycle`
directly. This adapter is a mechanical migration observer, not the final
semantic PHI authority: production Loop lowering must consume the
function-owned Binding SSA owner before any cutover claim. The operation-result
map is a short-lived projection distinct from the M6-B edge payload/destination
map and never becomes a second Binding SSA owner.

The first implementation task is
`joinir-loop-accum-verified-recipe-consumer0-p1-s0-task-2026-08-03.md`.

## Stop conditions

Stop and revise the design if the proposed implementation:

- reconstructs AST or `CorePlan` in the portable consumer;
- makes PlanLowerer a portable production backend;
- adds a route/family switch inside the physicalizer;
- mixes operation-result mapping into the PHI-only M6-B map;
- exposes a reserve-only PHI to Binding SSA or body emission;
- mutates a live external Builder or publishes partially;
- adds Retry/fallback/`Ok(None)` after the first physical effect.

## Acceptance for this design row

- the worker consultation closes the four ownership questions above;
- P4-S0's legacy snapshot and all existing PHI/SSA guards remain green;
- no production caller or new physical mutation is added by this row;
- the next implementation row is a single bounded caller-zero Accum slice,
  not a route-by-route physicalizer series.
