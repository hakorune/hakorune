Status: Done
Date: 2026-06-18
Scope: move TypedObjectFieldStorage into hakorune-mir-plans
Related:
  - docs/development/current/main/phases/phase-296x/296x-1088-BUILD-MIR-PLANS-FIFTH-FAMILY-SELECTION-001.md
  - crates/hakorune_mir_plans/src/typed_field_storage.rs
  - src/mir/function/object_metadata.rs

# BUILD-MIR-PLANS-TYPED-FIELD-STORAGE-SPLIT-001

## Purpose

Move the shared typed-field storage vocabulary into `hakorune-mir-plans`,
while preserving the existing main-crate import path for current consumers.

## Change

```text
new_owner=crates/hakorune_mir_plans/src/typed_field_storage.rs
moved_type=TypedObjectFieldStorage
main_crate_compat_reexport=crate::mir::function::TypedObjectFieldStorage
behavior_changed=0
```

The new owner is passive. It names storage classes and helper predicates only.
It does not infer field storage, mutate MIR, enable backend lowering, or change
runtime behavior.

## Verification

```text
cargo_test_hakorune_mir_plans=green
cargo_check=green
cargo_build_release_bin_hakorune=green
current_state_pointer_guard=green
large_file_count=0
```

## Contract

```text
output_contract=build-mir-plans-typed-field-storage-split-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
main_crate_import_path_preserved=1
storage_inference_moved=0
new_backend_lowering_enabled=0
new_runtime_route_enabled=0
new_large_file_created=0

summary=ok
```

## Next

```text
next_task=BUILD-MIR-PLANS-ARRAY-RECORD-PASSIVE-BUNDLE-SELECTION-001
```
