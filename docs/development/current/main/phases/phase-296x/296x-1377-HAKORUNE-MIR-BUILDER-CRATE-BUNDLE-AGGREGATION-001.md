# 296x-1377 HAKORUNE-MIR-BUILDER-CRATE-BUNDLE-AGGREGATION-001

Status: open
Date: 2026-06-20

## Purpose

Aggregate the already materialized `hakorune_mir_builder` crate into one
crate-mode RustSubset bundle and consume it through one manifest-driven Hako
FileBox route.

This row closes the crate-level transport milestone after all 7 modules were
individually materialized.

## Selected By

```text
296x-1376-RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-018
```

## Scope

Implementation shape:

```text
implementation_shape=A2-lite manifest-driven reusable file-route helper
```

Add a thin helper:

```text
apps/rust-subset-to-hako/crate_bundle_file_route.hako

RustSubsetCrateBundleFileRouteBox:
  convert_bundle(
    manifest_path,
    bundle_root,
    expected_crate_name,
    expected_module_count
  ) -> String | null
```

Helper responsibilities:

```text
manifest FileBox read
manifest v0 validation
safe relative path validation
duplicate module/artifact rejection
manifest order iteration
artifact FileBox read
module root validation
manifest module id match
RustSubsetConverter.convert(module_text)
module/source framing
```

Check in one real crate-mode bundle:

```text
apps/rust-subset-to-hako/examples/hakorune_mir_builder_crate_expected/
  crate-manifest.json
  modules/0000.json
  ...
  modules/0006.json
```

Add one wrapper:

```text
apps/rust-subset-to-hako/convert_hakorune_mir_builder_crate_file.hako
```

## Acceptance

```text
adapter_crate_mode_bundle_golden=green
manifest_bundle_checked_in=1
manifest_schema_version=0
manifest_kind=RustSubsetCrateManifest
manifest_crate_name=hakorune_mir_builder
manifest_target_kind=lib
manifest_root_module=crate
module_count=7
manifest_order_preserved=1
root_module_present_exactly_once=1
duplicate_module_id_rejected=1
duplicate_artifact_path_rejected=1
all_source_paths_safe=1
all_artifact_paths_safe=1
all_artifacts_read=1
all_artifact_roots=RustSubsetModule
all_module_ids_match=1
all_module_outputs_generated=1
stable_module_source_framing=1
bundle_output_golden=green
wrapper_exe_parity=green
existing_single_module_mir_gates=green
aggregate_text_mir_emit=green
aggregate_text_mir_emit_scope=fixture_only
generated_program_execution_claim=0
cross_module_linking_claim=0
combined_namespace_semantics_claim=0
```

Commands:

```bash
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
RUST_SUBSET_RUN_ADAPTER=1 bash apps/rust-subset-to-hako/smoke.sh
```

## Stop Line

```text
do_not_enable_use_resolution=1
do_not_interpret_Use_contents=1
do_not_deduplicate_Use_comments=1
do_not_derive_module_order_from_Use=1
do_not_rename_declarations_from_Use_aliases=1
do_not_connect_cross_module_calls=1
generated_program_execution_claim=0
```

Do not migrate existing single-module wrappers to the new helper in this row.
That cleanup is separate from proving the crate-bundle route.
