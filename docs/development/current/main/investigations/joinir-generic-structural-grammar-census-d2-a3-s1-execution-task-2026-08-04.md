---
Status: active; test-only D2-A3 natural-arm and nested-depth census
Date: 2026-08-04
Parent: ../design/joinir-generic-post-effect-debt-classification-ssot.md
Previous: joinir-generic-post-effect-debt-classification-d0-s1-execution-task-2026-08-04.md
Decision: accepted — JOINIR-GENERIC-STRUCTURAL-GRAMMAR-CENSUS0-D2-A3-S1
---

# Generic V0/V1 post-effect debt — D2-A3-S1 execution task

## Boundary

D0-S1 closed the deterministic stage/disposition ledger. D2-A3-S1 is the next
smallest child of the parent structural-grammar boundary census: audit whether
existing natural Generic sources can observe the remaining failure and
nested-depth arms. This is a test-only census. It must not add a production
route, policy winner, Recipe producer, JoinSig, PHI
writer, physicalizer, scheduler, retry, fallback, or candidate publication
path.

The production authority remains:

```text
route_loop
-> live preflight
-> ordered legacy route witness
-> Generic V0/V1 handler
-> composer/CorePlan/PlanVerifier/PlanLowerer
-> legacy JoinIR/JoinModule
```

The `loop_route_policy` subtree is `#[cfg(test)]` evidence only. It must not be
used as a V0/V1 winner or precedence oracle.

## Objective

Using only existing accepted Generic source fixtures and fresh Builder
candidates:

1. Re-observe one accepted Generic body in release and strict modes and record
   whether strict shadow `Err`, release verifier `Err`, or release lower `Err`
   occurs naturally.
2. Preserve strict shadow `None` and release lower `Ok(None)` as
   `NotYetObserved`/`ImpossibleEdge` unless a source-derived valid witness
   contradicts the completion invariant. Do not synthesize an invalid CorePlan
   and do not inject a failure.
3. Add a test-only nested owner observer around the existing
   `lower_nested_loop_depth1_any` fastpath and the subsequent
   `nested_loop_recipe_adoption` fallback. Record which owner is reached for
   the existing nested `Both` source shape, the first Builder mutation, and
   whether the route is a natural success or an unresolved stop.
4. Keep V0-only status honest. If `v0-additive` still produces a mixed or
   otherwise unproven Generic class, retain the explicit `UnresolvedStop` row;
   do not invent a source fixture solely to force V0 precedence.

## Authority and observation contract

Use the real source/facts/composer path and the existing fresh-candidate
snapshot contract from D0-S1:

```text
block_count
next_value_id
typed_value_count
variable_map/binding context when relevant
```

The nested observer must distinguish:

```text
NestedFastpath:
  lower_nested_loop_depth1_any returns Some/Ok

NestedGenericFallback:
  fastpath returns None/Err, then nested_loop_recipe_adoption is attempted
```

The observer is allowed to report `NotObserved` when a private helper cannot
be reached through a natural source shape. It must not call a production
helper with by-name routing or alter helper visibility merely for the test.

The first mutation owner is determined by before/after Builder snapshots, not
by helper names. A variable-map restore is not candidate rollback; block/value
and type counters remain part of the snapshot.

## Required matrix additions

Extend the D0-S1 table with rows for:

| mode | route | arm | expected current disposition |
| --- | --- | --- | --- |
| strict/dev | V0/V1 | shadow `Some` | observed or `NotYetObserved` |
| strict/dev | V0/V1 | shadow `None` | `ImpossibleEdge`/`NotYetObserved` |
| strict/dev | V0/V1 | shadow `Err` | `UnresolvedStop` if not naturally observed |
| release | V0/V1 | verifier `Ok` | observed on accepted plans |
| release | V0/V1 | verifier `Err` | `UnresolvedStop` if not naturally observed |
| release | V0/V1 | lower `Some` | observed on accepted plans |
| release | V0/V1 | lower `Ok(None)` | `ImpossibleEdge`/`NotYetObserved` |
| release | V0/V1 | lower `Err` | `UnresolvedStop` if not naturally observed |
| all | nested | depth1 fastpath | observed or `NotYetObserved` |
| all | nested | Generic fallback | observed or `NotYetObserved` |

Every row retains source anchor, contract-present bit, raw schedule, attempted
prefix, first-effect owner, snapshots, receipt, terminal, disposition, and
evidence level. The raw `ENTRIES` order remains an execution trace only.

## Implementation steps

1. Add one test-only accepted-body re-observation helper; reuse the D0-S1
   snapshot and mode configuration rather than adding another observer schema.
2. Add one test-only nested-depth observer, preferably in the existing generic
   loop-body test module or a small sibling module below 800 lines.
3. Feed the existing `Both` nested source shape first. If fastpath/fallback
   cannot be distinguished without production edits, record both as
   `NotYetObserved` with source anchors and stop.
4. Update the D0-S1 matrix test and this card with observed/unobserved rows.
5. Keep all touched Rust/test files below 800 lines and preserve the focused
   generic gates.

## Acceptance gates

```text
RUSTFLAGS='-Awarnings' cargo test -q --lib generic_stage_matrix_tests -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib generic_stage_observer_tests -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib generic_accepted_plan_reachability_tests -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib generic_loop_body -- --test-threads=1
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/lib/joinir_logical_demand_contract.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
git diff --check
```

The broad `generic_` gate is evidence-only for this row. The known unrelated
`collection_builders.rs:696` baseline failure must remain recorded and must
not be fixed by weakening the Generic matrix.

## Post-implementation reference closeout

Before marking D2-A3-S1 complete, update all applicable reference surfaces:

* `src/mir/builder/control_flow/plan/generic_loop/README.md` — stage owner,
  nested fastpath/fallback, receipt, and candidate rollback boundary;
* `docs/development/current/main/design/joinir-generic-post-effect-debt-classification-ssot.md`
  — D2-A3-S1 rows and unresolved natural-arm result;
* `docs/reference/mir/generic-loop-stage-matrix.md` — observed depth/failure
  rows and explicit non-claims;
* this task card and `CURRENT_STATE.toml` — exact commands, blocker, and next
  row. State explicitly that no normative grammar/IR behavior changed.

Reference synchronization is required closeout, not optional cleanup.

## Stop conditions

Stop and return to the parent design stop if any requested fix requires:

* changing production Generic handlers, selector/predicates, all-route
  preflight, scheduler, retry/fallback, Recipe/JoinSig/PHI/physicalizer, or
  candidate publication;
* invalid-plan construction, fault injection, a new environment switch, or
  by-name helper dispatch;
* treating a successful V0 path as V0/V1 winner proof;
* changing an existing expectation without auditing the source contract;
* any touched source/test file reaching 800 lines.
