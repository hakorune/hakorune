---
Status: Landed
Date: 2026-06-25
Card: MIRBUILDER-MODULE-METADATA-PUBLICATION-001
---

# MIRBUILDER-MODULE-METADATA-PUBLICATION-001

## Summary

`ModuleMetadataPublication` is now a source-derived PlanOnly capability for the
prepared-state minimal MirBuilder path. The slice owns only the declaration
inventory copied from `CompilationContext` into `MirModule` metadata before
semantic refresh. It does not claim record/packed layout refresh, typed object
plan refresh, direct state plan refresh, all-functions PHI materialization, full
finalize, generated Hako, backend routes, ABI changes, runtime fallback, or
source selfhost.

## Source Authority

- `src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module`
- Predecessor plan:
  `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-slot-registry-release-plan-v0.json`

## Publication Contract

```text
timing = AfterSlotRegistryReleaseBeforeSemanticRefresh
module.metadata.user_box_decls <- self.comp_ctx.user_defined_boxes.clone()
module.metadata.user_box_field_decls <- self.comp_ctx.user_box_field_decls mapped to UserBoxFieldDecl
module.metadata.record_decls <- self.comp_ctx.record_decls.clone().into_iter().collect()
module.metadata.enum_decls <- self.comp_ctx.enum_decls_for_module_metadata()
```

## Artifacts

- `tools/rust_lifecycle/mirbuilder_module_metadata_publication.py`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-module-metadata-publication-plan-v0.json`
- `tools/checks/rust_lifecycle_mirbuilder_module_metadata_publication_guard.sh`

## Derived Frontier Result

The minimal execution path analyzer now marks
`finalize_module.module_metadata_publication` as `Available`.

The next derived unsupported edge is:

```text
edge_id: finalize_module.record_packed_layout_refresh
callsite: MirBuilder::finalize_module -> refresh_module_record_and_packed_layout_plans
deny_reason: UnsupportedDirectShape
deny_detail: RecordAndPackedLayoutRefreshRequired
semantic_owner: MirBuilder::finalize_module record/packed layout refresh
next_slice_token: MIRBUILDER-RECORD-PACKED-LAYOUT-REFRESH-001
```

## Non-Claims

```text
semantic_refresh = 0
record_and_packed_layout_refresh = 0
typed_object_plan_refresh = 0
direct_state_plan_refresh = 0
all_functions_phi_materialization = 0
full_finalize_module = 0
generated_hako_artifact = 0
backend_route_changed = 0
abi_changed = 0
runtime_fallback = 0
mainline_selected = 0
```

## Acceptance

```text
python3 -m py_compile tools/rust_lifecycle/mirbuilder_module_metadata_publication.py tools/rust_lifecycle/mirbuilder_minimal_execution_path_selection.py
bash tools/checks/rust_lifecycle_mirbuilder_module_metadata_publication_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
