---
Status: Landed
Date: 2026-06-16
Task: COMPILER-OBJECT-FINAL-SHAPE-001
Scope: Fix the final compiler object-shape boundary before selfhost MIRBuilder
  growth.
Related:
  - docs/development/current/main/design/compiler-object-final-shape-ssot.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-824-MIMALLOC-CURRENT-FRONT-OPTIMIZATION-PAUSE-CHECKPOINT-001.md
---

# COMPILER-OBJECT-FINAL-SHAPE-001

## Purpose

The current mimalloc front is paused.  Before selecting another optimization
front or growing selfhost MIRBuilder support, this row fixes the object-shape
compiler boundary.

The goal is not to enable a new fast path.  The goal is to make the final boxes
clear enough that future selfhost work does not put representation truth in
MIRBuilder.

## Result

```text
output_contract=hako-compiler-object-final-shape-v0
compiler_object_final_shape_contract=hako-compiler-object-final-shape-v0
source_evidence=296x-824
mirbuilder_object_management_enabled=0
mirbuilder_records_object_meaning=1
semantic_refresh_owns_object_facts=1
box_callable_registry_is_callable_truth=1
routeplan_is_call_execution_truth=1
objectplan_is_representation_truth=1
objectplan_is_publication_site_truth=1
standalone_publication_plan_enabled=0
backend_consumes_routeplan_and_objectplan=1
backend_helper_symbol_inference_enabled=0
backend_method_name_special_case_enabled=0
backend_variable_name_special_case_enabled=0
runtime_generic_box_world_preserved=1
product_default_changed=0
selfhost_mirbuilder_metadata_only=1
implementation_started=0
selected_next=MIRBUILDER-OBJECT-BOUNDARY-GUARD-001
summary=ok
```

## Interpretation

The final object-shape compiler boundary is:

```text
MIRBuilder:
  meaning only

SemanticRefresh / Analysis:
  facts

BoxCallableRegistry:
  callable truth

RoutePlan:
  execution truth

ObjectPlan:
  representation + publication-site truth

Backend:
  consumes RoutePlan + ObjectPlan

Runtime:
  product generic fallback world
```

## Stop Line

```text
do not enable lowering in this row
do not move object representation into MIRBuilder
do not create standalone PublicationPlan in this row
do not add backend helper/method/variable-name special cases
do not bypass HostHandle
do not retire Arc
do not change product default runtime behavior
```

## Proof

```bash
bash tools/checks/k2_wide_phase296x_compiler_object_final_shape_guard.sh
```
