---
Status: Landed
Date: 2026-06-15
Task: OBJECT-STORAGE-PLAN-SSOT-001
Scope: Add the code-facing ObjectStoragePlan vocabulary and guard surface after
  296x-710 inventory. This row does not change execution.
Related:
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-710-OBJECT-BOUNDARY-INVENTORY-001.md
---

# OBJECT-STORAGE-PLAN-SSOT-001

## Purpose

Introduce a stable code-facing vocabulary for exact-AOT object representation
planning without enabling object representation changes.

This row prepares `EXACT-OBJECT-PLAN-SHADOW-001`. It is not the pilot.

## Required Output

```text
output_contract=hako-object-storage-plan-ssot-v0
source_evidence=296x-710
mirbuilder_object_management_enabled=0
box_callable_registry_is_callable_truth=1
routeplan_is_call_execution_truth=1
object_storage_plan_is_representation_truth=1
object_storage_plan_vocabulary_defined=1
object_storage_plan_execution_enabled=0
exact_object_shadow_ready=1
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
summary=ok
```

## Allowed Change

```text
Add a report/vocabulary module or docs/guard surface that names:
  GenericBox
  HostHandleEscaped
  ArcDynBox
  ExactStackObject
  ExactNativeStruct
  Scalarized

Do not connect it to lowering.
```

## Stop Line

```text
do not change MIRBuilder behavior
do not change backend lowering
do not change runtime object representation
do not retire Arc
do not remove HostHandle
do not change product defaults
```

## Handoff

```text
next_task=EXACT-OBJECT-PLAN-SHADOW-001
```

## Result

```text
output_contract=hako-object-storage-plan-ssot-v0
source_evidence=296x-710
mirbuilder_object_management_enabled=0
box_callable_registry_is_callable_truth=1
routeplan_is_call_execution_truth=1
object_storage_plan_is_representation_truth=1
object_storage_plan_vocabulary_defined=1
object_storage_plan_execution_enabled=0
exact_object_shadow_ready=1
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
next_task=EXACT-OBJECT-PLAN-SHADOW-001
summary=ok
```

Evidence:

```text
code_vocabulary=src/object_storage_plan.rs
report_fields=object_storage_plan_report_fields
lib_export=pub mod object_storage_plan
```
