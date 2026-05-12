# 293x-207: C201 User Box Field-Index Fast Path

Status: Complete

## Purpose

Expose a backend-readable field-index fast path for ordinary `box`
declarations without changing ordinary box semantics.

C201 is the bridge before `record`: ordinary `box` remains identity-capable and
dynamic-field-compatible, but fields that already have a legal typed-object
plan can be consumed as:

```text
layout_id + field_index + storage
```

## Implementation

MIR JSON `user_box_decls[].field_decls[]` now carries:

- `field_index_fast_path: true`
- `layout_id`
- `field_index`
- `storage`

only when the field is present in the module's typed-object plan. Other fields
stay on the names/dynamic route with `field_index_fast_path: false`.

## Stop Line

C201 does not add:

- `record` syntax
- identity erasure for ordinary `box`
- packed `ArrayBox` storage
- allocator-specific metadata lowering
- native/provider/hook behavior

## Acceptance

- Unit coverage proves typed fields expose `layout_id + field_index + storage`.
- Weak or unsupported fields do not claim the fast path.
- Existing `typed_object_plans` remain the layout authority.
- Focused guard confirms no `.inc` shim/provider leakage.
