---
Status: Landed
Date: 2026-05-30
Scope: refresh the post-retirement perf owner after the scoped retirement smoke and decide whether the lane can move to ArrayRepr design.
Blocker: ARRAY-REPR-DESIGN-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-376-ARRAY-SLOT-NATIVEDIRECT-LEGACY-HELPER-CACHE-RETIREMENT-SEMANTIC-SMOKE.md
  - docs/development/current/main/phases/phase-296x/296x-375-ARRAY-SLOT-NATIVEDIRECT-LEGACY-HELPER-CACHE-RETIREMENT-IMPLEMENTATION.md
  - docs/development/current/main/phases/phase-296x/296x-374-DIRECTARRAY-FAMILY-STORAGE-SUBSTRATE-ROADMAP.md
  - docs/development/current/main/phases/phase-296x/296x-378-ARRAY-REPR-DESIGN-ROW.md
  - tools/allocator/array_slot_nativedirect_post_retirement_perf_owner_refresh.py
  - tools/checks/k2_wide_phase296x_array_slot_nativedirect_post_retirement_perf_owner_refresh_guard.sh
---

# 296x-377 ArraySlot NativeDirect Post-Retirement Perf Owner Refresh

## Purpose

Refresh the hot-owner classification after the scoped retirement smoke.

The smoke from row376 proved the retired slice is structurally valid. This row
checks whether the DirectArray runtime path now dominates enough to move the
lane toward `ArrayRepr` design rather than reopening helper micro-optimization.

This row does not delete code. It only classifies the owner and selects the
next design row.

## Contract

```text
output_contract=array-slot-nativedirect-post-retirement-perf-owner-refresh-v0
input_contract=array-slot-nativedirect-legacy-helper-cache-retirement-semantic-smoke-v0
workload_id=representative-object-lifecycle-small-block-v0
attribution_source=perf_callgraph
selected_method=HakoAllocPageModel.acquire_usize/1
direct_array_backend_store_pct=...
direct_array_backend_load_pct=...
direct_array_backend_direct_op_pct=...
direct_array_backend_total_pct=...
legacy_field_helper_pct=...
legacy_array_helper_pct=...
arraybox_public_helper_pct=...
legacy_hash_pct=...
legacy_helper_cache_total_pct=...
hako_method_pct=...
direct_array_dominates_legacy_helper_cache=1
array_repr_design_open=1
selected_boundary=array_repr_design_row
next_diagnostic=array_repr_design_row
selected_next=array_repr_design_row
selected_reason=direct_array_path_ready_for_arrayrepr_design_after_retirement_smoke
legacy_retirement_candidate_0=single_thread_exact_array_helper_backend
legacy_retirement_candidate_1=array_slot_handle_entry_cache
legacy_retirement_candidate_2=array_slot_public_helper_fast_lane
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

If the DirectArray path still dominates after the scoped retirement smoke, the
next row should introduce the `ArrayRepr` design bridge instead of adding
another helper micro-lane.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_array_slot_nativedirect_post_retirement_perf_owner_refresh_guard.sh
```
