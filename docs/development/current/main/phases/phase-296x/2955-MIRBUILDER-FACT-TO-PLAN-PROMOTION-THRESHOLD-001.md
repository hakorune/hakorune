---
Status: Landed
Date: 2026-07-05
Scope: Fact track stop threshold before Plan track promotion.
---

# MIRBUILDER-FACT-TO-PLAN-PROMOTION-THRESHOLD-001

## Decision

Set a hard stop threshold for the current Fact-facade cadence.

```text
fact_facade_extra_budget_after_selection_023_correction=2
selection_024_must_compare_plan_runner_up=1
plan_runner_up_candidates=build_plan_with_facts_ctx,try_build_outcome
promotion_target=MIRBUILDER-REGISTRY-RULE-PLAN-PARITY-PILOT-SELECTION-001
```

Selection-024 may choose one more Fact owner only if it is strictly smaller
than the runner-up Plan candidates and is not already parity-landed or
HakoAdopted. If that condition is not proven by the selection card, the next
task must promote to a REGISTRY-rule plan DTO pilot.

## Stop Conditions

Promote to Plan track immediately when any condition is true:

```text
non_adopted_fact_candidate_absent=1
candidate_already_has_parity_or_adoption=1
candidate_requires_AST_payload_migration=1
candidate_requires_route_selection_or_lowering=1
candidate_not_strictly_smaller_than_plan_runner_up=1
additional_fact_facade_adoptions_since_this_card >= 2
```

This prevents width-only progress. Fact facade work remains useful, but it is
not allowed to indefinitely delay the first facts-to-plan DTO slice.

## Non-Claims

```text
source_selfhost_claim=0
plan_track_started=0
registry_rule_selected=0
backend_lowering_migrated=0
mir_mutation_migrated=0
id_allocation_migrated=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-024
```
