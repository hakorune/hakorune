# 296x-1341 HAKORUNE-MIR-CORE-ID-MODULES-MATERIALIZATION-001

Status: closed
Date: 2026-06-20

## Purpose

Materialize the selected `hakorune_mir_core` ID-module RustSubset bundle now
that the focused skeleton source-shape blockers have been cleared.

Selected modules:

```text
crate::basic_block_id
crate::binding_id
crate::value_id
```

## Evidence

The local re-probe after 296x-1340 generated a combined skeleton for the three
selected modules and reached MIR emit:

```bash
NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-mir-json \
  /tmp/hakorune_mir_core_id_modules_generated.mir.json \
  /tmp/hakorune_mir_core_id_modules_generated.hako
```

Result:

```text
generated_skeleton_mir_emit=green
```

## Scope

Allowed:

```text
adapter_generated_json_checked_in=1
selected_manifest_checked_in=1
converter_wrapper_updated_or_new=1
generated_skeleton_expected_checked_in=1
generated_skeleton_mir_emit=1
wrapper_emit_exe=1
```

Not allowed:

```text
generated_program_execution_claim=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
new_hako_syntax_added=0
```

## Acceptance

Check in a selected bundle under:

```text
apps/rust-subset-to-hako/examples/hakorune_mir_core_id_modules_expected/
```

Add/update a focused wrapper, then verify:

```bash
NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-mir-json \
  /tmp/hakorune_mir_core_id_modules_expected.mir.json \
  apps/rust-subset-to-hako/examples/hakorune_mir_core_id_modules_expected.hako

NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-exe \
  /tmp/convert_hakorune_mir_core_id_modules_crate_file \
  apps/rust-subset-to-hako/convert_hakorune_mir_core_id_modules_crate_file.hako
```

General checks:

```bash
cargo check -q --lib
RUST_SUBSET_RUN_ADAPTER=1 bash apps/rust-subset-to-hako/smoke.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
generated_program_execution_claim=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
new_hako_syntax_added=0
```

## Result

```text
adapter_generated_json_checked_in=1
selected_manifest_checked_in=1
converter_wrapper_added=1
generated_skeleton_expected_checked_in=1
generated_skeleton_mir_emit=1
wrapper_emit_exe=1
generated_program_execution_claim=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
new_hako_syntax_added=0
summary=ok
```

Checked-in artifacts:

```text
apps/rust-subset-to-hako/examples/hakorune_mir_core_id_modules_expected/crate-manifest.json
apps/rust-subset-to-hako/examples/hakorune_mir_core_id_modules_expected/modules/0000.json
apps/rust-subset-to-hako/examples/hakorune_mir_core_id_modules_expected/modules/0001.json
apps/rust-subset-to-hako/examples/hakorune_mir_core_id_modules_expected/modules/0002.json
apps/rust-subset-to-hako/examples/hakorune_mir_core_id_modules_expected.hako
apps/rust-subset-to-hako/convert_hakorune_mir_core_id_modules_crate_file.hako
```

Verification:

```text
hakorune_mir_core_id_modules_expected_mir_emit=green
convert_hakorune_mir_core_id_modules_crate_file_exe_parity=green
smoke_crate_wrappers_exe=green
RUST_SUBSET_RUN_ADAPTER=1 apps/rust-subset-to-hako/smoke.sh=green
cargo_check_lib=green
current_state_pointer_guard=green
git_diff_check=green
```

## Next

Continue to a selection row before opening the next implementation slice:

```text
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-002
```
