---
Status: Landed
Date: 2026-05-28
Scope: split typed-object field helper cost after Array direct-op fusion before selecting another keeper.
Blocker: TYPED-OBJECT-FIELD-HELPER-SUBOWNER-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-212-SELECTED-METHOD-ARRAY-SLOT-DIRECT-OP-MEASUREMENT.md
  - docs/development/current/main/phases/phase-296x/296x-192-TYPED-OBJECT-RUNTIME-FAST-LANE-KEEPER-MEASUREMENT.md
---

# 296x-213 Typed Object Field Helper Subowner Refresh

## Purpose

Refresh the typed-object field-helper owner after the selected Array slot direct
op moved ArrayBox helper cost below typed-object field helper cost.

This row is observation-only. It does not change runtime behavior, MIR lowering,
or `.hako` source.

## Evidence

```text
output_contract=typed-object-field-helper-subowner-refresh-v0
input_contract=selected-method-array-slot-direct-op-post-fusion-owner-refresh-v0
workload_id=representative-object-lifecycle-small-block-v0
perf_field_helper_pct=59.42
perf_symbol_pct.nyash.object.field_set_hii=23.70
perf_symbol_pct.nyash.object.field_set_u64_hiu=14.17
perf_symbol_pct.nyash.object.field_get_hii=11.92
perf_symbol_pct.nyash.object.field_get_u64_hii=9.63
annotate_local_pct.backend_tls_entry=6.63
annotate_local_pct.control_validation_branch=99.47
annotate_local_pct.direct_vec_field_access=39.74
annotate_local_pct.prologue_validation=40.62
annotate_local_pct.return_epilogue=10.00
annotate_local_pct.safe_mutex_fallback=0.00
annotate_local_pct.unknown=3.53
dominant_field_helper_subowner=control_validation_branch
secondary_field_helper_subowner=backend_tls_entry
rejected_owner=array_slot_backend_handle_map_hash
rejected_reason=secondary_owner_below_typed_object_field_helper
recommended_next=typed_object_exact_slot_direct_helper_selection
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Interpretation

The `single_thread_exact` store removed the original global lock cost, but the
exported field helpers still carry a broad generic shape:

- backend selection / thread-local entry checks;
- negative handle and slot validation;
- Vec object lookup and field lookup;
- `TypedSlotValue` storage checks for legacy i64 and exact u64 paths;
- safe-mutex fallback branches remain in the emitted helper body even when the
  exact lane is selected.

The next row should not retry broad MIR residence yet. The narrower next owner
is an exact-lane runtime helper seam for typed-object field get/set that keeps
default helpers unchanged and lets exact-EXE lowering call a helper with fewer
generic branches.

## Decision

```text
selected_owner_family=typed_object_exact_slot_direct_helper
selected_reason=field_helpers_remain_primary_and_annotate_is_branch_validation_heavy
secondary_owner_family=array_slot_backend_handle_map_hash
rejected_owner=generic_typed_field_residence_retry
rejected_reason=previous_selected_method_residence_had_net_helper_call_delta_zero
next_row=typed_object_exact_slot_direct_helper_selection
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_typed_object_field_helper_subowner_refresh_guard.sh
```
