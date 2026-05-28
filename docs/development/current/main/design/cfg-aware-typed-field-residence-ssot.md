---
Status: Provisional
Date: 2026-05-28
Scope: CFG-aware MIR typed-field residence contract for hot scalar field access.
Related:
  - docs/development/current/main/phases/phase-296x/296x-193-MIR-TYPED-FIELD-RESIDENCE-SSOT.md
  - docs/development/current/main/phases/phase-296x/296x-198-CFG-RESIDENCE-OR-RUNTIME-OWNER-SELECTION.md
---

# CFG-Aware Typed Field Residence SSOT

## Purpose

Define the first compiler-side design that can erase typed-object field helper
calls across CFG boundaries. This is not generic CSE, not a runtime storage
backend, and not a hako_alloc by-name special case.

## Decision

```text
Decision: provisional

owner=cfg_aware_typed_field_residence
primary_goal=erase_exported_typed_object_field_helpers
runtime_helper_abi=fallback
transform_open=0
by_name_special_case=0
generic_cse=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Residence Key

```text
FieldResidenceKey:
  method_symbol
  receiver_value
  receiver_box_id
  field_slot
  declared_scalar_type
```

Only scalar fields with stable typed-object metadata are candidates.

Rejected candidates:

```text
- handle fields
- dynamic or missing declared_type
- dynamic field slot
- weak or alias-sensitive fields
- receiver values that escape without a flush/writeback boundary
```

## CFG Ownership

The residence owner must track both value and dirty state.

```text
ResidentFieldState:
  value_ssa
  dirty_bit
  initialized_bit
```

Required behavior:

```text
field_get:
  if initialized, reuse resident value
  otherwise emit one helper load and mark initialized

field_set:
  update resident value
  mark dirty
  do not emit helper set immediately

merge:
  merge resident value and dirty bit with PHI only when all incoming states are
  representable
  otherwise flush dirty incoming states before the merge and invalidate

loop:
  loop-carried residence requires explicit value and dirty PHIs
  otherwise flush at loop boundary/backedge

unknown call / extern call / effectful mir_call:
  flush dirty resident fields before the call
  invalidate read cache after the call unless the call is proven no-alias for
  the receiver

return:
  flush dirty resident fields before returning from the method
```

## Net Erasure Rule

Do not implement a transform unless the selected-method inventory proves
positive net erasure under this policy.

```text
net_helper_call_delta =
  erased_field_get_count
  + erased_field_set_count
  - inserted_helper_load_count
  - inserted_helper_writeback_count

required:
  net_helper_call_delta > 0
```

Block-local residence without CFG ownership is explicitly rejected after row197:

```text
block_local_residence_feasible=0
net_helper_call_delta=0
```

## Fallback

The typed-object helper ABI remains the fallback for unsupported shapes.

```text
unsupported_shape:
  keep helper get/set calls
  do not silently change semantics

declared static contract failure:
  fail-fast only after an explicit future Contract(static_field_residence)
  marker exists
```

## Non-Goals

```text
- Do not add hako_alloc box/field name checks.
- Do not reopen generic MIR CSE.
- Do not optimize ArrayBox in this owner.
- Do not change SafeMutexStore or SingleThreadExactStore semantics.
- Do not open provider activation, allocator replacement, hooks, globals, or
  winner claims.
```

## Required Next Inventory

The next row must be observation-only and produce a CFG-aware selected-method
plan before implementation.

```text
output_contract=cfg-aware-typed-field-residence-plan-v0
selected_method=HakoAllocPageModel.acquire_usize/1
eligible_resident_field_count=...
erased_field_get_count=...
erased_field_set_count=...
inserted_helper_load_count=...
inserted_helper_writeback_count=...
net_helper_call_delta=...
phi_value_required_count=...
phi_dirty_required_count=...
flush_before_call_count=...
flush_before_return_count=...
fallback_field_count=...
transform_open=0
summary=ok
```

Implementation is allowed only after this report shows:

```text
net_helper_call_delta_positive=1
unsupported_shape_fallbacks_documented=1
```
