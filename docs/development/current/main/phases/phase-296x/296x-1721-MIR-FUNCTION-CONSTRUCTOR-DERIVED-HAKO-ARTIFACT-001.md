---
Status: Landed
Date: 2026-06-26
Card: MIR-FUNCTION-CONSTRUCTOR-DERIVED-HAKO-ARTIFACT-001
---

# MIR-FUNCTION-CONSTRUCTOR-DERIVED-HAKO-ARTIFACT-001

## Summary

`MirFunction::new` now has a bounded DerivedShadow Hako artifact for the
constructor shell only. The artifact materializes:

```text
prepared function signature surface
entry-block-only block table
nested BasicBlock constructor defaults
parameter ValueId prepopulation
next_value_id = max(param_count, 1)
default function metadata shell
```

It does not claim prepared-state install, function body lowering, instruction
emission, parameter setup compatibility fallback, `reserve_parameter_value_ids`,
function finalization, full `MirFunction` conversion, mainline selection, or
source selfhost.

## Authority

Semantic source:

```text
MirFunctionConstructorCompositionPlanV1
  -> MirFunctionConstructorShell DerivedShadow artifact
```

Generated artifact:

```text
lang/generated/rust_derived/hakorune_mir_builder/mir_function_constructor_shell.hako
lang/generated/rust_derived/hakorune_mir_builder/mir_function_constructor_shell.artifact.json
```

The artifact is an executable materialization of the existing source-derived
constructor composition plan. It is not a new semantic authority.

## Acceptance

```text
deterministic regeneration = green
generated Hako parse/MIR = green
generated Hako EXE/AOT = green
constructor shell only = green
nested BasicBlock defaults = green
entry-block-only table = green
params prepopulated = green
next_value_id seed = max(param_count, 1) = green
fresh params / entry-block instruction arrays = green
```

The semantic closure report now consumes this artifact and advances the first
executable materialization gap:

```text
first_executable_materialization_gap = prepare_module.state_install
next_slice = MIRBUILDER-PREPARED-STATE-INSTALL-DERIVED-HAKO-ARTIFACT-001
```

## Artifacts

- `tools/rust_lifecycle/mir_function_constructor_artifacts.py`
- `tools/checks/rust_lifecycle_mir_function_constructor_derived_artifact_guard.sh`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mir-function-constructor-derived-hako-oracle-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mir-function-constructor-derived-hako-recipe-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mir-function-constructor-derived-hako-verifier-result-v0.json`
- `lang/generated/rust_derived/hakorune_mir_builder/mir_function_constructor_shell.hako`
- `lang/generated/rust_derived/hakorune_mir_builder/mir_function_constructor_shell.artifact.json`

## Non-Claims

```text
prepared_state_install = 0
separate_block_only_claim = 0
function_body_lowering = 0
instruction_emission = 0
parameter_setup_compatibility_fallback = 0
reserve_parameter_value_ids_call = 0
function_finalization = 0
full_mir_function_conversion = 0
mainline_selected = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
```

## Gates

```text
python3 -m py_compile tools/rust_lifecycle/mir_function_constructor_artifacts.py
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mir-function-constructor-shell --check
bash tools/checks/rust_lifecycle_mir_function_constructor_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_semantic_closure_report_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
