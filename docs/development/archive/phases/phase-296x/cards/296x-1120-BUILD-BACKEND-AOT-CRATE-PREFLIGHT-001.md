Status: Done
Date: 2026-06-18
Scope: preflight backend AOT crate split boundary
Related:
  - docs/development/current/main/phases/phase-296x/296x-1119-BUILD-BACKEND-NEXT-BOUNDARY-SELECTION-001.md
  - src/backend/aot
  - docs/development/current/main/design/build-crate-split-plan-ssot.md

# BUILD-BACKEND-AOT-CRATE-PREFLIGHT-001

## Inventory

```text
backend_aot_total_lines=950
backend_aot_dependency_refs=4

full_split_blocked_by=MirModule
full_split_blocked_by=WasmBackend

compiler_rs_depends_on=crate::mir::MirModule
compiler_rs_depends_on=crate::backend::wasm::{WasmBackend,WasmError}
mod_rs_depends_on=crate::mir::MirModule
```

## Decision

```text
full_backend_aot_crate_split_selected=0
selected_first_slice=aot_passive_config_executable_error
selected_next_task=BUILD-BACKEND-AOT-PASSIVE-CRATE-SCAFFOLD-001
reason=move dependency-light AOT configuration/error/executable support before compiler pipeline
```

The full AOT backend cannot move before the MIR input and WASM backend boundary
are available outside the main crate. The low-risk first slice is the passive
support vocabulary:

```text
move_candidate=AotError
move_candidate=AotConfig
move_candidate=ExecutableBuilder
stay_in_main_crate=AotCompiler
stay_in_main_crate=AotBackend
```

## Contract

```text
output_contract=build-backend-aot-crate-preflight-v0

selection_only=1
behavior_changed=0
code_moved=0
full_backend_aot_crate_split_selected=0
selected_next_task=BUILD-BACKEND-AOT-PASSIVE-CRATE-SCAFFOLD-001

summary=ok
```
