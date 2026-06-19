# 296x-1318 RUST-SUBSET-CRATE-MANIFEST-V0-001

Status: closed
Date: 2026-06-20

## Purpose

Accept the crate-wide handoff contract before implementing multi-module adapter
output or running a `creat`-style pilot.

This row is schema/docs only. It does not implement crate graph discovery,
multi-file adapter output, manifest reading in `.hako`, output directory
management, or generated program execution.

## Decision

Accept B-prime:

```text
RustSubsetCrateManifest JSON
  + per-module RustSubsetModule JSON artifacts
```

Rejected as the first formal contract:

```text
single giant crate JSON embedding all modules
creat-specific schema
converter_core-owned crate graph discovery
workspace/package-wide conversion by default
```

The manifest is a transport index. `RustSubsetModule` remains the per-module
semantic input SSOT.

## Schema

```text
apps/rust-subset-to-hako/schema/CrateManifest-v0.md
```

Root:

```json
{
  "schema_version": 0,
  "kind": "RustSubsetCrateManifest",
  "crate_name": "mini_crate",
  "target": {"kind": "lib", "name": "mini_crate"},
  "root_module": "crate",
  "modules": [
    {
      "module": "crate",
      "source_path": "src/lib.rs",
      "artifact_path": "modules/0000.json"
    }
  ]
}
```

## Ownership

External adapter owns:

```text
Cargo target selection
crate root discovery
mod file graph discovery
deterministic module ordering
source_path / artifact_path assignment
Rust parsing
per-module RustSubsetModule artifact production
graph-incomplete fail-fast
```

Hakorune crate handoff wrapper may own later:

```text
manifest parsing
manifest schema validation
artifact_path safety validation
FileBox reads for listed artifacts
module id match validation
per-module converter invocation
```

`converter_core.hako` still owns only:

```text
RustSubsetModule JSON text -> .hako skeleton
```

## Stop Line

```text
multi_file_adapter_enabled=0
converter_core_manifest_ownership=0
converter_core_filebox_ownership=0
rust_parser_inside_hakorune=0
cargo_metadata_inside_hakorune=0
dependency_crates_included=0
giant_crate_json_enabled=0
creat_specific_schema_enabled=0
generated_program_exe_aot_claim=0
```

## Evidence

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

Schema/docs consistency:

```bash
rg -n "CrateManifest-v0|RustSubsetCrateManifest|crate_handoff_schema_accepted" \
  apps/rust-subset-to-hako docs/development/current/main
```

## Next

```text
RUST-SUBSET-SYN-ADAPTER-MULTI-MODULE-PROBE-001
```

Implement the first synthetic mini-crate probe:

```text
mini-crate/
  Cargo.toml
  src/lib.rs
  src/model.rs
  src/util.rs
```

Expected artifacts:

```text
crate-manifest.json
modules/0000.json
modules/0001.json
modules/0002.json
```

The probe must not make `converter_core.hako` read the manifest.
