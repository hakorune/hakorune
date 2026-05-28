---
Status: Current
Date: 2026-05-28
Scope: select the ArrayBox runtime slot helper diagnostic boundary before any keeper implementation.
Blocker: ARRAY-RUNTIME-SLOT-HELPER-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-201-LARGE-OWNER-REFRESH-AFTER-RESIDENCE-ZERO-NET.md
---

# 296x-202 Array Runtime Slot Helper Selection

## Purpose

Select the first ArrayBox runtime helper diagnostic after row201 showed that
single-thread-exact perf samples are dominated by ArrayBox set/slot-store code.
This row is selection-only. It does not change runtime/compiler behavior.

## Hot Path

```text
selected_owner_family=array_runtime_slot_helper_lowering
hot_symbol_0=nyash_kernel::plugin::array_runtime_facade::array_runtime_set_idx_i64
hot_symbol_0_pct=69.59
hot_symbol_1=nyash_kernel::plugin::array_slot_store::array_slot_store_i64
hot_symbol_1_pct=19.90
```

Current Rust path:

```text
array_runtime_set_idx_i64(handle, idx, value)
  -> array_slot_store_i64(handle, idx, value)
      -> valid_handle_idx(handle, idx)
      -> with_array_box(handle, |arr| ...)
          -> handle cache lookup or host handle lookup/downcast
      -> arr.slot_store_i64_raw(idx, value)
          -> ArrayBox.items.write()
          -> ensure_inline_i64 / ensure_boxed
          -> store or append
```

## Decision

```text
Decision: accepted

selected_next_diagnostic=array_runtime_slot_helper_cost_probe
selected_reason=array_set_and_slot_store_are_dominant_after_typed_object_fast_lane
diagnostic_owner=array_runtime_slot_store_i64_path
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Probe Contract

The next row must split the hot path into subowners before implementation.

```text
output_contract=array-runtime-slot-helper-cost-probe-v0
input_contract=large-owner-refresh-after-residence-zero-net-v0
iterations=<positive>
valid_handle_idx_ns_per_op=<positive>
handle_cache_with_array_box_ns_per_op=<positive>
slot_store_i64_raw_ns_per_op=<positive>
array_runtime_set_idx_i64_ns_per_op=<positive>
array_slot_store_i64_ns_per_op=<positive>
dominant_subowner=<facade_boundary|handle_cache_lookup|array_storage_write_lock|inline_i64_store|mixed>
recommended_next=<single_thread_array_store_backend|slot_store_i64_raw_fast_lane|mir_array_slot_residence|measurement_refresh>
optimization_open=0
summary=ok
```

## Rejected

```text
typed_field_residence_retry:
  rejected because row200 has net_helper_call_delta=0

generic ArrayBox rewrite:
  rejected because this row needs one hot path diagnostic, not broad ArrayBox
  semantics work

provider/replacement/global allocator:
  rejected because this remains an explicit benchmark parity lane
```

## Acceptance

```text
array_runtime_slot_helper_selection=accepted
next_diagnostic=array_runtime_slot_helper_cost_probe
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_array_runtime_slot_helper_selection_guard.sh
```
