---
Status: Landed
Date: 2026-07-05
Scope: LoopCondContinueWithReturn plan-rule DTO parity slice.
---

# MIRBUILDER-LOOP-COND-CONTINUE-WITH-RETURN-PLAN-RULE-PARITY-001

## Decision

Land parity for the active `LoopCondContinueWithReturn` single-planner rule as
a facts-to-plan DTO owner.

```text
selected_owner=loop_cond_continue_with_return_plan_rule.authority_facade
input_contract=BackendSafeLoopCondContinueWithReturnPlanRuleTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/loop_cond_continue_with_return_plan_rule.hako
```

This is not a HakoAdopted decision yet.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-cond-continue-with-return-plan-rule-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/loop_cond_continue_with_return_plan_rule.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_loop_cond_continue_with_return_plan_rule_parity_gate.sh
oracle_rows=5
parity_status=green
```

## Non-Claims

```text
source_selfhost_claim=0
hako_adopted_decision=0
build_plan_with_facts_ctx_migrated=0
try_build_outcome_migrated=0
recipe_matching_migrated=0
route_execution_migrated=0
backend_lowering_migration=0
mir_mutation_migration=0
id_allocation_migration=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-LOOP-COND-CONTINUE-WITH-RETURN-PLAN-RULE-HAKOADOPTED-DECISION-001
```
