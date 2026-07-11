---
Status: Landed
Date: 2026-07-05
Scope: Fact-owner selection-024 inventory and Plan-track promotion.
---

# MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-024

## Decision

Promote to Plan track instead of selecting another Fact facade.

```text
selected_next_card=MIRBUILDER-REGISTRY-RULE-PLAN-PARITY-PILOT-SELECTION-001
reason=NoStrictlySmallerNonAdoptedFactCandidate
plan_runner_up_candidates=build_plan_with_facts_ctx,try_build_outcome
```

## Inventory Result

```text
loop_step_shape=already_hako_adopted
loop_condition_shape=already_hako_adopted
loop_simple_while_facts=already_hako_adopted
loop_continue_only_facts=already_hako_adopted
loop_break_body_local_facts=already_hako_adopted
loop_break_step_before_break_facts=already_hako_adopted
loop_break_facts=held_full_subset_dispatch
loop_break_body_local_subset=held_synthetic_condition_and_freeze_policy
loop_scan_with_init=held_scan_body_analysis
loop_split_scan=held_scan_body_analysis
nested_loop_minimal_facts=held_composes_multiple_fact_owners
loop_cond_break_continue_facts=held_multi_variant_route_facts
```

No remaining Fact candidate is both non-adopted and strictly smaller than the
Plan runner-ups. Width-only Fact progress stops here.

## Plan Candidate Basis

```text
single_planner_entry=src/mir/builder/control_flow/plan/single_planner/mod.rs
rule_order_ssot=src/mir/builder/control_flow/plan/single_planner/rule_order.rs
active_plan_rule_count=1
active_plan_rule=LoopCondContinueWithReturn
registry_entry=src/mir/builder/control_flow/plan/REGISTRY.md
```

The next selection must choose one REGISTRY/single-planner rule as a
facts-to-plan DTO parity pilot. The current active rule order contains only
`LoopCondContinueWithReturn`, so that rule is the default candidate unless the
Plan selection card finds a smaller non-mutating plan DTO boundary.

## Non-Claims

```text
source_selfhost_claim=0
new_fact_owner=0
plan_parity_started=0
registry_rule_adopted=0
route_selection_migrated=0
backend_lowering_migrated=0
mir_mutation_migrated=0
id_allocation_migrated=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-REGISTRY-RULE-PLAN-PARITY-PILOT-SELECTION-001
```
