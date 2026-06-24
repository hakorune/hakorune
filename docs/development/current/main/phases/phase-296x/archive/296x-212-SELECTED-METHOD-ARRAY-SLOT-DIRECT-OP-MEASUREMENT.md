---
Status: Landed
Date: 2026-05-28
Scope: measure the selected-method Array slot direct-op keeper and refresh the next hot owner.
Blocker: SELECTED-METHOD-ARRAY-SLOT-DIRECT-OP-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-211-SELECTED-METHOD-ARRAY-SLOT-DIRECT-OP-KEEPER.md
  - docs/development/current/main/phases/phase-296x/296x-201-LARGE-OWNER-REFRESH-AFTER-RESIDENCE-ZERO-NET.md
---

# 296x-212 Selected Method Array Slot Direct Op Measurement

## Purpose

Measure the fused `nyash.array.slot_load_store_i64_hihi` keeper and refresh the
large owner after the ArrayBox helper pair is no longer the dominant boundary.

This row does not open a new optimization. It fixes the current post-fusion
evidence and selects the next diagnostic owner.

## Measurement

```text
output_contract=array-runtime-single-thread-store-backend-keeper-measurement-v0
sample_count=3
typed_object_backend=single_thread_exact
safe_rwlock_body_elapsed_ns=214000000
single_thread_exact_body_elapsed_ns=123000000
body_elapsed_delta_ns=91000000
single_thread_exact_body_ratio_pct=57
safe_rwlock_external_elapsed_ms=220
single_thread_exact_external_elapsed_ms=120
keeper_effect=accepted
runtime_fast_lane_keeper=1
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Owner Refresh

```text
output_contract=selected-method-array-slot-direct-op-post-fusion-owner-refresh-v0
input_contract=selected-method-array-slot-direct-op-keeper-v0
workload_id=representative-object-lifecycle-small-block-v0
perf_field_helper_pct=51.04
perf_array_slot_backend_pct=18.14
perf_fused_direct_op_pct=0.89
perf_array_backend_hash_pct=19.96
perf_array_total_pct=38.99
perf_top_0_symbol=nyash.object.field_set_hii
perf_top_1_symbol=core::hash::BuildHasher::hash_one
perf_top_2_symbol=nyash.object.field_set_u64_hiu
perf_top_3_symbol=nyash_kernel::plugin::array_slot_backend::single_thread_store_i64
perf_top_4_symbol=nyash.object.field_get_hii
perf_top_5_symbol=nyash.object.field_get_u64_hii
perf_top_6_symbol=nyash_kernel::plugin::array_slot_backend::single_thread_load_encoded_i64
selected_boundary=typed_object_field_helper_lowering
secondary_boundary=array_slot_backend_handle_map_hash
next_diagnostic=typed_object_field_helper_subowner_refresh
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_owner_family=typed_object_field_helper_lowering
secondary_owner_family=array_slot_backend_handle_map_hash
selected_reason=post_fusion_perf_has_field_helpers_above_array_backend_total
next_row=typed_object_field_helper_subowner_refresh
```

The fused Array slot direct op is structurally active, but the next largest
owner is again typed-object field helper lowering. The Array single-thread
backend still has visible `HashMap` cost from handle-to-slot-vector lookup; that
is a secondary owner, not the next primary row.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_selected_method_array_slot_direct_op_measurement_guard.sh
```
