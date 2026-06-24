---
Status: Landed
Date: 2026-06-16
Task: LOCAL-KNOWN-RECEIVER-DIRECT-CALL-SHADOW-001
Scope: Report-only shadow for local known receiver direct-call candidates.
Related:
  - docs/development/current/main/phases/phase-296x/296x-818-LOCAL-KNOWN-RECEIVER-DIRECT-CALL-GUARD-SURFACE-001.md
  - docs/development/current/main/phases/phase-296x/296x-817-LOCAL-PAGE-RECEIVER-CANDIDATE-PROBE-001.md
  - tools/allocator/hako_local_known_receiver_direct_call_shadow.py
---

# LOCAL-KNOWN-RECEIVER-DIRECT-CALL-SHADOW-001

## Purpose

Build a report-only shadow route for the selected local known receiver direct
call shape. This row proves that the current front has three guarded direct-call
candidates. It does not lower them.

## Shadow Report

```text
output_contract=hako-local-known-receiver-direct-call-shadow-v0
source_evidence=296x-818,296x-817
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
shadow_kind=report_only
selected_shape=local_known_receiver_direct_call
first_target_receiver=page
shadow_guard_satisfied=1
shadow_direct_call_candidate_count=3
shadow_page_acquire_usize_count=2
shadow_page_reuse_count=1
shadow_route_kind=pre_publication_known_receiver_method_call
shadow_rule_source=objectplan_pre_publication_plus_known_receiver_surface
receiver_name_rule_enabled=0
method_name_rule_enabled=0
helper_symbol_inference_enabled=0
storage_direct_count=0
hosthandle_bypass_count=0
arc_retirement_count=0
routeplan_backend_consumable_proof_required_before_lowering=1
shadow_plan_behavior_changed=0
product_default_changed=0
pilot_implementation_candidate=1
summary=ok
```

## Interpretation

The shadow row authorizes a pilot implementation row to search for a generic
backend seam:

```text
ObjectPlan pre-publication receiver
  + known receiver/method surface
  + backend-consumable RoutePlan proof
  -> direct call candidate
```

The row does not authorize a page-specific branch. `page`, `acquire_usize`, and
`reuse` are evidence for the first target, not rule keys.

## Stop Line

```text
do not implement direct call from shadow alone
do not special-case page receiver name
do not special-case acquire_usize or reuse
do not infer from helper symbol
do not bypass HostHandle
do not open storage direct route
do not retire Arc
do not change product default runtime behavior
```

## Next Task

```text
LOCAL-KNOWN-RECEIVER-DIRECT-CALL-PILOT-001
```

The implementation row must either:

```text
1. find a generic ObjectPlan + RoutePlan backend seam and implement it, or
2. stop for design consultation if no such seam exists.
```
