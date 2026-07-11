---
Status: Landed
Date: 2026-06-26
Card: MIRBUILDER-PREPARED-STATE-INSTALL-DERIVED-HAKO-ARTIFACT-001
---

# MIRBUILDER-PREPARED-STATE-INSTALL-DERIVED-HAKO-ARTIFACT-001

## Summary

`MirBuilder::prepare_module` current state install now has a bounded
DerivedShadow Hako artifact. The artifact materializes only the prepared-state
installation of already-bounded module/function shells:

```text
current_module = present MirModuleMinimalShell
scope_ctx.current_function = present MirFunctionConstructorShell
current_block = present BasicBlockIdAsI64
```

The generated representation uses explicit presence tags plus payload fields.
It does not require boxed `Option<CustomObject>` transport in the backend.

## Authority

Semantic source:

```text
MirBuilderPreparedStateInstallPlanV1
  -> PreparedStateInstall DerivedShadow artifact
```

Generated artifact:

```text
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_prepared_state_install.hako
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_prepared_state_install.artifact.json
```

The artifact is an executable materialization of the existing source-derived
install plan. It is not a new semantic authority and does not select mainline
execution.

## Acceptance

```text
deterministic regeneration = green
generated Hako parse/MIR = green
generated Hako EXE/AOT = green
current_module installed = green
current_function installed = green
current_block installed = green
fresh state identity = green
```

The semantic closure report now consumes this artifact and advances the first
executable materialization gap:

```text
first_executable_materialization_gap = lower_root.literal_integer
next_slice = MIRBUILDER-LITERAL-INTEGER-DERIVED-HAKO-ARTIFACT-001
```

## Artifacts

- `tools/rust_lifecycle/mirbuilder_prepared_state_install_artifacts.py`
- `tools/checks/rust_lifecycle_mirbuilder_prepared_state_install_derived_artifact_guard.sh`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-prepared-state-install-plan-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-prepared-state-install-derived-hako-oracle-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-prepared-state-install-derived-hako-recipe-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-prepared-state-install-derived-hako-verifier-result-v0.json`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_prepared_state_install.hako`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_prepared_state_install.artifact.json`

## Non-Claims

```text
current_module_take = 0
current_function_take = 0
lower_root = 0
literal_integer_lowering = 0
return_emission = 0
finalize_module = 0
full_mirbuilder_object_transport = 0
mainline_selected = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
```

## Gates

```text
python3 -m py_compile tools/rust_lifecycle/mirbuilder_prepared_state_install_artifacts.py
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-prepared-state-install --check
bash tools/checks/rust_lifecycle_mirbuilder_prepared_state_install_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_semantic_closure_report_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
