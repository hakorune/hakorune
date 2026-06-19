# 296x-1332 HAKORUNE-MIR-CORE-RUSTSUBSET-PILOT-001

Status: closed
Date: 2026-06-20

## Purpose

Materialize the selected `hakorune_mir_core` RustSubset module slice and run it
through the existing skeleton pipeline.

This row is a focused crate-pilot slice. It does not add `.hako` syntax, Rust
name resolution, Rust `use` resolution, crate graph discovery, or generated
program execution semantics.

## Selected Slice

```text
selected_crate=hakorune_mir_core
selected_modules=crate::control_ids,crate::types
selected_module_count=2
```

Checked-in artifacts:

```text
apps/rust-subset-to-hako/examples/hakorune_mir_core_selected_expected/crate-manifest.json
apps/rust-subset-to-hako/examples/hakorune_mir_core_selected_expected/modules/0000.json
apps/rust-subset-to-hako/examples/hakorune_mir_core_selected_expected/modules/0001.json
apps/rust-subset-to-hako/examples/hakorune_mir_core_selected_expected.hako
apps/rust-subset-to-hako/convert_hakorune_mir_core_selected_crate_file.hako
```

The checked-in manifest is a selected-slice transport index:

```text
partial_crate_adapter_implemented=0
full_crate_graph_discovery_changed=0
selected_slice_manifest_checked_in=1
```

## Unsupported Expression Handoff

The selected `crate::types` module contains a `Display` impl whose Rust `match`
body is out of v0 scope. The converter now emits unsupported expressions as a
MIR-safe placeholder while preserving the TODO diagnostic:

```text
unsupported_expression_output=null /* TODO: <reason> */
unsupported_statement_output=TODO comment
unsupported_item_output=TODO comment
match_semantics_enabled=0
```

This keeps the skeleton parse/MIR-safe without claiming Rust `match` semantics.

## Evidence

Inventory:

```bash
python3 apps/rust-subset-to-hako/tools/crate_inventory.py \
  --manifest apps/rust-subset-to-hako/examples/hakorune_mir_core_selected_expected/crate-manifest.json
```

Result:

```text
module_count=2
module_0_id=crate::control_ids
module_0_unsupported_total=0
module_1_id=crate::types
module_1_unsupported_total=2
total_unsupported_rust_kind.Use=1
total_unsupported_rust_kind.<missing>=1
summary=ok
```

Python reference parity:

```bash
python3 apps/rust-subset-to-hako/convert.py \
  apps/rust-subset-to-hako/examples/hakorune_mir_core_selected_expected/modules/0000.json \
  > /tmp/mir_core_selected_python.hako
printf '\n' >> /tmp/mir_core_selected_python.hako
python3 apps/rust-subset-to-hako/convert.py \
  apps/rust-subset-to-hako/examples/hakorune_mir_core_selected_expected/modules/0001.json \
  >> /tmp/mir_core_selected_python.hako
diff -u \
  apps/rust-subset-to-hako/examples/hakorune_mir_core_selected_expected.hako \
  /tmp/mir_core_selected_python.hako
```

Generated skeleton acceptance:

```bash
NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --dump-ast \
  apps/rust-subset-to-hako/examples/hakorune_mir_core_selected_expected.hako

NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-mir-json \
  /tmp/hakorune_mir_core_selected_expected.mir.json \
  apps/rust-subset-to-hako/examples/hakorune_mir_core_selected_expected.hako
```

Result:

```text
generated_skeleton_dump_ast=ok
generated_skeleton_emit_mir_json=ok
```

Wrapper acceptance:

```bash
NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-mir-json \
  /tmp/convert_hakorune_mir_core_selected_crate_file.mir.json \
  apps/rust-subset-to-hako/convert_hakorune_mir_core_selected_crate_file.hako
```

Result:

```text
wrapper_emit_mir_json=ok
```

Known non-goal / next blocker:

```text
wrapper_emit_exe=blocked_by_existing_unsupported_pure_shape
mini_crate_wrapper_emit_exe_same_failure=1
hakorune_box_core_wrapper_emit_exe_same_failure=1
```

The EXE blocker is not specific to the selected `hakorune_mir_core` slice and
must be handled as a crate-wrapper route issue.

## Acceptance

```bash
python3 apps/rust-subset-to-hako/selftest.py
python3 apps/rust-subset-to-hako/tools/crate_inventory.py \
  --manifest apps/rust-subset-to-hako/examples/hakorune_mir_core_selected_expected/crate-manifest.json
NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-mir-json \
  /tmp/convert_hakorune_mir_core_selected_crate_file.mir.json \
  apps/rust-subset-to-hako/convert_hakorune_mir_core_selected_crate_file.hako
NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-mir-json \
  /tmp/hakorune_mir_core_selected_expected.mir.json \
  apps/rust-subset-to-hako/examples/hakorune_mir_core_selected_expected.hako
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
new_hako_syntax_added=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
match_semantics_enabled=0
generated_program_exe_aot_claim=0
converter_core_filebox_ownership=0
```

## Next

Continue:

```text
RUST-SUBSET-CRATE-WRAPPER-EXE-PURE-ROUTE-UNBLOCK-001
```

The next row should diagnose the existing `unsupported pure shape` failure for
crate handoff wrappers. It should not mix that route fix with another crate
pilot.
