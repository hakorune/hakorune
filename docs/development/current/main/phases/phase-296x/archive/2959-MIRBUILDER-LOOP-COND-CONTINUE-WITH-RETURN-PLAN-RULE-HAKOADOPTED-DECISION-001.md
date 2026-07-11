---
Status: Landed
Date: 2026-07-05
Scope: scoped HakoAdopted decision for LoopCondContinueWithReturn plan-rule DTO.
---

# MIRBUILDER-LOOP-COND-CONTINUE-WITH-RETURN-PLAN-RULE-HAKOADOPTED-DECISION-001

## Decision

Adopt the Plan-rule DTO facade for `LoopCondContinueWithReturn`.

```text
decision=HakoAdoptedScoped
adopted_owner=loop_cond_continue_with_return_plan_rule.authority_facade
input_contract=BackendSafeLoopCondContinueWithReturnPlanRuleTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/loop_cond_continue_with_return_plan_rule.hako
```

This does not adopt `build_plan_with_facts_ctx`, full `try_build_outcome`,
recipe matching, router execution, route execution, backend lowering, MIR
mutation, or ID allocation.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-cond-continue-with-return-plan-rule-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/loop_cond_continue_with_return_plan_rule.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_loop_cond_continue_with_return_plan_rule_parity_gate.sh
adoption_guard=tools/checks/rust_lifecycle_mirbuilder_loop_cond_continue_with_return_plan_rule_hako_adoption_decision_guard.sh
oracle_rows=5
parity_status=green
```

## Non-Claims

```text
source_selfhost_claim=0
build_plan_with_facts_ctx_migrated=0
try_build_outcome_migrated=0
recipe_matching_migrated=0
router_execution_migrated=0
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
MIRBUILDER-PLAN-TRACK-NEXT-PILOT-SELECTION-001
```
