---
Status: Active design stop
Date: 2026-08-03
Decision: accepted design boundary — `JOINIR-LOOP-ACCUM-VERIFIED-RECIPE-CONSUMER0-P1-S1-D0`
Scope: define the next caller-zero DirectAccum operation-emission slice without
       widening the PHI/SSA authority or connecting a production route
Related:
  - joinir-loop-accum-verified-recipe-consumer0-p1-d0-task-2026-08-03.md
  - joinir-loop-accum-verified-recipe-consumer0-p1-s0-task-2026-08-03.md
  - ../design/phi-lifecycle-ssot.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
---

# Accum verified-recipe consumer P1-S1 design stop

## Decision

P1-S1 remains a test-only DirectAccum candidate slice. It does not create a
second Binding SSA builder, PHI writer, route selector, retry path, or
production caller. The existing PHI and Binding SSA documents remain the
authorities. The recipe consumer only consumes their APIs through a sealed
candidate plan.

```text
VerifiedLoopRecipe + JoinSig + P1-S0 role plan
  -> VerifiedLoopOperationScheduleV1 (Builder-free)
  -> BindingReadResolutionV1 (borrowed Binding SSA receipt/alias only)
  -> OperationReservationV1 (typed Const/Binary/Compare result IDs only)
  -> sealed OperationEmissionPlanV1
  -> test-only emitter on unpublished candidate
  -> existing PHI handle finalization / existing Binding SSA checks
```

`ReadBinding` is an alias lookup through an existing Binding SSA receipt or
capability and must not allocate a new MIR value. `BindingReadResolutionV1`
is therefore a test-only projection/plan record, not a new binding-resolution
authority or name-keyed map. If the current Binding SSA API cannot provide a
borrowed receipt yet, stop at that API seam instead of inventing one in the
recipe consumer.
`WriteBinding` advances only an ephemeral emission cursor in this slice; it
does not publish a binding definition. `ConstI64`, `BinaryI64`, and
`CompareI64` use the canonical builder effect path and return typed errors.

## PHI/SSA boundary

The repository already has the PHI/SSA SSOTs:

- `../design/phi-lifecycle-ssot.md` — `PhiTxn` is the sole provisional-PHI
  lifecycle writer.
- `../design/binding-ssa-first-control-lowering-ssot.md` — one
  function-owned `BindingSsaBuilderV1` owns binding reaching definitions.

P1-S0 proved a two-phase wrapper around the existing `PhiTxn`, but it did not
make a new authority. P1-S1 must preserve that fact. In particular, do not
instantiate a second `BindingSsaBuilderV1` inside the recipe consumer and do
not call raw PHI insertion/update APIs.

## Ordering constraint

The current production-free `LoopPhiMaterializerV1::begin` performs incoming
value/type/dominance preflight before defining provisional PHIs. A real loop
body cannot use that entry point before its operation definitions exist. The
P1-S1 test slice therefore needs a distinct candidate-only handle (or an
explicitly named candidate-begin seam) that:

1. validates the sealed role/predecessor shape;
2. defines provisional header PHIs through the existing `PhiTxn` boundary;
3. emits the sealed operation plan;
4. runs the existing preflight and patches/commits the same transaction.

This is an API design stop, not permission to bypass `PhiTxn` with direct
mutation. If the existing transaction API cannot express this ordering
without widening its authority, stop and reopen the design instead of adding
an adapter around raw MIR instructions.

## Required products (next implementation card)

1. `VerifiedLoopOperationScheduleV1`: Builder-free ordered operation keys with
   physical block roles and no AST/CorePlan/route data.
2. `BindingReadResolutionV1`: test-only borrowed projection of an existing
   Binding SSA receipt/capability keyed by `BindingRefV1`; no new resolution
   authority and no name-keyed map.
3. `OperationReservationV1`: candidate-local typed result reservations for
   Const/Binary/Compare; Read/Write reserve nothing.
4. `OperationEmissionPlanV1`: sealed operation inputs/outputs and block roles.
5. A test-only candidate emitter and candidate-PHI ordering handle, followed
   by one structural alpha digest.

## Acceptance gates

- reservation is alpha-stable and emits no MIR;
- ReadBinding emits no new value and consumes only its resolved alias;
- operation kinds, block roles, types, and operands match the DirectAccum
  physical witness;
- PHIs are finalized only after operation definitions exist;
- injected late failure aborts the candidate and leaves the live builder
  fingerprint unchanged; a fresh session remains reusable;
- static guard proves caller-zero, no direct PHI API, no second SSA builder,
  no route/retry/PlanLowerer dependency, and all touched Rust files remain
  below 800 lines.

## Explicit non-claims

This row does not claim production recipe consumption, route cutover,
Generic/nested/all-family parity, post-effect retry removal, final binding
publication, or selfhost ownership of Loop PHI/SSA. Those remain later gates.
