Status: Done
Date: 2026-06-18
Scope: first passive plan family moved into hakorune-mir-plans
Related:
  - docs/development/current/main/phases/phase-296x/296x-1079-BUILD-MIR-PLANS-CRATE-PREFLIGHT-001.md
  - crates/hakorune_mir_plans/src/object_storage_plan.rs
  - src/object_storage_plan.rs

# BUILD-MIR-PLANS-OBJECT-STORAGE-SPLIT-001

## Purpose

Create `hakorune-mir-plans` and move the passive ObjectStoragePlan vocabulary
family into it while preserving the main-crate `crate::object_storage_plan`
facade.

## Change

```text
new_crate=crates/hakorune_mir_plans
first_family=object_storage_plan
main_crate_compat_facade=src/object_storage_plan.rs
behavior_changed=0
```

Moved files:

```text
crates/hakorune_mir_plans/src/object_storage_plan.rs
crates/hakorune_mir_plans/src/object_storage_plan/alias.rs
crates/hakorune_mir_plans/src/object_storage_plan/decision.rs
crates/hakorune_mir_plans/src/object_storage_plan/fastpath.rs
crates/hakorune_mir_plans/src/object_storage_plan/ids.rs
crates/hakorune_mir_plans/src/object_storage_plan/inventory.rs
crates/hakorune_mir_plans/src/object_storage_plan/publication.rs
crates/hakorune_mir_plans/src/object_storage_plan/reason_domain.rs
crates/hakorune_mir_plans/src/object_storage_plan/report.rs
crates/hakorune_mir_plans/src/object_storage_plan/storage.rs
crates/hakorune_mir_plans/src/object_storage_plan/tests.rs
```

## Verification

```text
cargo_test_hakorune_mir_plans=green
object_storage_plan_test_count=13
main_crate_facade_preserved=1
```

## Contract

```text
output_contract=build-mir-plans-object-storage-split-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
new_backend_lowering_enabled=0
mirbuilder_object_management_enabled=0
product_runtime_changed=0

summary=ok
```

## Next

```text
next_task=BUILD-TIME-BASELINE-MEASURE-001
```
