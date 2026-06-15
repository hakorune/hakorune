---
Status: Active
Date: 2026-06-15
Task: EXACT-OBJECT-PLAN-SHADOW-001
Scope: Generate exact-object storage plan candidates as a shadow report only.
  Do not change execution.
Related:
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-711-OBJECT-STORAGE-PLAN-SSOT-001.md
  - src/object_storage_plan.rs
---

# EXACT-OBJECT-PLAN-SHADOW-001

## Purpose

Use the ObjectStoragePlan vocabulary to classify exact-object candidates from
the object-lifecycle MIR evidence without changing runtime behavior.

This row decides whether one pilot candidate is strong enough for
`EXACT-OBJECT-PILOT-001`.

## Required Output

```text
output_contract=hako-exact-object-plan-shadow-v0
source_evidence=296x-711
target_front=object_lifecycle_body
object_storage_plan_vocabulary_defined=1
object_storage_plan_execution_enabled=0
exact_object_shadow_enabled=1
generic_box_plan_count=<n>
host_handle_escaped_plan_count=<n>
arc_dynbox_plan_count=<n>
exact_stack_object_plan_count=<n>
exact_native_struct_plan_count=<n>
scalarized_plan_count=<n>
selected_pilot_candidate=<candidate|none>
selected_pilot_confidence=<low|medium|high>
pilot_allowed=<0|1>
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
summary=ok
```

## Stop Line

```text
do not change backend lowering
do not change MIRBuilder behavior
do not change runtime object representation
do not retire Arc
do not remove HostHandle
do not change product defaults
```

## Handoff

```text
next_task=EXACT-OBJECT-PILOT-001
```
