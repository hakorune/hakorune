# 296x-994 LOCALFIRSTOBJECTPLAN-ALIAS-RETIRE-IMPLEMENTATION-001

Status: Landed
Date: 2026-06-17
Scope: compatibility alias removal / no behavior change

## Contract

```text
output_contract=hako-localfirstobjectplan-alias-retire-implementation-v0
source_evidence=296x-991,296x-992,296x-993
row_kind=implementation
localfirstobjectplan_alias_removed=1
objectplan_canonical_name_required=1
local_first_object_plan_alias_retired=1
local_first_object_plan_compat_alias_enabled=0
historical_guards_tolerate_alias_retire=1
public_api_reexport_preserved=1
vocabulary_merge_count=1
backend_lowering_changed=0
mir_json_metadata_changed=0
mirbuilder_object_management_enabled=0
summary=ok
```

## Purpose

Remove the `LocalFirstObjectPlan` public compatibility alias after guard
compatibility landed.

`ObjectPlan` is now the only canonical source type for the passive
representation + publication-site planning artifact.

## Changes

```text
src/object_storage_plan/storage.rs:
  removed pub type LocalFirstObjectPlan = ObjectPlan

src/object_storage_plan/report.rs:
  replaced local_first_object_plan_compat_alias_enabled=1 with
  local_first_object_plan_alias_retired=1

src/object_storage_plan/tests.rs:
  uses ObjectPlan::new directly
```

## Stop Line

This row does not:

```text
change ObjectPlan fields
change public re-export modules
change MIR JSON metadata
change backend lowering
move object management into MIRBuilder
merge reason enums or ids
```

## Verification

```bash
bash tools/checks/k2_wide_phase296x_localfirstobjectplan_alias_retire_implementation_guard.sh
bash tools/checks/k2_wide_phase296x_object_plan_local_first_guard.sh
bash tools/checks/k2_wide_phase296x_objectplan_passive_unify_guard.sh
bash tools/checks/k2_wide_phase296x_routeplan_objectplan_handoff_guard.sh
bash tools/checks/k2_wide_phase296x_localfirstobjectplan_alias_retire_preflight_guard.sh
bash tools/checks/k2_wide_phase296x_localfirstobjectplan_alias_retire_guard_compat_guard.sh
cargo test --lib object_storage_plan --quiet
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
