---
Status: Landed
Date: 2026-05-28
Scope: quantify typed-object helper lock/global-slab cost before runtime fast-lane work.
Blocker: TYPED-OBJECT-HELPER-LOCK-COST-PROBE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-188-TYPED-OBJECT-FIELD-HELPER-FAST-LANE-SELECTION.md
---

# 296x-189 Typed Object Helper Lock Cost Probe

## Purpose

Run a one-row exit-ramp probe before implementing a typed-object runtime fast
lane. Row187/188 already selected typed-object field helpers as the large owner.
This row does not search for a new owner; it splits the selected helper cost
into lock/global-slab, slot lookup, and enum/value pieces so the next keeper can
be chosen cleanly.

## Current Owner

```text
current_owner=exact_typed_field_access_to_public_runtime_typed_object_storage_transition
hot_transition=slot_indexed_scalar_field_op_to_mutex_protected_global_vec_typed_slot_object
perf_field_helper_pct=72.96
field_dynamic_estimate=30072832
```

## Probe Contract

```text
output_contract=typed-object-helper-lock-cost-probe-v0
input_contract=hako-mimalloc-field-array-runtime-boundary-probe-v0
workload_id=representative-object-lifecycle-small-block-v0
field_dynamic_estimate=30072832
perf_field_helper_pct=72.96

lock_unlock_ns_per_op=<positive>
mutex_vec_read_ns_per_op=<positive>
mutex_vec_write_ns_per_op=<positive>
handle_to_index_ns_per_op=<positive>
slot_normalize_ns_per_op=<positive>
typed_slot_match_ns_per_op=<positive>
u64_exact_get_ns_per_op=<positive>
u64_exact_set_ns_per_op=<positive>
hii_get_ns_per_op=<positive>
hii_set_ns_per_op=<positive>

dominant_helper_subowner=<lock_global_slab|typed_slot_value_repr|helper_call_boundary|mixed_helper_cost>
recommended_next=<runtime_single_thread_fast_lane|typed_slot_repr_fast_lane|mir_scalar_residence_first>
optimization_open=0
summary=ok
```

## Evidence

Guard:

```bash
bash tools/checks/k2_wide_phase296x_typed_object_helper_lock_cost_probe_guard.sh
```

Observed scout output:

```text
output_contract=typed-object-helper-lock-cost-probe-v0
iterations=500000
field_dynamic_estimate=30072832
perf_field_helper_pct=72.96
lock_unlock_ns_per_op=8
mutex_vec_read_ns_per_op=9
mutex_vec_write_ns_per_op=9
handle_to_index_ns_per_op=1
slot_normalize_ns_per_op=1
typed_slot_match_ns_per_op=1
u64_exact_get_ns_per_op=9
u64_exact_set_ns_per_op=10
hii_get_ns_per_op=9
hii_set_ns_per_op=9
lock_fraction_pct=88
storage_lookup_fraction_pct=100
enum_value_fraction_pct=11
dominant_helper_subowner=lock_global_slab
recommended_next=runtime_single_thread_fast_lane
summary=ok
```

## Selection Rules

```text
lock/global storage fraction >= 40%:
  recommended_next=runtime_single_thread_fast_lane

lock is small but enum/value conversion dominates:
  recommended_next=typed_slot_repr_fast_lane

helper whole cost is large but internal subowner is mixed:
  recommended_next=mir_scalar_residence_first
```

## Implementation Boundary

The probe may use a sidecar Rust microprobe compiled by a Python adapter. It must
not change exported typed-object helper behavior.

Allowed:

```text
- add a tool under tools/allocator/
- add a guard under tools/checks/
- model the same Mutex<Vec<TypedSlotObject>> access shape
- report stable contract fields
```

Rejected:

```text
- changing crates/nyash_kernel typed-object helper behavior
- adding by-name hako_alloc special cases
- adding runtime single-thread backend before the probe result
- starting MIR scalar residence implementation
- optimizing ArrayBox in the same row
```

## Acceptance

```text
output_contract=typed-object-helper-lock-cost-probe-v0
dominant_helper_subowner=lock_global_slab
recommended_next=runtime_single_thread_fast_lane
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```
