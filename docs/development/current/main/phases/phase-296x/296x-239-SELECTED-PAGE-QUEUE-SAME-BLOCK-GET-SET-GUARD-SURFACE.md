---
Status: Landed
Date: 2026-05-29
Scope: freeze selected page queue same-block get/set fusion guard surface.
Blocker: SELECTED-PAGE-QUEUE-SAME-BLOCK-GET-SET-GUARD-SURFACE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-238-PAGE-QUEUE-FIELD-OWNER-SELECTION.md
---

# 296x-239 Selected Page Queue Same-Block Get/Set Guard Surface

## Purpose

Freeze the exact candidates for selected page queue same-block `field_get -> add
-> field_set` fusion before implementation.

This row refines row238's inventory estimate. Scanning all
`HakoAllocObjectLifecyclePageQueue.*` methods in the current MIR finds 21
fusible `usize` candidates, not 12. This is still a narrow page-queue-family
owner and does not open generic typed-field residence.

## Evidence

```text
output_contract=selected-page-queue-same-block-get-set-guard-surface-v0
input_contract=page-queue-field-owner-selection-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_owner=selected_page_queue_same_block_get_set_fusion
target_family=page_queue_helpers
candidate_count=21
candidate_i64_count=0
candidate_usize_count=21
candidate_u64_count=0
planned_erased_get_set_helper_calls=42
planned_added_fused_helper_calls=21
planned_net_helper_call_delta=21
planned_net_helper_call_delta_positive=1
runtime_storage_owner_preserved=1
helper_free_direct_op_rejected=1
generic_residence_open=0
source_rewrite=0
candidate_method_0=HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
candidate_method_0_count=3
candidate_method_1=HakoAllocObjectLifecyclePageQueue.addPage/1
candidate_method_1_count=3
candidate_method_2=HakoAllocObjectLifecyclePageQueue.selectPage/0
candidate_method_2_count=6
candidate_method_3=HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0
candidate_method_3_count=9
candidate_field_0=HakoAllocObjectLifecyclePageQueue.active_select_count
candidate_field_0_count=1
candidate_field_1=HakoAllocObjectLifecyclePageQueue.add_count
candidate_field_1_count=1
candidate_field_2=HakoAllocObjectLifecyclePageQueue.decommitted_skip_count
candidate_field_2_count=2
candidate_field_3=HakoAllocObjectLifecyclePageQueue.miss_count
candidate_field_3_count=5
candidate_field_4=HakoAllocObjectLifecyclePageQueue.page_count
candidate_field_4_count=1
candidate_field_5=HakoAllocObjectLifecyclePageQueue.reject_count
candidate_field_5_count=1
candidate_field_6=HakoAllocObjectLifecyclePageQueue.request_count
candidate_field_6_count=1
candidate_field_7=HakoAllocObjectLifecyclePageQueue.retired_skip_count
candidate_field_7_count=2
candidate_field_8=HakoAllocObjectLifecyclePageQueue.reuse_select_count
candidate_field_8_count=1
candidate_field_9=HakoAllocObjectLifecyclePageQueue.select_count
candidate_field_9_count=1
candidate_field_10=HakoAllocObjectLifecyclePageQueue.single_page_fallback_count
candidate_field_10_count=1
candidate_field_11=HakoAllocObjectLifecyclePageQueue.single_page_fast_path_count
candidate_field_11_count=2
candidate_field_12=HakoAllocObjectLifecyclePageQueue.unavailable_skip_count
candidate_field_12_count=2
selected_next=selected_page_queue_same_block_get_set_keeper
by_name_special_case=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_next=selected_page_queue_same_block_get_set_keeper
next_row=selected_page_queue_same_block_get_set_keeper
```

Implementation should reuse the runtime-owned fused exact-slot `usize` add
helper already used by the selected facade keeper and lower only these
same-block page queue patterns. It must keep typed-object storage ownership
inside Rust runtime code and preserve fallback semantics for all non-selected
field access.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_selected_page_queue_same_block_get_set_guard_surface_guard.sh
```
