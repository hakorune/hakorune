# 296x-1382 HAKORUNE-MIR-BUILDER-CRATE-BUNDLE-AGGREGATION-RESUME-001

Status: open
Date: 2026-06-20

## Purpose

Resume the `hakorune_mir_builder` 7-module crate-bundle aggregation after
296x-1380 proved dynamic FileBox reads inside a `Main`-owned loop on the EXE
pure route.

This replaces the blocked helper-owned FileBox shape from 296x-1377 with the
current input-route boundary:

```text
Main:
  owns FileBox reads
  owns dynamic path loop

converter / manifest helpers:
  parse and validate already-read JSON text
  do not own FileBox
```

## Selected By

```text
296x-1380-FILEBOX-DYNAMIC-PATH-LOOP-EXE-SHAPE-001
```

## Scope

Implement one crate-bundle wrapper for:

```text
apps/rust-subset-to-hako/examples/hakorune_mir_builder_crate_expected/
  crate-manifest.json
  modules/0000.json
  ...
  modules/0006.json
```

Allowed shape:

```text
static box Main:
  read manifest using FileBox
  iterate manifest-order module artifact paths
  read each artifact using FileBox in the same input-route surface
  pass module JSON text to RustSubsetConverter
  emit stable module/source framing
```

Optional helper shape:

```text
helpers may validate JsonNode / module metadata
helpers may format output
helpers must not create or operate FileBox
```

## Acceptance

```text
manifest_bundle_checked_in=1
manifest_schema_version=0
manifest_kind=RustSubsetCrateManifest
manifest_crate_name=hakorune_mir_builder
module_count=7
manifest_order_preserved=1
all_artifacts_read=1
all_artifact_roots=RustSubsetModule
all_module_ids_match=1
all_module_outputs_generated=1
stable_module_source_framing=1
bundle_output_golden=green
wrapper_exe_parity=green
aggregate_text_mir_emit=green
aggregate_text_mir_emit_scope=fixture_only
generated_program_execution_claim=0
cross_module_linking_claim=0
combined_namespace_semantics_claim=0
```

Checks:

```bash
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
RUST_SUBSET_RUN_ADAPTER=1 bash apps/rust-subset-to-hako/smoke.sh
```

## Stop Line

```text
do_not_reopen_helper_owned_FileBox=1
do_not_hand_unroll_7_module_wrapper=1
do_not_enable_use_resolution=1
do_not_enable_name_resolution=1
do_not_connect_cross_module_calls=1
generated_program_execution_claim=0
converter_core_changed_only_if_needed=1
```

If the `Main`-owned dynamic loop cannot express manifest iteration without
opening a new compiler shape, stop and taskize that compiler shape before
editing converter semantics.
