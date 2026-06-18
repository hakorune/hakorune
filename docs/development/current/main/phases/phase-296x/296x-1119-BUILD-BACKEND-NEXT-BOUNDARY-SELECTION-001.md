Status: Done
Date: 2026-06-18
Scope: select next backend-adjacent build crate split boundary
Related:
  - docs/development/current/main/phases/phase-296x/296x-1118-BUILD-MIR-JSON-EMIT-POST-SPLIT-MEASURE-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md

# BUILD-BACKEND-NEXT-BOUNDARY-SELECTION-001

## Inventory

```text
src_runner_total_lines=49455
src_backend_total_lines=19948
src_frontend_parser_ast_total_lines=16308

stage1_bridge_lines=3370
stage1_bridge_dependency_refs=53
json_v0_bridge_lines=6625
json_v0_bridge_dependency_refs=150
mir_json_emit_lines=12623
mir_json_emit_dependency_refs=373

backend_aot_lines=950
backend_aot_dependency_refs=4
backend_wasm_lines=5088
backend_wasm_dependency_refs=47
backend_mir_interpreter_lines=12944
```

## Decision

```text
selected_next_boundary=backend_aot
selected_next_task=BUILD-BACKEND-AOT-CRATE-PREFLIGHT-001
reason=small_low_dependency_backend_boundary_on_product_exe_route

rejected_boundary=backend_mir_interpreter
rejected_reason=vm_product_route_retired

rejected_boundary=backend_wasm
rejected_reason=not_current_product_route

rejected_boundary=runner_json_v0_bridge
rejected_reason=larger_and_more_cross_crate_dependencies_than_backend_aot
```

The AOT backend boundary is small enough to audit before moving code. The
preflight must decide whether to split the full module or only passive config /
command vocabulary first.

## Contract

```text
output_contract=build-backend-next-boundary-selection-v0

selection_only=1
behavior_changed=0
code_moved=0
selected_next_task=BUILD-BACKEND-AOT-CRATE-PREFLIGHT-001

summary=ok
```
