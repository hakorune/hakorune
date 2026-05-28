---
Status: Provisional
Date: 2026-05-28
Scope: MIR ArrayBox slot residence / DirectSlotOp design owner after Array runtime backend floor measurement.
Related:
  - docs/development/current/main/phases/phase-296x/296x-206-ARRAY-RUNTIME-SINGLE-THREAD-STORE-BACKEND-KEEPER-MEASUREMENT.md
---

# MIR Array Slot Residence SSOT

## Purpose

Define the C-parity target after the ArrayBox runtime helper backend floor was
measured. The runtime `SingleThreadExact` backend is a useful diagnostic fast
lane, but C-like performance ultimately requires hot ArrayBox get/set helpers
to disappear from selected MIR methods.

This document defines the design owner only. It does not open a MIR transform.

## Decision

```text
Decision: provisional

mir_array_slot_residence_ssot=accepted
runtime_array_backend_floor=measured
array_helper_abi_fallback=1
transform_open=0
positive_net_helper_call_delta_required=1
by_name_hako_alloc_special_case=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Planned ArraySlotResidencePlan Shape

```text
ArraySlotResidencePlan:
  function
  receiver_value
  array_identity
  storage_class:
    - InlineI64
  index_class:
    - constant_index
    - loop_index_range_proven
    - append_at_end_proven
  operation:
    - get_i64
    - set_i64
    - rmw_i64
  residence_kind:
    - method_local_slot_cache
    - direct_slot_op
  init_policy:
    - helper_load_on_first_use
    - default_empty_inline_i64
  writeback_policy:
    - no_writeback_readonly
    - helper_writeback_before_escape
    - helper_writeback_on_return
  fallback_helper:
    - array_runtime_get_idx
    - array_runtime_set_idx_i64
    - array_slot_load_encoded_i64
    - array_slot_store_i64
```

## Barriers

```text
unknown_call:
  barrier

array_handle_escape:
  barrier

storage_kind_change:
  barrier

boxed_fallback_required:
  barrier

aliasing_write:
  barrier

phi_merge:
  barrier unless the inventory can prove one array identity and one compatible
  slot state across all incoming edges

dynamic_slot_without_range_proof:
  barrier
```

## Required Inventory Before Transform

Implementation must not start until an inventory row reports positive net
helper-call delta:

```text
net_helper_call_delta =
  erased_get_set_helper_calls
  - added_guard_helper_calls
  - added_writeback_helper_calls

required:
  net_helper_call_delta > 0
```

The inventory must also report:

```text
eligible_array_get_count
eligible_array_set_count
erased_get_set_helper_calls
added_guard_helper_calls
added_writeback_helper_calls
barrier_unknown_call_count
barrier_escape_count
barrier_phi_count
barrier_storage_kind_count
selected_method
summary=ok
```

## Non-Goals

```text
- Do not transform MIR in the SSOT row.
- Do not remove ArrayBox runtime helper ABI.
- Do not change public ArrayBox semantics.
- Do not replace provider / host allocator behavior.
- Do not add hako_alloc box-name or field-name special cases.
- Do not treat the runtime SingleThreadExact backend as the final C-parity
  design.
```
