# 296x-1375 HAKORUNE-MIR-BUILDER-CRATE-ROOT-MATERIALIZATION-001

Status: closed
Date: 2026-06-20

## Purpose

Materialize the remaining `hakorune_mir_builder` crate-root RustSubset module:

```text
module=crate
source_path=src/lib.rs
```

This is a crate-root transport slice. It exists to complete checked-in
single-module coverage for `hakorune_mir_builder` before any broader crate
aggregation row.

## Selected By

```text
296x-1374-RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-017
```

## Current Evidence

Temporary probe from 296x-1374:

```text
module=crate
source_path=src/lib.rs
artifact_path=modules/0000.json
items=6
generated_skeleton_mir_emit=green
```

The generated skeleton is expected to contain only explicit `Use` Unsupported
handoff comments. That is acceptable for this row because `use` resolution is
not part of the RustSubset converter core.

## Scope

Check in the focused crate-root bundle and guard it through the existing
rust-subset app-front route:

```text
apps/rust-subset-to-hako/examples/hakorune_mir_builder_crate_root_expected/
apps/rust-subset-to-hako/examples/hakorune_mir_builder_crate_root_expected.hako
apps/rust-subset-to-hako/convert_hakorune_mir_builder_crate_root_crate_file.hako
apps/rust-subset-to-hako/smoke.sh
```

Expected gates:

```text
manifest_checked_in=1
module_artifact_checked_in=1
generated_skeleton_expected_checked_in=1
focused_wrapper_added=1
generated_skeleton_mir_emit=green
wrapper_exe_parity=green
```

## Non-Goals

```text
use_resolution_enabled=0
rust_name_resolution_enabled=0
module_namespace_linking_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
generated_program_execution_claim=0
new_hako_syntax_added=0
```

## Acceptance

```bash
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
RUST_SUBSET_RUN_ADAPTER=1 bash apps/rust-subset-to-hako/smoke.sh
```

Closeout report:

```text
manifest_checked_in=<0|1>
module_artifact_checked_in=<0|1>
generated_skeleton_expected_checked_in=<0|1>
focused_wrapper_added=<0|1>
generated_skeleton_mir_emit=<green|red>
wrapper_exe_parity=<green|red>
full_rust_subset_smoke=<green|red>
generated_program_execution_claim=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
new_hako_syntax_added=0
summary=<ok|blocked>
```

## Stop Line

Do not aggregate the full `hakorune_mir_builder` crate in this row. Do not turn
`Use` into executable import/name-resolution semantics. This row only checks in
the crate-root module as an explicit skeleton handoff.

## Closeout

Checked in:

```text
apps/rust-subset-to-hako/examples/hakorune_mir_builder_crate_root_expected/
apps/rust-subset-to-hako/examples/hakorune_mir_builder_crate_root_expected.hako
apps/rust-subset-to-hako/convert_hakorune_mir_builder_crate_root_crate_file.hako
```

Report:

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
new_hako_syntax_added=0
summary=ok
```
