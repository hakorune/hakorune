---
Status: Landed
Date: 2026-06-25
Card: MIRBUILDER-FUNCTION-REGION-STACK-POP-001
---

# MIRBUILDER-FUNCTION-REGION-STACK-POP-001

## Summary

`FunctionRegionStackPop` is now a source-derived PlanOnly capability for the
prepared-state minimal MirBuilder path. The slice owns only the
`finalize_module` edge that calls `region::observer::pop_function_region(self)`.
It does not claim SlotRegistry release, metadata publication, semantic refresh,
all-functions PHI materialization, full finalize, generated Hako, backend
routes, ABI changes, runtime fallback, or source selfhost.

## Source Authority

- `src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module`
- `src/mir/region/observer.rs::pop_function_region`
- Predecessor plan:
  `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-condition-fn-injection-plan-v0.json`

## Pop Contract

```text
callsite = region::observer::pop_function_region(self)
guard = NYASH_REGION_TRACE == 1
operation = metadata_ctx.pop_region
result_ignored = true
tracing_disabled_effect = NoOp
mutates_when_guard_enabled = builder.metadata_ctx.current_region_stack
```

The observer push counterpart is detected as evidence only. This slice does not
claim `observe_function_region`.

## Artifacts

- `tools/rust_lifecycle/mirbuilder_function_region_stack_pop.py`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-function-region-stack-pop-plan-v0.json`
- `tools/checks/rust_lifecycle_mirbuilder_function_region_stack_pop_guard.sh`

## Derived Frontier Result

The minimal execution path analyzer now marks
`finalize_module.region_stack_pop` as `Available`.

The next derived unsupported edge is:

```text
edge_id: finalize_module.slot_registry_release
callsite: MirBuilder::finalize_module -> current_slot_registry = None
deny_reason: UnsupportedDirectShape
deny_detail: SlotRegistryReleaseRequired
semantic_owner: MirBuilder::finalize_module slot registry cleanup
next_slice_token: MIRBUILDER-SLOT-REGISTRY-RELEASE-001
```

## Non-Claims

```text
observe_function_region_claim = 0
slot_registry_release = 0
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
python3 -m py_compile tools/rust_lifecycle/mirbuilder_function_region_stack_pop.py tools/rust_lifecycle/mirbuilder_minimal_execution_path_selection.py
bash tools/checks/rust_lifecycle_mirbuilder_function_region_stack_pop_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
