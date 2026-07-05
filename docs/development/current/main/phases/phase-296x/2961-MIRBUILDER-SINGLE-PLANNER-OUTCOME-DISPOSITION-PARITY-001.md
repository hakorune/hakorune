---
Status: Landed
Date: 2026-07-05
Scope: single_planner outcome disposition DTO parity slice.
---

# MIRBUILDER-SINGLE-PLANNER-OUTCOME-DISPOSITION-PARITY-001

## Decision

Land parity for the read-only `try_build_outcome` disposition boundary that
decides whether planner-required None freezes or the existing outcome returns.

```text
selected_owner=single_planner_outcome_disposition.authority_facade
input_contract=BackendSafeSinglePlannerOutcomeDispositionTokenSnapshotV1
rust_oracle_symbol=try_build_outcome planner_required None freeze boundary
rust_source=src/mir/builder/control_flow/plan/single_planner/rules.rs
hako_source=lang/src/compiler/lib/single_planner_outcome_disposition.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_single_planner_outcome_disposition_parity_gate.sh
oracle_rows=7
```

## Scope

The facade owns only the disposition summary for these token facts:

```text
planner_required_token=PlannerRequired|PlannerOptional
planner_present_token=PlannerPresent|PlannerAbsent
outcome_facts_token=FactsPresent|FactsNone
```

It mirrors the Rust freeze condition:

```text
planner_required && !planner_present && outcome.facts.is_none()
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
MIRBUILDER-SINGLE-PLANNER-OUTCOME-DISPOSITION-HAKOADOPTED-DECISION-001
```
