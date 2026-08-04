---
Status: active; test-only stage-matrix completeness and disposition audit
Date: 2026-08-04
Parent: ../design/joinir-generic-post-effect-debt-classification-ssot.md
Decision: accepted — JOINIR-LOOP-GENERIC-POST-EFFECT-DEBT-CLASSIFICATION0-D0-S1
---

# Generic V0/V1 post-effect debt — D0-S1 execution task

## Boundary

This is the smallest executable slice selected after the worker-backed M4
design audit. It adds only test-side observation and contract assertions for
the existing Generic V0/V1 route. It does not add a Recipe producer, JoinSig,
PHI writer, physicalizer, scheduler, retry, fallback, route policy, or
candidate publication path.

The current production authority remains:

```text
route_loop
-> live preflight
-> ordered legacy route witness
-> Generic V0/V1 handler
-> composer/CorePlan/PlanVerifier/PlanLowerer
-> legacy JoinIR/JoinModule
```

The test observer must use the real facts builder, selector, witness,
handlers, composer, verifier, and lowerer. It must not use
`all_route_preflight` as a winner oracle and must not manufacture failures by
fault injection.

## Objective

Lock one machine-readable `GenericStageDispositionMatrixV1` observation table
for both Generic composers. The table must distinguish a pre-effect decline or
blocked precondition from a candidate that has already mutated the Builder.
It must retain both the first effect owner and the later stage delta.

The disposition vocabulary is the parent SSOT vocabulary:

```text
PreEffectDeclined
PreEffectBlocked
TerminalFreezeTarget
ImpossibleEdge
UnresolvedStop
```

Do not rename an effectful composer/verifier/lowerer failure to
`PreEffectDeclined`. Keep `UnresolvedStop` when winner equivalence, candidate
abort, or a debt-to-later-winner trace is not proven.

## Matrix dimensions

Every row records source anchors, mode, facts, raw schedule, attempted prefix,
stage, first effect owner, before/after candidate snapshot, typed receipt (if
any), terminal/disposition, and evidence level.

Required fixture families:

```text
V0-only
V1-only
Both
Neither
```

Required modes:

```text
release
strict/dev
strict/dev + planner_required
```

Required stages/arms:

1. facts absent or non-matching;
2. composer precondition failure before allocation;
3. composer success and first skeleton/body/pipeline mutation;
4. composer `Err` after mutation, where a natural source witness exists;
5. strict shadow lower `Some`, `None`, and `Err`;
6. release `PlanVerifier` `Ok` and `Err`;
7. release `PlanLowerer` `Some`, `Ok(None)`, and `Err`;
8. nested Generic composer calls and the direct nested-depth route;
9. legacy receipt and attempted-prefix disposition.

Unobserved natural arms must remain explicit `NotYetObserved`/
`UnresolvedStop` rows with source anchors. Do not add synthetic error injection
merely to make the matrix look complete.

## Snapshot and owner contract

The candidate snapshot must include at least:

```text
block_count
next_value_id
typed_value_count
variable_map/binding context when relevant
```

`with_saved_variable_map_typed` restores only variable-map/binding state; it is
not a rollback for block/value counters or type context. Therefore:

```text
before_compose -> after_compose
```

determines whether `GenericComposer` is the first effect owner, while

```text
before_lower -> after_lower
```

records later PlanLowerer effects separately. A PlanVerifier error is pure but
occurs after composer effects when the composer already allocated a candidate.

Nested rows must distinguish an outer Generic composer from a nested composer;
the depth and first mutation owner are separate evidence fields.

## Existing evidence and blockers

The current observer already proves the raw overlap schedule
`[GenericLoopV0, GenericLoopV1]` for the Both fixture and the planner-required
suppression of V0. It also shows the current release/strict witness attempts
V0 and succeeds without a V1 debt-to-later-winner trace. This is not a
precedence proof.

The worker audit found these unresolved test-side contract issues; preserve
them as blockers until source and fixture ownership are audited:

* the nested-carrier policy witness currently reaches `UnresolvedStop` when
  the production trace has no recipe contract; do not make policy ignore that
  contract requirement;
* top-level `CompoundAssignment` currently differs from the nested
  `Unavailable("CompoundAssignment")` extractor contract; decide the source
  boundary with a fixture/test contract rather than changing an expectation
  blindly;
* the focused `generic_` suite currently has three failures, including the two
  observations above and an unrelated module-metadata assertion. They are
  evidence to classify, not permission to weaken guards.

No M4 closeout, V0/V1 winner, or M5/M6/M10 production handoff may be claimed
while these unresolved rows or the semantic digest mismatch remain.

## Implementation steps

1. Add or extend a `#[cfg(test)]` matrix observer by reusing
   `observe_fixture`/`observe_selected_fixture` and existing frame/witness
   helpers.
2. Add rows for V0-only, V1-only, Both, and Neither across the three modes.
3. Record stage, owner, snapshots, receipt, attempted prefix, and disposition
   for every observed row; leave natural unobserved arms marked unresolved.
4. Add nested-depth coverage without changing production nested composition.
5. Audit and resolve the two source/fixture contract mismatches above; if a
   row cannot be resolved without production policy changes, leave it as an
   explicit blocker in this card.
6. Keep all touched Rust and test files below 800 lines.

## Acceptance gates

The task is complete only when the matrix is deterministic on a fresh
candidate and the following gates are green (or an explicitly documented
unresolved row is the intended result):

```text
RUSTFLAGS='-Awarnings' cargo test -q --lib generic_ -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib generic_accepted_plan_reachability_tests -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib generic_stage_observer_tests -- --test-threads=1
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/lib/joinir_logical_demand_contract.sh
git diff --check
```

The focused generic suite must not be made green by deleting a failing row,
weakening a contract assertion, adding a by-name route, or enabling a hidden
environment switch.

## Post-implementation reference closeout

After the test implementation and evidence land, update the applicable Generic
Loop design/reference surfaces before marking D0-S1 closed:

* `docs/development/current/main/design/joinir-generic-post-effect-debt-classification-ssot.md`
  — stage matrix rows, owner/disposition definitions, unresolved blockers,
  and the next selected row;
* the Generic Loop route/recipe contract README or registry that describes
  the observed stage and receipt vocabulary;
* any `docs/reference/ir/` or `docs/reference/mir/` page that claims Generic
  V0/V1 precedence, verifier/lowerer receipts, or candidate rollback;
* `CURRENT_STATE.toml` and the active execution card, including the exact
  acceptance commands and whether the row remains `UnresolvedStop`.

If no normative language or IR behavior changes, state that explicitly and do
not invent a grammar change. Reference synchronization is a required closeout
step, not optional cleanup.

## Stop conditions

Stop and return to the parent design stop if any requested fix requires:

* production changes to Generic handlers, route selection, `all_route_preflight`,
  Recipe/JoinSig/PHI/physicalizer owners, scheduler, retry, or fallback;
* treating raw ENTRIES order as V0/V1 semantic precedence;
* declaring winner equivalence from a single successful V0 path;
* deleting legacy receipts before a proven M10 handoff;
* changing a test expectation without an audited source/fixture contract;
* any touched source/test file reaching 800 lines.

