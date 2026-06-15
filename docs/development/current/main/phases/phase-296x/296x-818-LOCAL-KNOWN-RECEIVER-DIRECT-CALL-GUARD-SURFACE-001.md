---
Status: Landed
Date: 2026-06-16
Task: LOCAL-KNOWN-RECEIVER-DIRECT-CALL-GUARD-SURFACE-001
Scope: Define the guard surface for local known receiver direct-call shadowing.
Related:
  - docs/development/current/main/phases/phase-296x/296x-817-LOCAL-PAGE-RECEIVER-CANDIDATE-PROBE-001.md
  - docs/development/current/main/phases/phase-296x/296x-816-LOCAL-FIRST-DIRECT-PILOT-SELECTION-001.md
---

# LOCAL-KNOWN-RECEIVER-DIRECT-CALL-GUARD-SURFACE-001

## Purpose

Define the conditions required before a local known receiver method call may be
shadowed as a direct-call candidate.

This row opens shadowing only. It does not implement direct calls and does not
authorize backend lowering.

## Guard Report

```text
output_contract=hako-local-known-receiver-direct-call-guard-surface-v0
source_evidence=296x-817,296x-816
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1

selected_shape=local_known_receiver_direct_call
first_target_receiver=page
first_target_methods=acquire_usize,reuse
first_target_call_count=3

guard_receiver_pre_publication_required=1
guard_receiver_type_known_required=1
guard_method_surface_known_required=1
guard_dynamic_api_absent_required=1
guard_plugin_or_extern_absent_required=1
guard_task_boundary_absent_required=1
guard_page_call_after_publication_required_zero=1

storage_direct_required=0
storage_direct_enabled=0
hosthandle_bypass_enabled=0
arc_retirement_enabled=0
page_specific_rule_enabled=0
method_name_special_case_enabled=0
helper_symbol_inference_enabled=0

routeplan_backend_consumable_proof_required_before_implementation=1
shadow_allowed=1
implementation_allowed=0
product_default_changed=0

next_task=LOCAL-KNOWN-RECEIVER-DIRECT-CALL-SHADOW-001
summary=ok
```

## Required Shape

Shadowing may only classify a call as a candidate when all of these are true:

```text
receiver is pre-publication
receiver type is known
method surface is known
dynamic NyashBox API is absent
plugin / extern escape is absent
task / Future / Channel boundary is absent
call occurs before receiver publication
```

This guard deliberately does not require storage direct proof, because this
pilot is not an object-storage pilot.

## Implementation Boundary

Implementation remains closed until a later shadow row proves a generic
ObjectPlan + RoutePlan rule.

```text
OK later:
  if ObjectPlan says receiver is pre-publication
  and RoutePlan says method target is backend-consumable direct
  then direct call may be considered

NG:
  if receiver variable name == page
  if method name == acquire_usize or reuse
  if helper symbol name implies direct route
```

## Stop Line

```text
do not implement direct call from this row
do not bypass HostHandle
do not open storage direct route
do not retire Arc
do not special-case page receiver name
do not special-case acquire_usize or reuse
do not infer from helper symbol
do not change product default runtime behavior
```

## Next Task

```text
LOCAL-KNOWN-RECEIVER-DIRECT-CALL-SHADOW-001
```
