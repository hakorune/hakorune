# JOINIR Direct Accum Structural Projection S0

Status: Design-approved; caller-zero implementation slice.

Task: `JOINIR-LOOP-DIRECT-ACCUM-STRUCTURAL-PROJECTION0-S0`

Depends on: `JOINIR-LOOP-DIRECT-ACCUM-STRUCTURAL-PROJECTION0-D1`

## Purpose

Issue the first real AST-free Direct Accum structural-facts capability while
keeping Recipe production, legacy route selection, Builder mutation, and
PHI/SSA production wiring at zero.

This is a contract-and-test slice. It proves exact source navigation,
resolved binding identity, and same-execution ownership. It does not lower MIR
or change the current scheduler.

## Authority path

```text
VerifiedResolvedSourceUnitV1
  -> FunctionSourceViewV1
  -> LocatedStmtV1 / LocatedExprV1
  -> DirectAccumObservedShapeV1       (AST-bearing adapter, one observation)
  -> VerifiedResolvedFunctionV1       (variable_ref / assignment_target)
  -> VerifiedDirectAccumFactsV1       (AST-free, sealed)
```

The topology adapter must navigate only through the existing shared roles:

```text
BodyChildRoleV1::LoopBody
ExprChildRoleV1::LoopCondition
ExprChildRoleV1::{AssignmentTarget, AssignmentValue}
ExprChildRoleV1::{BinaryLeft, BinaryRight}
```

No hand-written `SourcePathSegmentV1`, raw body index, pointer search, or name
lookup is allowed. `CanonicalLoopFacts` and `AccumConstLoopFacts` remain
legacy observation/parity oracles and are not neutral inputs.

## Capability contract

`VerifiedDirectAccumFactsV1` contains only:

```text
LoopExecutionFrameKeyV1
loop source identity
induction BindingRefV1
accumulator BindingRefV1
condition operator + i64 constant
accumulator update operator + i64 constant
induction step operator + i64 constant
fixed two-statement body order
private seal
```

`VerifiedResolvedLoopSourceV1` is the only issuer of
`LoopExecutionFrameKeyV1`. The key is opaque and compared by equality; its
private payload is not a route cursor or semantic route ID. The policy winner,
facts product, and source capability must carry the same key. The selected
demand issuer rejects a key mismatch before sealing the handoff.

The existing `VerifiedLoopPolicyWinnerV1` raw cursor remains diagnostic
provenance only. It must not become a route/family dispatch input.

## Required tests

1. Positive Direct Accum fixture: source-view navigation yields the exact loop
   condition, update target/value, update operands, and step operands; resolver
   lookup yields the expected `BindingRefV1`s; the sealed facts product is
   AST-free.
2. Wrong child role or foreign `Located*` owner rejects before product issue.
3. Missing condition/operand topology rejects; ScopeBox lineage is accepted
   only through the source view and otherwise fails closed.
4. Shadowed binding and non-binding assignment target reject; no name-based
   fallback is permitted.
5. Source identity mismatch and `LoopExecutionFrameKeyV1` mismatch reject at
   selected-demand handoff.
6. All three capabilities are consumed once; no `Clone` escape or second
   issuer exists.

## Guard and scope

The neutral facts module must have zero imports of AST, route registry,
CanonicalLoopFacts, Recipe, CorePlan, PlanLowerer, MirBuilder, physical IDs,
PHI, Binding SSA, Retry, scheduler, physicalizer, or Generic debt machinery.

Production `route_loop`, legacy scheduler, Recipe producer, and
`LoopPhiMaterializerV1` callers remain zero. The existing PHI/SSA SSOT chain
(`CanonicalCfgSessionV1` + function-owned `BindingSsaBuilderV1` + `PhiTxn`) is
not modified.

## Acceptance commands

```text
RUSTFLAGS='-Awarnings' cargo test -q 'mir::loop_structural_facts::' --lib
RUSTFLAGS='-Awarnings' cargo test -q 'mir::loop_route_policy::' --lib
bash tools/checks/lib/joinir_logical_demand_contract.sh
bash tools/checks/current_state_pointer_guard.sh
cargo build --release --bin hakorune
```

All touched Rust files stay below 800 lines. A failing gate stops the slice;
do not add a production caller or broaden the DTO to make the fixture pass.

## Explicit non-goals

- No Recipe construction or verifier/JoinSig connection.
- No PHI/SSA materializer or CFG change.
- No route winner recomputation, Generic classification, or retry removal.
- No `LoopRouteContext`-only production adapter; it lacks the located/resolved
  execution frame required by this contract.
- No symbolic MIR, detached Builder, undo journal, or Loop-local transaction.
