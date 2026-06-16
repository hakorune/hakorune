# 296x-850 MIMALLOC-LEAF-ARRAY-STRING-LEN-LOOP-SESSION-DESIGN-001

Status: Landed
Date: 2026-06-16

## Purpose

Design the loop-local text session boundary selected by 296x-849.

The design outcome is conservative: do not implement a loop session by passing
raw `ArrayTextSession` / `ArrayBox` pointers through ad-hoc FFI. First introduce
a proof surface that can say the repeated `array_string_len` calls are within a
read-only loop window over the same array handle.

## Decision

```text
accepted_owner=array_text_loop_session_plan_surface
implementation_allowed=0
```

The current hot loop is:

```text
for idx in loop:
  call nyash.array.string_len_hi / hako.array_text.slot_len(handle, idx)
```

The tempting implementation would be:

```text
resolve handle once
reuse ArrayTextSession / ArrayBox pointer inside loop
```

That is not allowed without a plan because it would move object lifetime,
publication, and mutation assumptions into the backend/runtime boundary.

## Required Future Proof Surface

Future implementation needs an explicit plan, not helper-name inference:

```text
ArrayTextLoopSessionPlan:
  same_array_handle=1
  loop_region_known=1
  array_text_read_only_in_region=1
  no_array_store_or_mutation_in_region=1
  no_drop_or_publication_boundary_in_region=1
  index_domain_proven_or_guarded=1
  route_aliases_share_body=1
  backend_session_lowering_allowed=1
```

This plan may be consumed by backend lowering later. Until then, the current
runtime helper path remains the product-correct fallback.

## Result

```text
output_contract=hako-mimalloc-leaf-array-string-len-loop-session-design-v0
source_evidence=296x-849,worker-inventory-2026-06-16
row_kind=design
target_front=kilo_leaf_array_string_len

selected_owner=array_text_loop_session_plan_surface
selected_owner_confidence=medium
implementation_allowed=0

raw_array_text_session_ffi_enabled=0
raw_arraybox_pointer_ffi_enabled=0
helper_name_inference_enabled=0
backend_loop_session_lowering_enabled=0
mirbuilder_object_management_enabled=0
product_default_changed=0

required_plan=ArrayTextLoopSessionPlan
same_array_handle_required=1
loop_region_required=1
read_only_region_required=1
no_mutation_region_required=1
no_drop_or_publication_boundary_required=1
index_domain_guard_required=1

selected_next=MIMALLOC-ARRAY-TEXT-LOOP-SESSION-PLAN-SURFACE-001
summary=ok
```

## Stop Line

```text
do not pass raw ArrayTextSession or ArrayBox pointers through FFI
do not implement backend loop-session lowering without ArrayTextLoopSessionPlan
do not infer from helper aliases
do not change ArrayBox storage or product runtime defaults
do not touch MIRBuilder object management
do not broaden to indexOf/store paths
```
