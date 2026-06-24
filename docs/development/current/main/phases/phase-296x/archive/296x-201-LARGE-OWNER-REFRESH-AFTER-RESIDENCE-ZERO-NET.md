---
Status: Landed
Date: 2026-05-28
Scope: refresh the large owner after typed-field residence selected-method plans have zero net helper-call erasure.
Blocker: LARGE-OWNER-REFRESH-AFTER-RESIDENCE-ZERO-NET-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-187-FIELD-ARRAY-RUNTIME-LOWERING-BOUNDARY-PROBE.md
  - docs/development/current/main/phases/phase-296x/296x-192-TYPED-OBJECT-RUNTIME-FAST-LANE-KEEPER-MEASUREMENT.md
  - docs/development/current/main/phases/phase-296x/296x-200-CFG-AWARE-TYPED-FIELD-RESIDENCE-PLAN-INVENTORY.md
---

# 296x-201 Large Owner Refresh After Residence Zero Net

## Purpose

Refresh the large owner before any more optimization. Row192 accepted the
typed-object single-thread exact store as the runtime floor, and row197/200
showed that selected-method typed-field residence for
`HakoAllocPageModel.acquire_usize/1` has `net_helper_call_delta=0`.

This row reruns the field/Array runtime boundary probe against a fresh
single-thread-exact perf scout and selects the next owner.

## Evidence

```text
output_contract=hako-mimalloc-field-array-runtime-boundary-probe-v0
input_contract=object-lifecycle-large-owner-reality-check-v0
workload_id=representative-object-lifecycle-small-block-v0
field_static_total=80
field_dynamic_estimate=30072832
array_method_static_total=7
array_method_dynamic_estimate=2121728
perf_report_available=1
perf_field_helper_pct=10.51
perf_array_helper_pct=89.49
perf_top_0_pct=69.59
perf_top_0_symbol=nyash_kernel::plugin::array_runtime_facade::array_runtime_set_idx_i64
perf_top_1_pct=19.90
perf_top_1_symbol=nyash_kernel::plugin::array_slot_store::array_slot_store_i64::_$u7b$$u7b$closure$u7d$$u7d$::h7828d98f0aaf784e
perf_top_2_pct=10.51
perf_top_2_symbol=nyash.object.field_get_hii
selected_boundary=array_runtime_slot_helper_lowering
secondary_boundary=typed_object_field_helper_lowering
next_diagnostic=array_runtime_slot_helper_selection
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Selection

```text
selected_owner_family=array_runtime_slot_helper_lowering
selected_reason=single_thread_exact_perf_shifts_samples_to_array_runtime_set_and_slot_store
secondary_owner=typed_object_field_helper_call_boundary
rejected_owner=selected_method_typed_field_residence
rejected_reason=net_helper_call_delta_zero_for_acquire_usize
confidence=medium
```

The scout has few samples, but the direction is strong enough to stop the
typed-field residence path and open an ArrayBox-specific diagnostic. The next
row must not implement a keeper yet; it should split `ArrayBox.set/get` helper
cost into facade boundary, handle cache, slot store, and value representation.

## Next

```text
row202:
  array_runtime_slot_helper_selection

Goal:
  define the ArrayBox runtime slot helper diagnostic boundary before any
  runtime/compiler changes.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_large_owner_refresh_after_residence_zero_net_guard.sh
```
