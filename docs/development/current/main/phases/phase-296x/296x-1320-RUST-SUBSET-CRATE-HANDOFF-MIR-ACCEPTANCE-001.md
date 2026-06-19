# 296x-1320 RUST-SUBSET-CRATE-HANDOFF-MIR-ACCEPTANCE-001

Status: closed
Date: 2026-06-20

## Purpose

Implement the first `.hako` crate handoff wrapper after the synthetic
multi-module adapter probe.

This row proves:

```text
crate-manifest.json -> FileBox read
manifest validation -> ok
module artifact reads -> ok
module id match -> ok
per-module converter invocation -> ok
generated skeleton MIR emit -> ok
```

`converter_core.hako` remains the `RustSubsetModule` JSON to `.hako` skeleton
owner. It still does not own manifests, FileBox, crate graph discovery, Rust
parsing, or adapter invocation.

## Implementation

```text
apps/rust-subset-to-hako/convert_crate_file.hako
  synthetic mini-crate crate handoff wrapper
  manifest validation
  artifact path safety / expected artifact checks
  FileBox reads for three module artifacts
  module id match validation
  per-module RustSubsetConverter.convert() calls

apps/rust-subset-to-hako/examples/mini_crate_expected.hako
  expected crate handoff skeleton

apps/rust-subset-to-hako/smoke.sh
  crate handoff fixture parity
  generated skeleton MIR acceptance
```

## Accepted Scope

```text
input_contract=RustSubsetCrateManifest-v0
semantic_module_input=RustSubsetModule-v0
fixture=synthetic_mini_crate
module_count=3
manifest_read=ok
all_artifacts_read=ok
module_id_match=ok
per_module_golden=ok
converter_app_exe_aot=ok
generated_hako_parse=ok
generated_hako_mir_emit=ok
generated_hako_exe_aot_claim=0
```

The mini-crate fixture intentionally contains record declarations and an
Unsupported trait handoff only. Top-level function skeleton transport is already
covered by dedicated converter parity fixtures and remains outside this crate
handoff MIR-acceptance row.

## Stop Line

```text
converter_core_manifest_ownership=0
converter_core_filebox_ownership=0
external_adapter_invocation_from_hako=0
rust_parser_owned_by_hako=0
general_dynamic_artifact_path_runner=0
generated_program_exe_aot_claim=0
cross_module_symbol_resolution=0
creat_specific_schema_enabled=0
```

The wrapper validates `artifact_path` safety and expected manifest entries, then
reads the synthetic fixture artifacts by fixed known paths. General dynamic
artifact-path iteration is a later input-route/compiler-acceptance row, not this
row.

## Evidence

```bash
bash apps/rust-subset-to-hako/smoke.sh
```

Focused checks used while implementing:

```bash
NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-exe \
  /tmp/hako_rust_subset_convert_crate_file \
  apps/rust-subset-to-hako/convert_crate_file.hako

NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-mir-json \
  /tmp/rust_subset_mini_crate_handoff_generated.mir.json \
  /tmp/rust_subset_mini_crate_handoff_generated.hako
```

## Next

```text
CREAT-SUBSET-PILOT-SELECTION-001
```

Inventory a real `creat` subset before adding more schema nodes or language
surface.
