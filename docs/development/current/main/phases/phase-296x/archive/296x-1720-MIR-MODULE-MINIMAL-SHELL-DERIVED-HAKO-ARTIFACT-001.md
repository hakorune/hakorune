---
Status: Landed
Date: 2026-06-26
Card: MIR-MODULE-MINIMAL-SHELL-DERIVED-HAKO-ARTIFACT-001
---

# MIR-MODULE-MINIMAL-SHELL-DERIVED-HAKO-ARTIFACT-001

## Summary

`MirModule::new` now has a bounded DerivedShadow Hako artifact for the minimal
module shell. The artifact materializes only the constructor-owned shell:

```text
name
fresh empty function table
fresh empty global table
default metadata shell with source_file = None
```

It does not claim function insertion, global publication, metadata publication,
finalize behavior, full `MirModule` conversion, mainline selection, or source
selfhost.

## Authority

Semantic source:

```text
MirModuleMinimalShellTransportPlanV1
  -> MirModuleMinimalShell DerivedShadow artifact
```

Generated artifact:

```text
lang/generated/rust_derived/hakorune_mir_builder/mir_module_minimal_shell.hako
lang/generated/rust_derived/hakorune_mir_builder/mir_module_minimal_shell.artifact.json
```

The artifact is an executable materialization of the existing source-derived
transport plan. It is not a new semantic authority.

## Acceptance

```text
deterministic regeneration = green
generated Hako parse/MIR = green
generated Hako EXE/AOT = green
module name preserved = green
functions initially empty = green
globals initially empty = green
source_file absent = green
fresh function/global table identity = green
```

The semantic closure report now consumes this artifact and advances the first
executable materialization gap:

```text
first_executable_materialization_gap = prepare_module.function_new
next_slice = MIR-FUNCTION-CONSTRUCTOR-DERIVED-HAKO-ARTIFACT-001
```

## Artifacts

- `tools/rust_lifecycle/mir_module_minimal_shell_artifacts.py`
- `tools/checks/rust_lifecycle_mir_module_minimal_shell_derived_artifact_guard.sh`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mir-module-minimal-shell-derived-hako-oracle-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mir-module-minimal-shell-derived-hako-recipe-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mir-module-minimal-shell-derived-hako-verifier-result-v0.json`
- `lang/generated/rust_derived/hakorune_mir_builder/mir_module_minimal_shell.hako`
- `lang/generated/rust_derived/hakorune_mir_builder/mir_module_minimal_shell.artifact.json`

## Non-Claims

```text
function_insertion = 0
global_publication = 0
metadata_publication = 0
source_file_assignment = 0
finalize_module = 0
full_mir_module_conversion = 0
full_mirbuilder_new = 0
mainline_selected = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
```

## Gates

```text
python3 -m py_compile tools/rust_lifecycle/mir_module_minimal_shell_artifacts.py
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mir-module-minimal-shell --check
bash tools/checks/rust_lifecycle_mir_module_minimal_shell_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_semantic_closure_report_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
