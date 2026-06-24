---
Status: Landed
Date: 2026-06-16
Task: BACKEND-PLAN-CONSUMER-GUARD-001
Scope: Guard backend plan consumption boundaries without adding a new lowering path.
Related:
  - docs/development/current/main/phases/phase-296x/296x-829-ROUTEPLAN-OBJECTPLAN-HANDOFF-001.md
  - docs/development/current/main/phases/phase-296x/296x-830-PUBLICATION-SITE-INVENTORY-GENERIC-001.md
  - docs/development/current/main/design/compiler-object-final-shape-ssot.md
---

# BACKEND-PLAN-CONSUMER-GUARD-001

## Purpose

The backend already has a guarded exact-object consumer seam for flattened
nested fields. This row does not add another lowering path. It fixes the
generic rule for future backend consumers:

```text
direct call:
  RoutePlan required

representation bypass / direct field / native/scalar lowering:
  ObjectPlan required

combined C-like lowering:
  RoutePlan + ObjectPlan required
```

## Result

```text
output_contract=hako-backend-plan-consumer-guard-v0
source_evidence=296x-829,296x-830
backend_plan_consumer_guard_enabled=1
backend_plan_consumer_requires_routeplan_and_objectplan=1
backend_existing_flattened_nested_consumer_allowed=1
backend_new_lowering_enabled=0
backend_direct_call_without_routeplan_enabled=0
backend_representation_bypass_without_objectplan_enabled=0
backend_helper_symbol_inference_enabled=0
backend_method_name_special_case_enabled=0
backend_variable_name_special_case_enabled=0
routeplan_owns_execution_not_representation=1
objectplan_owns_representation_not_execution=1
product_default_changed=0
selected_next=COMPILER-OBJECT-SHAPE-CLOSEOUT-001
summary=ok
```

## Existing Consumer

The existing flattened-nested-field consumer remains allowed as the previously
guarded exact-object pilot seam.

```text
backend_existing_flattened_nested_consumer_allowed=1
```

This row does not create a new consumer:

```text
backend_new_lowering_enabled=0
```

## Stop Line

```text
do not add a new backend lowering path in this row
do not infer direct call from helper names
do not infer direct call from method names
do not infer object representation from variable names
do not bypass HostHandle without ObjectPlan
do not bypass RoutePlan for callable target
```

## Proof

```bash
bash tools/checks/k2_wide_phase296x_backend_plan_consumer_guard.sh
cargo test --lib object_storage_plan
```
