# 296x-990 OBJECT-STORAGE-PLAN-GUARD-PATH-COMPAT-001

Status: Landed
Date: 2026-06-17
Scope: guard/tool compatibility / behavior unchanged

## Contract

```text
output_contract=hako-object-storage-plan-guard-path-compat-v0
source_evidence=296x-989
row_kind=guard_compat
facade_and_module_tree_checked=1
legacy_guard_source_path_compat_fixed=1
publication_inventory_tool_reads_split_modules=1
behavior_changed=0
public_api_reexport_preserved=1
vocabulary_merge_count=0
backend_lowering_changed=0
mirbuilder_object_management_enabled=0
next_task=OBJECT-STORAGE-PLAN-VOCAB-AUDIT-001
summary=ok
```

## Purpose

Repair legacy guard/tool path assumptions after the ObjectStoragePlan module
split.

296x-989 correctly made `src/object_storage_plan.rs` a thin facade, but older
guards still grepped that facade for type bodies. This row makes those checks
look at both the facade and `src/object_storage_plan/` modules.

## Changes

```text
tools/checks/k2_wide_phase296x_object_storage_plan_ssot_guard.sh
tools/checks/k2_wide_phase296x_local_publication_classifier_guard.sh
tools/checks/k2_wide_phase296x_local_alias_class_mvp_guard.sh
tools/checks/k2_wide_phase296x_local_publication_inventory_v2_guard.sh
tools/checks/k2_wide_phase296x_local_known_receiver_direct_call_shadow_guard.sh
tools/checks/k2_wide_phase296x_object_plan_local_first_guard.sh
tools/checks/k2_wide_phase296x_objectplan_passive_unify_guard.sh
tools/checks/k2_wide_phase296x_routeplan_objectplan_handoff_guard.sh
tools/checks/k2_wide_phase296x_backend_plan_consumer_guard.sh
tools/checks/k2_wide_phase296x_publication_site_generic_inventory_guard.sh
tools/checks/k2_wide_phase296x_exact_object_flattened_nested_field_layout_guard.sh
tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_candidate_inventory_guard.sh
tools/checks/k2_wide_phase296x_compiler_object_shape_closeout_guard.sh
tools/checks/k2_wide_phase296x_compiler_object_shape_closeout_followup_guard.sh
tools/allocator/hako_publication_site_generic_inventory.py
```

## Stop Line

This row does not:

```text
merge vocabulary types
remove public aliases
change ObjectStoragePlan semantics
enable object storage execution
change backend lowering
move object management into MIRBuilder
```

## Verification

```bash
bash tools/checks/k2_wide_phase296x_object_storage_plan_guard_path_compat_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
OBJECT-STORAGE-PLAN-VOCAB-AUDIT-001
```

With guard compatibility restored, the next row can audit synonym candidates
without breaking historical checks.
