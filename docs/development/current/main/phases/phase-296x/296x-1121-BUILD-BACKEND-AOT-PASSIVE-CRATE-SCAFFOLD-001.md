Status: Done
Date: 2026-06-18
Scope: scaffold passive AOT support crate
Related:
  - docs/development/current/main/phases/phase-296x/296x-1120-BUILD-BACKEND-AOT-CRATE-PREFLIGHT-001.md
  - crates/hakorune_backend_aot
  - src/backend/aot

# BUILD-BACKEND-AOT-PASSIVE-CRATE-SCAFFOLD-001

## Purpose

Create the future passive AOT support crate without changing the main crate AOT
backend route.

## Change

```text
new_crate=hakorune-backend-aot
new_crate_scope=aot_error_config_executable_builder
new_crate_reads_mir_directly=0
new_crate_depends_on_wasm_backend=0
main_crate_dependency_added=0
behavior_changed=0
```

The scaffold mirrors `AotError`, `AotConfig`, and `ExecutableBuilder` as public
crate API. `AotCompiler` and `AotBackend` stay in the main crate because they
still depend on `MirModule` and `WasmBackend`.

## Verification

```text
cargo_test_hakorune_backend_aot=green
current_state_pointer_guard=green
```

## Contract

```text
output_contract=build-backend-aot-passive-crate-scaffold-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
main_crate_dependency_added=0
new_crate_reads_mir_directly=0
new_crate_depends_on_wasm_backend=0
full_backend_aot_crate_split_selected=0

summary=ok
```

## Next

```text
next_task=BUILD-BACKEND-AOT-PASSIVE-FACADE-WIRING-001
purpose=wire main-crate AOT config/error/executable facade to the new crate API
```
