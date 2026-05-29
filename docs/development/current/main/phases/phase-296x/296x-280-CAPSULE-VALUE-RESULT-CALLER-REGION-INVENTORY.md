---
Status: Landed
Date: 2026-05-29
Scope: inventory caller regions for recordSuccess value-result materialization.
Blocker: CAPSULE-VALUE-RESULT-CALLER-REGION-INVENTORY-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-279-CAPSULE-VALUE-RESULT-PLAN-INVENTORY.md
---

# 296x-280 Capsule Value Result Caller Region Inventory

## Purpose

Inventory whether recordSuccess value deltas can be carried through caller
regions and materialized later at known observer boundaries.

This row is observation-only. It decides whether to continue the
ValueAggregate lane or return to a narrow helper-fusion guard surface.

## Evidence

```text
output_contract=capsule-value-result-caller-region-inventory-v0
input_contract=capsule-value-result-plan-inventory-v0
workload_id=representative-object-lifecycle-small-block-v0
record_success_callsite_count=3
immediate_return_callsite_count=3
observer_before_return_count=0
unknown_call_after_success_count=0
public_method_return_boundary_count=3
materialization_must_happen_before_public_return=1
caller_region_defer_past_return_allowed=0
caller_region_value_aggregate_net_delta=0
caller_region_value_aggregate_net_delta_positive=0
helper_fusion_net_delta=12
helper_fusion_net_delta_positive=1
selected_next=record_success_helper_fusion_guard_surface
selected_reason=public_method_return_boundary_prevents_value_delta_deferral
rejected_owner=capsule_value_result_implementation
rejected_reason=caller_region_cannot_defer_materialization_past_public_method_return
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
callsite_0_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
callsite_0_block=575
callsite_0_callee=HakoAllocObjectLifecycleAllocResult.recordSuccess
callsite_1_method=HakoAllocObjectLifecycleFacade.recordReleaseSuccess/2
callsite_1_block=590
callsite_1_callee=HakoAllocObjectLifecycleReleaseResult.recordSuccess
callsite_2_method=HakoAllocObjectLifecycleFacade.recordSmallAllocSuccess/1
callsite_2_block=592
callsite_2_callee=HakoAllocObjectLifecycleAllocResult.recordSuccess
summary=ok
```

## Decision

```text
selected_next=record_success_helper_fusion_guard_surface
next_row=record_success_helper_fusion_guard_surface
optimization_open=0
```

The caller-region inventory confirms that public method returns are the
materialization boundary. ValueAggregate does not currently produce a positive
net helper delta for this surface. The next row may return to the narrow
recordSuccess helper-fusion guard surface, now with the representation lane
explicitly rejected for this shape.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_capsule_value_result_caller_region_inventory_guard.sh
```
