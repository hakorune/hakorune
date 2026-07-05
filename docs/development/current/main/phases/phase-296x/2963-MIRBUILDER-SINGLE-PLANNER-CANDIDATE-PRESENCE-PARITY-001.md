---
Status: Landed
Date: 2026-07-05
Scope: single_planner candidate presence DTO parity slice.
---

# MIRBUILDER-SINGLE-PLANNER-CANDIDATE-PRESENCE-PARITY-001

## Decision

Land parity for `planner_candidate_present` as the next Plan-track DTO pilot.

```text
selected_owner=single_planner_candidate_presence.authority_facade
input_contract=BackendSafeSinglePlannerCandidatePresenceTokenSnapshotV1
rust_oracle_symbol=planner_candidate_present
rust_source=src/mir/builder/control_flow/plan/single_planner/rules.rs
hako_source=lang/src/compiler/lib/single_planner_candidate_presence.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_single_planner_candidate_presence_parity_gate.sh
oracle_rows=6
```

The facade owns only the `PlanBuildOutcome` fact-slot presence reduction for
the active `LoopCondContinueWithReturn` candidate.

## Non-Claims

```text
source_selfhost_claim=0
build_plan_with_facts_ctx_migrated=0
full_try_build_outcome_migrated=0
fact_extraction_migrated=0
recipe_matching_migrated=0
route_execution_migrated=0
backend_lowering_migrated=0
mir_mutation_migrated=0
id_allocation_migrated=0
hako_generation=0
runtime_fallback=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-SINGLE-PLANNER-CANDIDATE-PRESENCE-HAKOADOPTED-DECISION-001
```
