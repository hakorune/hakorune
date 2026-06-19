# 296x-1319 RUST-SUBSET-SYN-ADAPTER-MULTI-MODULE-PROBE-001

Status: closed
Date: 2026-06-20

## Purpose

Implement the first synthetic crate-mode adapter probe after accepting the
crate manifest v0 contract.

This row proves that the host-side `syn` adapter can emit:

```text
crate-manifest.json
modules/0000.json
modules/0001.json
modules/0002.json
```

for a deterministic mini-crate fixture. It does not make `.hako` read the
manifest.

## Fixture

```text
apps/rust-subset-to-hako/examples/mini_crate/
  Cargo.toml
  src/lib.rs
  src/model.rs
  src/util.rs
```

Expected artifacts:

```text
apps/rust-subset-to-hako/examples/mini_crate_expected/
  crate-manifest.json
  modules/0000.json
  modules/0001.json
  modules/0002.json
```

## Implementation

```text
apps/rust-subset-to-hako/tools/syn_adapter/src/cli.rs
  Command::SingleFile
  Command::Crate

apps/rust-subset-to-hako/tools/syn_adapter/src/crate_mode.rs
  flat synthetic crate discovery
  root src/lib.rs
  external `mod foo;` declarations to src/foo.rs
  deterministic module order
  manifest/modules artifact writer

apps/rust-subset-to-hako/tools/syn_adapter/src/items.rs
  file_to_json_for_crate()
  external mod declarations excluded from per-module semantic JSON

apps/rust-subset-to-hako/smoke.sh
  adapter crate artifact diff under RUST_SUBSET_RUN_ADAPTER=1
```

## Accepted Scope

```text
supported_module_graph=flat_mod_declarations
root_source=src/lib.rs
child_source=src/<module>.rs
inline_mod_supported=0
path_attribute_supported=0
macro_generated_mod_supported=0
cfg_graph_selection_supported=0
converter_core_manifest_ownership=0
```

## Stop Line

```text
converter_core_filebox_ownership=0
converter_core_manifest_ownership=0
generated_hako_parse_claim=0
generated_program_exe_aot_claim=0
creat_specific_schema_enabled=0
workspace_scan_enabled=0
dependency_crates_included=0
rust_name_resolution_enabled=0
```

## Evidence

```bash
cargo check --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml
bash -n apps/rust-subset-to-hako/smoke.sh
bash apps/rust-subset-to-hako/smoke_adapter.sh
```

Final repository-wide gate:

```bash
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Next

```text
RUST-SUBSET-CRATE-HANDOFF-MIR-ACCEPTANCE-001
```

Implement the `.hako` crate handoff wrapper:

```text
manifest read / validation
artifact_path safety validation
FileBox reads for module artifacts
module id match validation
per-module converter invocation
generated_hako_parse=ok
generated_hako_mir_emit=ok
generated_program_exe_aot_claim=0
```
