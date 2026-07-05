---
Status: Landed
Date: 2026-07-05
Scope: single_planner recipe-match gate DTO parity slice.
---

# MIRBUILDER-SINGLE-PLANNER-RECIPE-MATCH-GATE-PARITY-001

## Decision

Land parity for the read-only gate that decides whether `try_build_outcome`
should call `RecipeMatcher`.

```text
selected_owner=single_planner_recipe_match_gate.authority_facade
input_contract=BackendSafeSinglePlannerRecipeMatchGateTokenSnapshotV1
rust_oracle_symbol=try_build_outcome recipe matcher gate
rust_source=src/mir/builder/control_flow/plan/single_planner/rules.rs
hako_source=lang/src/compiler/lib/single_planner_recipe_match_gate.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_single_planner_recipe_match_gate_parity_gate.sh
oracle_rows=9
```

This facade owns only the decision to `match_required`,
`match_strict_or_dev`, `match_best_effort`, or `skip`. `RecipeMatcher`
execution remains Rust.

## Non-Claims

```text
source_selfhost_claim=0
recipe_matcher_execution_migrated=0
build_plan_with_facts_ctx_migrated=0
full_try_build_outcome_migrated=0
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
MIRBUILDER-SINGLE-PLANNER-RECIPE-MATCH-GATE-HAKOADOPTED-DECISION-001
```
