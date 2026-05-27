---
Status: Landed
Date: 2026-05-27
Scope: inspect the lowered MIR shape for objectLifecycleSmallAlloc before selecting another keeper.
Blocker: HAKO-MIMALLOC-SMALL-ALLOC-MIR-SHAPE-DEEP-DIVE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-113-HAKO-MIMALLOC-POST-ROLLBACK-INLINE-SUCCESS-SOURCE-MIR-REFRESH.md
---

# 296x-114 Hako Mimalloc Small Alloc MIR Shape Deep Dive

## Purpose

`objectLifecycleSmallAlloc` remains the top active owner after two measured
non-keepers. Before another `.hako` keeper is selected, inspect the lowered
shape and classify the actual owner of the remaining cost.

This row is diagnostic-only.

## Required Output

```text
output_contract=hako-mimalloc-small-alloc-mir-shape-deep-dive-v0
input_contract=hako-mimalloc-post-rollback-inline-success-source-mir-refresh-v0
selected_owner=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
mir_instruction_count
mir_call_count
mir_field_access_count
mir_phi_count
mir_copy_count
dominant_shape_owner=method_call|field_access|phi_copy|branching|unknown
next_action=keeper_selection|mir_lowering_probe|measurement_refresh|stop_line
summary=ok
```

## Stop Line

Do not implement a keeper in this row. Do not open provider activation,
replacement, hooks, globals, or winner claims.

## Landed Evidence

```text
output_contract=hako-mimalloc-small-alloc-mir-shape-deep-dive-v0
input_contract=hako-mimalloc-post-rollback-inline-success-source-mir-refresh-v0
selected_owner=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
mir_instruction_count=247
mir_call_count=26
mir_field_access_count=13
mir_phi_count=76
mir_copy_count=94
mir_branch_count=8
dominant_shape_owner=phi_copy
next_action=mir_lowering_probe
top_callee_0=recordSmallAllocFailure
top_callee_0_count=5
top_callee_1=HakoAllocObjectLifecycleFacadeReason.small_no_page/0
top_callee_1_count=4
top_callee_2=HakoAllocObjectLifecycleFacadeReason.small_bad_selected_kind/0
top_callee_2_count=2
top_callee_3=HakoAllocObjectLifecycleFacadeReason.small_reuse_failed/0
top_callee_3_count=2
top_callee_4=HakoAllocObjectLifecycleFacadeReason.small_acquire_failed/0
top_callee_4_count=2
top_callee_5=resetSmallAllocResult
top_callee_5_count=1
top_callee_6=recordAttempt
top_callee_6_count=1
top_callee_7=beginSelection
top_callee_7_count=1
top_field_0=alloc_result
top_field_0_count=3
top_field_1=last_selected_page_id
top_field_1_count=3
top_field_2=request_count
top_field_2_count=2
top_field_3=object_lifecycle_queue
top_field_3_count=1
top_field_4=page_count
top_field_4_count=1
top_field_5=last_selected_index
top_field_5_count=1
top_field_6=last_selected_page
top_field_6_count=1
top_field_7=last_selected_kind
top_field_7_count=1
next_diagnostic=small_alloc_phi_copy_lowering_probe
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_small_alloc_mir_shape_deep_dive_guard.sh
```
