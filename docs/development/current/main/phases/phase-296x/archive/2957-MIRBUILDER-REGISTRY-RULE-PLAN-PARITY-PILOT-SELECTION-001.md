---
Status: Landed
Date: 2026-07-05
Scope: Plan-track selection for the first REGISTRY/single-planner DTO parity pilot.
---

# MIRBUILDER-REGISTRY-RULE-PLAN-PARITY-PILOT-SELECTION-001

## Decision

Select the active `LoopCondContinueWithReturn` single-planner rule for the
first facts-to-plan DTO parity pilot.

```text
selected_owner=loop_cond_continue_with_return_plan_rule.authority_facade
rust_oracle_symbol=single_planner::planner_matches_rule_kind / PLAN_RULE_ORDER
rust_source=src/mir/builder/control_flow/plan/single_planner/rules.rs
rule_order_source=src/mir/builder/control_flow/plan/single_planner/rule_order.rs
next_card=MIRBUILDER-LOOP-COND-CONTINUE-WITH-RETURN-PLAN-RULE-PARITY-001
```

## Why This Candidate

```text
active_plan_rule_count=1
active_plan_rule=LoopCondContinueWithReturn
facts_to_plan_dto=1
symbolic_ids_only=1
no_mir_mutation=1
no_backend_lowering=1
no_id_allocation=1
no_new_hako_backend_capability=1
```

The selected facade may cover only rule-order membership, planner-present
acceptance, recipe-only classification, semantic label, tag label, and route
label. It must not migrate `build_plan_with_facts_ctx`, recipe matching,
router execution, route selection beyond the selected DTO, backend lowering,
MIR mutation, or ID allocation.

## Held Candidates

```text
build_plan_with_facts_ctx:
  held; constructs full PlanBuildOutcome and fact aggregation

try_build_outcome:
  held; owns PlannerGate, freeze behavior, recipe matching, and logging

additional PlanRuleId variants:
  held; not currently active in PLAN_RULE_ORDER
```

## Non-Claims

```text
source_selfhost_claim=0
build_plan_with_facts_ctx_migrated=0
try_build_outcome_migrated=0
recipe_matching_migrated=0
route_execution_migrated=0
backend_lowering_migrated=0
mir_mutation_migrated=0
id_allocation_migrated=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-LOOP-COND-CONTINUE-WITH-RETURN-PLAN-RULE-PARITY-001
```
