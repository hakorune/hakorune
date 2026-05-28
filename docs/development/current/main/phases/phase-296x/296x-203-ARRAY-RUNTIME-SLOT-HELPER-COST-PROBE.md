---
Status: Current
Date: 2026-05-28
Scope: split ArrayBox runtime slot helper cost before keeper implementation.
Blocker: ARRAY-RUNTIME-SLOT-HELPER-COST-PROBE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-202-ARRAY-RUNTIME-SLOT-HELPER-SELECTION.md
  - tools/allocator/array_runtime_slot_helper_cost_probe.py
---

# 296x-203 Array Runtime Slot Helper Cost Probe

## Purpose

Split the selected `array_runtime_set_idx_i64` hot path into subowners before
any keeper implementation. This row is measurement/diagnostic only.

## Contract

```text
output_contract=array-runtime-slot-helper-cost-probe-v0
input_contract=large-owner-refresh-after-residence-zero-net-v0
iterations=<positive>
valid_handle_idx_ns_per_op=<positive>
handle_cache_with_array_box_ns_per_op=<positive>
array_storage_write_lock_ns_per_op=<positive>
inline_i64_store_ns_per_op=<positive>
array_slot_store_i64_ns_per_op=<positive>
array_runtime_set_idx_i64_ns_per_op=<positive>
dominant_subowner=<array_storage_write_lock|facade_boundary|handle_cache_lookup|inline_i64_store|mixed>
recommended_next=<single_thread_array_store_backend|array_slot_direct_emit_or_inline_facade|array_handle_cache_fast_lane|slot_store_i64_raw_fast_lane|measurement_refresh>
optimization_open=0
summary=ok
```

## Acceptance

```text
output_contract=array-runtime-slot-helper-cost-probe-v0
dominant_subowner=array_storage_write_lock
recommended_next=single_thread_array_store_backend
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_array_runtime_slot_helper_cost_probe_guard.sh
```
