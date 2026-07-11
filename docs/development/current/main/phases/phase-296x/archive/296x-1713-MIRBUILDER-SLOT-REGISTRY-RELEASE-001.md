---
Status: Landed
Date: 2026-06-25
Card: MIRBUILDER-SLOT-REGISTRY-RELEASE-001
---

# MIRBUILDER-SLOT-REGISTRY-RELEASE-001

## Summary

`SlotRegistryRelease` is now a source-derived PlanOnly capability for the
prepared-state minimal MirBuilder path. The slice owns only the
`finalize_module` edge that clears `comp_ctx.current_slot_registry`. It does
not claim SlotMetadata classification, module metadata publication, semantic
refresh, all-functions PHI materialization, full finalize, generated Hako,
backend routes, ABI changes, runtime fallback, or source selfhost.

## Source Authority

- `src/mir/builder/module_lifecycle.rs::MirBuilder::prepare_module`
- `src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module`
- Predecessor plan:
  `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-function-region-stack-pop-plan-v0.json`

## Release Contract

```text
lifecycle_owner = CompilationContext.current_slot_registry
init_operation = Some(FunctionSlotRegistry::new())
release_operation = current_slot_registry = None
release_timing = AfterFunctionRegionStackPopBeforeModuleMetadataPublication
released_value = FunctionSlotRegistry
```

## Artifacts

- `tools/rust_lifecycle/mirbuilder_slot_registry_release.py`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-slot-registry-release-plan-v0.json`
- `tools/checks/rust_lifecycle_mirbuilder_slot_registry_release_guard.sh`

## Derived Frontier Result

The minimal execution path analyzer now marks
`finalize_module.slot_registry_release` as `Available`.

The next derived unsupported edge is:

```text
edge_id: finalize_module.module_metadata_publication
callsite: MirBuilder::finalize_module -> publish module metadata
deny_reason: UnsupportedDirectShape
deny_detail: ModuleMetadataPublicationRequired
semantic_owner: MirBuilder::finalize_module module metadata publication
next_slice_token: MIRBUILDER-MODULE-METADATA-PUBLICATION-001
```

## Non-Claims

```text
slot_metadata_classification = 0
function_region_stack_pop = 0
module_metadata_publication = 0
metadata_publication = 0
semantic_refresh = 0
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
python3 -m py_compile tools/rust_lifecycle/mirbuilder_slot_registry_release.py tools/rust_lifecycle/mirbuilder_minimal_execution_path_selection.py
bash tools/checks/rust_lifecycle_mirbuilder_slot_registry_release_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
