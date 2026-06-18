Status: Done
Date: 2026-06-18
Scope: close passive AOT support split and select default-build-impact boundary
Related:
  - docs/development/current/main/phases/phase-296x/296x-1122-BUILD-BACKEND-AOT-PASSIVE-FACADE-WIRING-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/design/vm-active-lane-retirement-ssot.md

# BUILD-BACKEND-AOT-PASSIVE-CLOSEOUT-001

## Result

```text
passive_aot_support_split_closed=1
new_crate=hakorune-backend-aot
dependency_feature_gate=wasm-backend
main_crate_removed_files=src/backend/aot/config.rs,src/backend/aot/executable.rs
compiler_pipeline_owner=main_crate
behavior_changed=0
```

This split is structurally valid but does not target default build time because
`backend/aot` is compiled only under `wasm-backend`.

## Decision

```text
post_split_default_cold_build_measure_selected=0
reason=aot_boundary_is_optional_feature_not_default_build_owner

selected_next_task=BUILD-VM-MIR-INTERPRETER-COMPILE-AUDIT-001
reason=mir_interpreter_is_large_default_compiled_backend_surface_and_vm_product_route_is_retired
```

The next row must be audit-only. It should not delete or gate VM code until it
proves which default callers still require `MirInterpreter`, `VMValue`, or
compat aliases.

## Contract

```text
output_contract=build-backend-aot-passive-closeout-v0

selection_only=1
behavior_changed=0
code_moved=0
post_split_default_cold_build_measure_selected=0
selected_next_task=BUILD-VM-MIR-INTERPRETER-COMPILE-AUDIT-001

summary=ok
```
