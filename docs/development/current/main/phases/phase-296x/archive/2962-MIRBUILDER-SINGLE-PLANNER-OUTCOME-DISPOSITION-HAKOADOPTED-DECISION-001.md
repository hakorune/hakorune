---
Status: Landed
Date: 2026-07-05
Scope: scoped HakoAdopted decision for single_planner outcome disposition DTO.
---

# MIRBUILDER-SINGLE-PLANNER-OUTCOME-DISPOSITION-HAKOADOPTED-DECISION-001

## Decision

Adopt the read-only single_planner outcome disposition facade.

```text
decision=HakoAdoptedScoped
adopted_owner=single_planner_outcome_disposition.authority_facade
input_contract=BackendSafeSinglePlannerOutcomeDispositionTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/single_planner_outcome_disposition.hako
```

This adopts only the planner-required None freeze vs return-outcome DTO. It does
not adopt `build_plan_with_facts_ctx`, full `try_build_outcome`, recipe
matching, logging, route execution, backend lowering, MIR mutation, or ID
allocation.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-single-planner-outcome-disposition-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/single_planner_outcome_disposition.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_single_planner_outcome_disposition_parity_gate.sh
adoption_guard=tools/checks/rust_lifecycle_mirbuilder_single_planner_outcome_disposition_hako_adoption_decision_guard.sh
oracle_rows=7
parity_status=green
```

## Non-Claims

```text
source_selfhost_claim=0
build_plan_with_facts_ctx_migrated=0
full_try_build_outcome_migrated=0
recipe_matching_migrated=0
logging_migrated=0
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
MIRBUILDER-PLAN-TRACK-NEXT-PILOT-SELECTION-002
```
