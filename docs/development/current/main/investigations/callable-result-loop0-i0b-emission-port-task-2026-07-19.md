---
Status: LOOP0-I0b closed; LOOP0-L0 is next
Date: 2026-07-19
Parent: callable-result-i64-site0-r0-expression-spine-loop0-task-2026-07-18.md
Scope: shared CorePlan effect-emission port and disconnected claim handoff
---

# Callable-result LOOP0-I0b emission-port closeout

## Decision

`LOOP0-I0b` is closed as a behavior-preserving BoxShape refactor. One
stack-scoped `CorePlanEffectEmissionPortV1` now owns every ordinary CorePlan
effect emission during one lowering invocation.

```text
raw PlanLowerer::lower
  -> Raw emission port
  -> existing raw effect emission

future sealed located Loop
  -> consuming claimed execution bundle
  -> Claimed emission port
  -> exact source-site claim consumption
```

The port is threaded through CorePlan, sequence, branch, loop, block, and body
lowering. It is borrowed through recursion only: `MirBuilder` stores no plan,
view, source-site, ledger, schedule, or claim-batch authority.

## Disconnected located handoff

`VerifiedLocatedCoreLoopPlanV1::into_claimed_execution` consumes the final
already-remapped plan seal and atomically acquires its source-order batch from
the existing caller ledger. The resulting non-Clone execution bundle owns both
the CorePlan and the single-use batch; neither can be paired or emitted twice.

Claim-batch failure invokes no `PlanLowerer` work. I0b intentionally does not
define selected-session poisoning; that remains the `LOOP0-L0` ingress
responsibility. Production located roots and production located execution
callers remain zero.

## Selected-call authority

The claimed port consumes each `LocatedMethodCall` by exact
`SourceExprSiteV1`. `Unselected` claims use the existing raw effect emitter.
`SelectedExactI64` is emitted only from the claim disposition's canonical
target, arity, and required-i64 argument contract. Raw `GlobalCall.func` and
`MethodCall.method` are not selected-call authority.

The focused terminal test supplies a deliberately false raw global spelling
and proves that emitted MIR uses the canonical claimed projection instead.

## Evidence

```text
cargo fmt --check
cargo test -q --lib generic_loop_p0c
cargo test -q --lib generic_loop_whole_parity_tests
cargo test -q --lib callable_result_representation::tests::loop_claim_batch
cargo test -q --lib plan::located_loop_tests
cargo test -q --lib plan::lowerer::tests
cargo check --all-targets
python3 tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0.py .
bash tools/checks/no_unapproved_plan_lowerer_entry.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The public expression-spine guard imports the I0b structural subchecker. It
fixes one port, one raw facade, one consuming handoff, one selected terminal,
no legacy direct effect entry, no selected raw-spelling read, and source/check
files below 800 lines.

## Next: `LOOP0-L0`

Connect the already-sealed located GenericLoopV1 carrier to pure selection,
located composition, final plan seal, the source-order claim batch, and this
shared port. Accept only the selected GenericLoopV1 profile; normalized shadow
and every other located route reject before effects. Do not connect a
production source root until `EXPR0-C0`.
