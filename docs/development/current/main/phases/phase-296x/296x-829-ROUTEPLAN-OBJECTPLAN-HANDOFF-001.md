---
Status: Landed
Date: 2026-06-16
Task: ROUTEPLAN-OBJECTPLAN-HANDOFF-001
Scope: Fix the RoutePlan/ObjectPlan backend handoff contract without enabling new lowering.
Related:
  - docs/development/current/main/design/compiler-object-final-shape-ssot.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-828-OBJECTPLAN-PASSIVE-UNIFY-001.md
---

# ROUTEPLAN-OBJECTPLAN-HANDOFF-001

## Purpose

`ObjectPlan` now names representation + publication-site truth. This row fixes
how it may combine with RoutePlan.

The handoff is deliberately narrow:

```text
RoutePlan:
  proves call/new/drop execution route

ObjectPlan:
  proves object representation and publication state

Backend:
  may direct-call only from RoutePlan
  may bypass representation boundary only from ObjectPlan
  may combine both only when both proofs are present
```

## Result

```text
output_contract=hako-routeplan-objectplan-handoff-v0
source_evidence=296x-825,296x-828
routeplan_objectplan_handoff_contract_defined=1
routeplan_owns_execution_not_representation=1
objectplan_owns_representation_not_execution=1
backend_requires_routeplan_for_direct_call=1
backend_requires_objectplan_for_representation_bypass=1
backend_direct_call_without_routeplan_enabled=0
backend_representation_bypass_without_objectplan_enabled=0
backend_helper_symbol_inference_enabled=0
backend_method_name_special_case_enabled=0
backend_variable_name_special_case_enabled=0
objectplan_execution_enabled=0
routeplan_representation_truth_enabled=0
standalone_publication_plan_enabled=0
product_default_changed=0
selected_next=PUBLICATION-SITE-INVENTORY-GENERIC-001
summary=ok
```

## Allowed Combination

```text
direct call:
  requires RoutePlan

direct field/native/scalar/HostHandle bypass:
  requires ObjectPlan

C-like exact-AOT lowering:
  requires RoutePlan + ObjectPlan
```

## Forbidden Combination

```text
RoutePlan alone cannot prove storage or publication.
ObjectPlan alone cannot prove callable target.
Backend cannot infer either proof from helper symbol, method name, variable
name, benchmark name, or source shape.
```

## Stop Line

```text
do not enable backend lowering in this row
do not make RoutePlan own object representation
do not make ObjectPlan own call execution
do not bypass HostHandle from RoutePlan alone
do not direct-call from ObjectPlan alone
do not infer proof from helper/method/variable names
```

## Proof

```bash
bash tools/checks/k2_wide_phase296x_routeplan_objectplan_handoff_guard.sh
cargo test --lib object_storage_plan
```
