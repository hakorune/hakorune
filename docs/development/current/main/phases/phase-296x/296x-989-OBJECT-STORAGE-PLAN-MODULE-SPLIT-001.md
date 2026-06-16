# 296x-989 OBJECT-STORAGE-PLAN-MODULE-SPLIT-001

Status: Landed
Date: 2026-06-17
Scope: BoxShape refactor / behavior unchanged

## Contract

```text
output_contract=hako-object-storage-plan-module-split-v0
row_kind=boxshape_refactor
behavior_changed=0
public_api_reexport_preserved=1
facade_line_count_max=80
object_storage_plan_execution_enabled=0
backend_lowering_changed=0
mirbuilder_object_management_enabled=0
next_task=OBJECT-STORAGE-PLAN-VOCAB-AUDIT-001
summary=ok
```

## Purpose

Keep the local-first / fast-path vocabulary thin at the file level.

`src/object_storage_plan.rs` had accumulated representation, publication,
fast-path, alias, inventory, report, and tests in one file. The vocabulary was
not empty ceremony, but the file mixed too many concerns. This row splits the
file into a facade plus focused submodules while preserving public re-exports.

## Module Layout

```text
src/object_storage_plan.rs
  facade / re-export only

src/object_storage_plan/ids.rs
  layout / field / value / route / site ids

src/object_storage_plan/storage.rs
  ObjectStoragePlan / ObjectPlan / scalar and flattened field plans

src/object_storage_plan/publication.rs
  ObjectPublicationSite / PublicationState / publication reasons

src/object_storage_plan/fastpath.rs
  LocalFastPathFact / kind / fallback reason

src/object_storage_plan/alias.rs
  local alias observation vocabulary

src/object_storage_plan/inventory.rs
  publication inventory rows and known-receiver shadow rows

src/object_storage_plan/report.rs
  report vocabulary

src/object_storage_plan/tests.rs
  existing unit tests
```

## Stop Line

This row does not:

```text
change LocalFastPathFact semantics
merge or delete vocabulary types
enable object storage execution
change backend lowering
move object management into MIRBuilder
change MapReprPlan
```

## Verification

```bash
cargo test --lib object_storage_plan -- --nocapture
cargo check --lib
bash tools/checks/k2_wide_phase296x_object_storage_plan_module_split_guard.sh
```

## Next

```text
OBJECT-STORAGE-PLAN-VOCAB-AUDIT-001
```

If more cleanup is needed, audit synonym candidates first. Do not merge
vocabulary types in this split row.
