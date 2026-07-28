Status: Done
Date: 2026-06-18
Scope: select the fifth passive family for hakorune-mir-plans split
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - src/mir/function/object_metadata.rs
  - src/mir/record_layout_plan.rs
  - src/mir/array_record_storage_plan.rs

# BUILD-MIR-PLANS-FIFTH-FAMILY-SELECTION-001

## Purpose

Select the next safe split after `local_fastpath_fact` pure aggregation moved
into `hakorune-mir-plans`.

## Audit

ArrayRecord and record layout passive data share one common storage vocabulary:

```text
shared_vocab=TypedObjectFieldStorage
used_by=typed_object_plan,record_layout_plan,array_record_storage_plan,array_record_autouse_eligibility,route_decision,mir_json_emit,verification
current_owner=src/mir/function/object_metadata.rs
```

Moving ArrayRecord passive data as a bundle before this storage vocabulary
would keep the bundle anchored to a main-crate type. The safer fifth slice is
therefore the shared storage enum itself.

## Decision

Select `TypedObjectFieldStorage` as the fifth passive family.

```text
selected_family=typed_field_storage_vocabulary
move=TypedObjectFieldStorage
keep_main_crate=compatibility re-export through crate::mir::function
behavior_changed=0
```

This unblocks the later ArrayRecord passive bundle split without moving
`MirModule` refresh logic, storage inference, route decisions, or backend
lowering.

## Contract

```text
output_contract=build-mir-plans-fifth-family-selection-v0

selected_family=typed_field_storage_vocabulary
boxshape_only=1
boxcount_allowed=0
behavior_change_allowed=0
storage_inference_moved=0
backend_lowering_enabled=0
runtime_route_enabled=0

summary=ok
```

## Next

```text
next_task=BUILD-MIR-PLANS-TYPED-FIELD-STORAGE-SPLIT-001
```
