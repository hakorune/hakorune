Status: Done
Date: 2026-06-18
Scope: wire main-crate AOT passive support facade to hakorune-backend-aot
Related:
  - docs/development/current/main/phases/phase-296x/296x-1121-BUILD-BACKEND-AOT-PASSIVE-CRATE-SCAFFOLD-001.md
  - crates/hakorune_backend_aot
  - src/backend/aot

# BUILD-BACKEND-AOT-PASSIVE-FACADE-WIRING-001

## Purpose

Move passive AOT support implementation to `hakorune-backend-aot` while keeping
the existing `backend::aot::*` surface stable.

## Change

```text
main_crate_dependency_added=1
dependency_feature_gate=wasm-backend
serialization_owner=not_applicable
passive_aot_support_owner=hakorune_backend_aot
main_crate_facade=src/backend/aot/mod.rs
removed_main_crate_files=src/backend/aot/config.rs,src/backend/aot/executable.rs
behavior_changed=0
```

`AotCompiler` and `AotBackend` remain in the main crate. They still depend on
`MirModule` and `WasmBackend`.

## Verification

```text
cargo_check_default=green
cargo_check_wasm_backend=green
cargo_test_hakorune_backend_aot=green
```

## Contract

```text
output_contract=build-backend-aot-passive-facade-wiring-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
full_backend_aot_crate_split_selected=0
passive_aot_support_owner=hakorune_backend_aot
compiler_pipeline_owner=main_crate

summary=ok
```

## Next

```text
next_task=BUILD-BACKEND-AOT-PASSIVE-CLOSEOUT-001
purpose=close passive AOT support split and decide whether to measure or select another boundary
```
