---
Status: closed as deterministic test-only evidence; semantic parity remains UnresolvedStop; M6-B is next
Date: 2026-08-04
Parent: ../design/joinir-generic-post-effect-debt-classification-ssot.md
Previous: joinir-generic-structural-grammar-census-d2-a3-s1-execution-task-2026-08-04.md
Decision: accepted — JOINIR-GENERIC-OVERLAP-SEMANTIC-PARITY0-D2-B2
---

# Generic V0/V1 overlap — D2-B2 semantic parity task

## Boundary

D2-A3-S1 closed the bounded natural-arm/depth census. D2-B2 is a design and
test-only evidence stop for the existing `Both` overlap. It must not add a
production policy winner, selector branch, Recipe producer, JoinSig, PHI
writer, physicalizer, retry removal, fallback deletion, or JoinIR retirement.
The ordered legacy scheduler remains the execution authority until parity is
proven.

The pure `loop_route_policy` subtree remains an observation product only. A
successful V0 or V1 direct stage is not a winner proof.

## Known evidence to reconcile

The current corpus already records:

```text
release / strict:
  raw schedule = [GenericLoopV0, GenericLoopV1]
  V0 and V1 direct plans lower successfully on fresh candidates
  legacy witness = V0 success, no debt receipt, no V1 attempt

strict + planner_required:
  V0 is suppressed before effect
  V1 remains the selected route

semantic digest:
  V0 and V1 differ for the nested-carrier `Both` row (carrier j)
```

These facts are not equivalent to a V0/V1 winner decision. The nested digest
difference is an explicit `UnresolvedStop`, not evidence for a
`V1ForNestedCarriers` policy.

## Required parity matrix

For every claimed `Both` row and each release, strict, and
strict+planner-required mode, record one row containing:

| column | required evidence |
| --- | --- |
| source/mode | exact D2-A3 grammar row and mode |
| frame | shared production `LivePreflightFrameV1`: strict/dev, planner-required, body-local, contract, recipe-first |
| selection | actual facts, raw schedule, and V0-only/V1-only/Both/Neither disposition |
| direct V0 | fresh composer → verifier/shadow → lower result, snapshots, first effect, semantic digest |
| direct V1 | fresh composer → verifier/shadow → lower result, snapshots, first effect, semantic digest |
| witness | actual attempted prefix, typed debt receipt if any, outer error, and terminal |
| pure probe | current test-only disposition, with no route-name inference |
| comparison | target plan/digest, prefix, terminal, and candidate-delta equality or mismatch |
| classification | closed debt vocabulary or explicit `UnresolvedStop` |

Run V0 and V1 on separate fresh candidates before reading the witness. The
frame must come from the shared production preparation helper; the observer
may borrow only the frame's schedule/environment. Do not synthesize malformed
plans or inject verifier/lower failure.

## Decision boundary

D2-B2 may close only with one of these production-derived proofs:

1. V0 and V1 are disjoint for every claimed `Both` row before Builder effect;
   or
2. the actual V0 and V1 targets are semantically equivalent, with equal
   normalized plan meaning, candidate ownership, witness prefix, and terminal
   behavior in every required mode.

Until then:

```text
V0/V1 overlap = UnresolvedStop
legacy scheduler = retained
retry/fallback deletion = forbidden
V0/V1 semantic policy change = forbidden
```

The planner-required suppression row is a separate pre-effect gate. It must
not be used to claim release/strict disjointness.

## Current matrix implementation evidence

The test-only matrix is implemented in
`generic_stage_observer_tests::semantic_parity_matrix`. Each row retains the
shared frame, direct V0/V1 `GenericDirectStageEvidenceV1` entries (including
snapshots, first-effect owner, stage, and semantic digest), the complete
witness trace, the pure probe result, and the final `UnresolvedStop`
classification. Fresh repeats are compared before any interpretation.

Current observed rows:

```text
release / strict:
  [V0, V1] direct routes; both LowerSome; digest differs; witness V0 success
  with no debt and no V1 attempt; comparison = UnresolvedStop

strict + planner_required:
  [V1] direct route; LowerSome; V0 suppressed before effect; comparison still
  UnresolvedStop because this is not release/strict overlap proof
```

The matrix is evidence only. `ParityDispositionV1::UnresolvedStop` is a locked
classification of these observations, not a policy evaluator and not a route
winner. The digest mismatch and the legacy V0 terminal prevent a
production-derived disjointness or semantic-equivalence proof. D2-B2 therefore
closes as a deterministic evidence/design stop while the parent D2/M4 overlap
decision remains unresolved and the ordered legacy scheduler remains active.

## D2-B2 closeout

Implementation and evidence are complete for the bounded `Both` matrix. The
release/strict rows retain raw `[V0, V1]`, fresh direct `LowerSome` results,
`GenericComposer` as the first effect owner, nested-carrier digest mismatch,
and a legacy V0 terminal with no debt receipt or V1 attempt. The
planner-required row records V0 suppression before effect and an independent
V1 `LowerSome` result; it is a separate pre-effect gate, not overlap proof.
Fresh repeats are identical, and all required focused gates are green:
`generic_stage_` 11 passed (matrix 2, observer 9, accepted-plan 6).

No production selector, Recipe, JoinSig, PHI, physicalizer, candidate
publication, retry/fallback, scheduler, grammar, or IR behavior changed. The
parent design SSOT, this task card, the MIR stage-matrix reference, the Generic
loop README, and the current-state pointers are synchronized as part of this
closeout. Reference-document synchronization is a completion requirement for
the implementation, not optional cleanup. The next bounded design boundary is
the caller-zero M6-B PHI materializer card
`JOINIR-LOOP-CFG-JOINSIG-PHI0-D0-S4`; Generic D2 remains `UnresolvedStop`.

## Acceptance gates

```text
RUSTFLAGS='-Awarnings' cargo test -q --lib generic_stage_matrix_tests -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib generic_stage_observer_tests -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib generic_accepted_plan_reachability_tests -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib generic_stage_ -- --test-threads=1
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/lib/joinir_logical_demand_contract.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
git diff --check
```

The broad `generic_` baseline failure at
`collection_builders.rs:696::refresh_module_semantic_metadata_accepts_array_string_push_in_generic_pure_string_body`
is unrelated evidence and must remain recorded rather than fixed in this
row. All touched source/test files remain below 800 lines.

## Post-implementation reference closeout

Before closing D2-B2, synchronize all applicable surfaces:

* `docs/development/current/main/design/joinir-generic-post-effect-debt-classification-ssot.md`
  — parity decision and unresolved comparison;
* this D2-B2 card — exact matrix rows and gates;
* `docs/reference/mir/generic-loop-stage-matrix.md` — direct/witness parity,
  digest differences, and non-claims;
* `src/mir/builder/control_flow/plan/generic_loop/README.md` — owner boundary;
* `CURRENT_STATE.toml` and `10-Now.md` — current blocker and next row.

State explicitly that no normative grammar or IR behavior changed. Reference
synchronization is part of completion, not optional cleanup.

## Stop conditions

Stop at this design boundary if the next change would require:

* production selector/handler/preflight changes;
* Recipe/JoinSig/PHI/physicalizer or candidate-publication changes;
* retry/fallback/scheduler deletion;
* by-name dispatch, invalid-plan construction, or fault injection;
* treating a direct success, digest coincidence, or planner-required row as
  universal winner proof;
* any touched source/test file reaching 800 lines.
