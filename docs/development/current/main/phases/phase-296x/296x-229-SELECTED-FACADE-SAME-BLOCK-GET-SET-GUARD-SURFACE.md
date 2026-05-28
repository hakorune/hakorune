---
Status: Landed
Date: 2026-05-29
Scope: freeze selected facade same-block get/set fusion guard surface.
Blocker: SELECTED-FACADE-SAME-BLOCK-GET-SET-GUARD-SURFACE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-228-OBJECT-LIFECYCLE-FACADE-FIELD-OWNER-SELECTION.md
---

# 296x-229 Selected Facade Same-Block Get/Set Guard Surface

## Purpose

Freeze the exact candidates for selected-facade same-block `field_get -> add ->
field_set` fusion before implementation.

This row refines row228's inventory estimate. Scanning all
`HakoAllocObjectLifecycleFacade.*` methods finds six fusible `usize` candidates,
not three. This is still a narrow facade-family owner and does not open generic
typed-field residence.

## Evidence

```text
output_contract=selected-facade-same-block-get-set-guard-surface-v0
input_contract=object-lifecycle-facade-field-owner-selection-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_owner=selected_facade_same_block_get_set_fusion
target_family=object_lifecycle_facade
candidate_count=6
candidate_i64_count=0
candidate_usize_count=6
candidate_u64_count=0
planned_erased_get_set_helper_calls=12
planned_added_fused_helper_calls=6
planned_net_helper_call_delta=6
planned_net_helper_call_delta_positive=1
runtime_storage_owner_preserved=1
helper_free_direct_op_rejected=1
generic_residence_open=0
source_rewrite=0
candidate_0_method=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
candidate_0_block=513
candidate_0_field=HakoAllocObjectLifecycleFacade.release_known_page_fast_path_count
candidate_0_storage=usize
candidate_1_method=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseKnownPageIndex/1
candidate_1_block=525
candidate_1_field=HakoAllocObjectLifecycleFacade.release_known_page_fallback_count
candidate_1_storage=usize
candidate_2_method=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseKnownPageIndex/1
candidate_2_block=535
candidate_2_field=HakoAllocObjectLifecycleFacade.release_known_page_fast_path_count
candidate_2_storage=usize
candidate_3_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
candidate_3_block=552
candidate_3_field=HakoAllocObjectLifecycleAllocResult.attempt_count
candidate_3_storage=usize
candidate_4_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
candidate_4_block=553
candidate_4_field=HakoAllocObjectLifecyclePageQueue.request_count
candidate_4_storage=usize
candidate_5_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAllocAligned/2
candidate_5_block=578
candidate_5_field=HakoAllocObjectLifecycleAllocResult.attempt_count
candidate_5_storage=usize
selected_next=selected_facade_same_block_get_set_keeper
by_name_special_case=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_next=selected_facade_same_block_get_set_keeper
next_row=selected_facade_same_block_get_set_keeper
```

Implementation should add or reuse a runtime-owned fused exact-slot `usize`
add helper and lower only these same-block patterns. It must keep typed-object
storage ownership inside Rust runtime code and preserve fallback semantics for
all non-selected field access.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_selected_facade_same_block_get_set_guard_surface_guard.sh
```
