Status: Done
Date: 2026-06-18
Scope: move object-state passive plan rows into hakorune-mir-plans
Related:
  - docs/development/current/main/phases/phase-296x/296x-1092-BUILD-MIR-PLANS-NEXT-PASSIVE-FAMILY-SELECTION-001.md
  - crates/hakorune_mir_plans/src/object_state_plan.rs
  - src/mir/function/object_metadata.rs

# BUILD-MIR-PLANS-OBJECT-STATE-PASSIVE-BUNDLE-SPLIT-001

## Purpose

Move typed-object, direct-state, and record-state passive plan rows into
`hakorune-mir-plans`, while preserving the existing
`crate::mir::function::*` compatibility import surface.

## Change

```text
new_owner=crates/hakorune_mir_plans/src/object_state_plan.rs
moved_rows=TypedObjectPlan,TypedObjectFieldPlan,DirectStatePlan,DirectStateFieldPlan,RecordStateResidencePlan,RecordStateResidenceFieldPlan,RecordStateResidenceRejectedFieldPlan,RecordStateFieldAccessPlan
main_crate_compat_reexport=crate::mir::function::*
behavior_changed=0
```

The main crate keeps declaration inventory (`UserBoxFieldDecl`, `RecordDecl`)
and all active producer/classifier logic.

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
output_contract=build-mir-plans-object-state-passive-bundle-split-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
main_crate_import_path_preserved=1
declaration_inventory_moved=0
producer_logic_moved=0
new_backend_lowering_enabled=0
new_runtime_route_enabled=0
new_large_file_created=0

summary=ok
```

## Next

```text
next_task=BUILD-MIR-PLANS-STAGE1-CLOSEOUT-CANDIDATE-001
```
