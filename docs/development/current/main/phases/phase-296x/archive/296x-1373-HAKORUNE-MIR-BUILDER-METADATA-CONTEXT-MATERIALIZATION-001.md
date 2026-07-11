# 296x-1373 HAKORUNE-MIR-BUILDER-METADATA-CONTEXT-MATERIALIZATION-001

Status: closed
Date: 2026-06-20

## Purpose

Materialize the `hakorune_mir_builder::metadata_context` single-module
RustSubset bundle.

This is a checked-in app-front materialization row. It must not add new Rust
semantics; it only freezes the already-green module artifact, expected Hako
skeleton, and focused wrapper.

## Current Evidence

Selected by:

```text
296x-1372 RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-016
```

Current target status:

```text
hakorune_mir_builder::metadata_context:
  generated_skeleton_mir_emit=green
```

Recently closed blockers that made this possible:

```text
generic_impl_target_emitted_names_parser_safe=1
reference_type_spelling_parser_safe=1
self_value_skeleton_safe=1
option_constructor_skeleton_safe=1
```

## Scope

Allowed:

```text
checked-in crate-manifest.json for crate::metadata_context
checked-in modules/0000.json for crate::metadata_context
checked-in expected .hako skeleton
focused convert_hakorune_mir_builder_metadata_context_crate_file.hako wrapper
smoke.sh fixture registration
```

Required behavior:

```text
metadata_context_manifest_checked_in=1
metadata_context_module_artifact_checked_in=1
metadata_context_expected_hako_checked_in=1
metadata_context_wrapper_added=1
generated_skeleton_mir_emit=green
wrapper_exe_parity=green
generated_program_execution_claim=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
new_hako_syntax_added=0
```

## Non-Goals

```text
do not implement closure semantics
do not implement Option semantics
do not rewrite metadata_context source
do not add generated program execution claim
do not aggregate multiple modules in this row
```

## Acceptance

Expected closeout evidence:

```text
manifest_checked_in=1
module_artifact_checked_in=1
generated_skeleton_expected_checked_in=1
focused_wrapper_added=1
generated_skeleton_mir_emit=green
wrapper_exe_parity=green
full_rust_subset_smoke=green
generated_program_execution_claim=0
summary=ok
```

## Closeout

```text
manifest_checked_in=1
module_artifact_checked_in=1
generated_skeleton_expected_checked_in=1
focused_wrapper_added=1
generated_skeleton_mir_emit=green
wrapper_exe_parity=green
full_rust_subset_smoke=green
generated_program_execution_claim=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
new_hako_syntax_added=0
summary=ok
```

The checked-in bundle covers only `crate::metadata_context`. It does not
aggregate modules or claim generated program execution.

General checks:

```bash
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

Run the rust-subset smoke when the implementation is ready:

```bash
RUST_SUBSET_RUN_ADAPTER=1 bash apps/rust-subset-to-hako/smoke.sh
```

## Stop Line

```text
new_hako_syntax_added=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
generated_program_execution_claim=0
```
