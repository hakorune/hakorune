# 296x-1335 RUST-SUBSET-CRATE-WRAPPER-EXE-SMOKE-001

Status: closed
Date: 2026-06-20

## Purpose

Pin the crate-wrapper EXE route that was unblocked by 296x-1333.

The crate bundles and generated skeleton MIR acceptance are already checked in.
The shared wrappers now compile through EXE, so the next step is to make that
route a stable app-front smoke instead of relying on ad-hoc commands.

## Scope

Add a focused smoke for the three existing crate-wrapper entrypoints:

```text
apps/rust-subset-to-hako/convert_crate_file.hako
apps/rust-subset-to-hako/convert_hakorune_box_core_crate_file.hako
apps/rust-subset-to-hako/convert_hakorune_mir_core_selected_crate_file.hako
```

The smoke should verify `--emit-exe` success only. It must not claim that the
generated `.hako` programs are executable applications.

```text
converter_wrapper_exe_route_pinned=1
generated_program_exe_aot_claim=0
converter_core_changed=0
rust_parser_changed=0
crate_graph_discovery_changed=0
```

## Acceptance

The new smoke or check script runs these wrapper commands:

```bash
NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-exe \
  /tmp/convert_crate_file_exe \
  apps/rust-subset-to-hako/convert_crate_file.hako

NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-exe \
  /tmp/convert_hakorune_box_core_crate_file_exe \
  apps/rust-subset-to-hako/convert_hakorune_box_core_crate_file.hako

NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-exe \
  /tmp/convert_hakorune_mir_core_selected_crate_file_exe \
  apps/rust-subset-to-hako/convert_hakorune_mir_core_selected_crate_file.hako
```

General checks:

```bash
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
new_hako_syntax_added=0
json_native_changed=0
rust_subset_converter_changed=0
route_selection_changed=0
generated_program_execution_claim=0
```

## Result

Closed by adding a focused wrapper-only EXE smoke:

```text
apps/rust-subset-to-hako/smoke_crate_wrappers_exe.sh
```

Verified wrappers:

```text
convert_crate_file.hako
convert_hakorune_box_core_crate_file.hako
convert_hakorune_mir_core_selected_crate_file.hako
```

The smoke checks only that the converter wrapper entrypoints compile through
`--emit-exe`. It does not execute or claim generated `.hako` programs.

Result:

```text
converter_wrapper_exe_route_pinned=1
generated_program_exe_aot_claim=0
summary=ok
```

## Next

After this smoke is pinned, return to crate/app-front pilot selection or the
next real RustSubset source-shape blocker exposed by the smoke.
