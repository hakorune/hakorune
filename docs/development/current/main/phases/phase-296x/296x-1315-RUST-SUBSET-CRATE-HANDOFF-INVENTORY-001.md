# 296x-1315 RUST-SUBSET-CRATE-HANDOFF-INVENTORY-001

Status: closed
Date: 2026-06-19

## Purpose

Inventory the boundary needed before converting a Rust crate or a larger tool
such as `creat` through the rust-subset-to-hako app front.

This row is docs-only. It does not implement crate discovery, multi-file JSON,
new RustSubset schema nodes, converter semantics, or adapter invocation from
`.hako`.

## Current State

The selected v0 adapter is a host-side `syn` producer:

```text
apps/rust-subset-to-hako/tools/syn_adapter
```

It currently accepts exactly one input file:

```bash
cargo run --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml -- \
  path/to/input.rs \
  --module module_name \
  -o /tmp/module.json
```

Current output root:

```json
{
  "schema_version": 0,
  "kind": "RustSubsetModule",
  "module": "module_name",
  "items": []
}
```

The Hakorune-owned converter consumes RustSubset JSON text and emits `.hako`
skeleton code. It does not own Rust parsing, crate graph discovery, file graph
discovery, adapter subprocess execution, or output-directory layout.

## Boundary

External adapter owns:

```text
Rust source reading
Rust parser choice
crate root / file graph discovery
module path and source path collection
per-file RustSubsetModule production
unsupported Rust syntax classification
lossy Rust-to-RustSubset normalization
```

Hakorune converter owns:

```text
RustSubset JSON text input
json_native parsing
RustSubset schema validation
JsonNode traversal
.hako skeleton emission
EXE/AOT acceptance
```

Not owned in this row:

```text
trait semantics
generic semantics
macro expansion
Rust name resolution
module resolution
subprocess adapter invocation from .hako
generated output directory management from converter_core.hako
```

## Candidate Schema Direction

Do not replace the current `RustSubsetModule` root. Crate-wide handoff should
keep `RustSubsetModule` as the per-module semantic input and add a thin
manifest outside it.

Preferred next design direction:

```json
{
  "schema_version": 0,
  "kind": "RustSubsetCrateManifest",
  "crate_name": "mini_crate",
  "target": {
    "kind": "lib",
    "name": "mini_crate"
  },
  "root_module": "crate",
  "modules": [
    {
      "module": "crate",
      "source_path": "src/lib.rs",
      "artifact_path": "modules/0000.json"
    },
    {
      "module": "crate::model",
      "source_path": "src/model.rs",
      "artifact_path": "modules/0001.json"
    }
  ]
}
```

Each `artifact_path` points to the existing root contract:

```json
{
  "schema_version": 0,
  "kind": "RustSubsetModule",
  "module": "crate::model",
  "items": []
}
```

This is a candidate, not an accepted schema. The next design row should decide
whether to accept this B-prime contract:

```text
crate manifest JSON + per-module RustSubsetModule JSON
```

Rejected for the first formal contract:

```text
single giant crate JSON embedding all modules
creat-specific schema driven by the first pilot
converter_core-owned crate graph discovery
```

Transport rules to decide in the design row:

```text
unit=one selected Cargo target, not a whole workspace
dependency_crates=excluded
source_path=crate-root-relative diagnostic context
artifact_path=manifest-directory-relative, no absolute paths, no ".."
modules_order=adapter-owned deterministic order
manifest_module_must_match_module_json_module=1
missing_artifact_or_duplicate_module=fail-fast
```

Before manifest implementation, close the existing module validation gap:

```text
known Unsupported node -> TODO comment
unknown JSON kind -> fail-fast
schema_version != 0 -> fail-fast
root kind != RustSubsetModule -> fail-fast
```

## Stop Lines

```text
do not implement Rust parsing inside Hakorune
do not make converter_core.hako depend on FileBox/stdin/argv/subprocesses
do not add RustSubsetCrateManifest as accepted schema in this row
do not add trait/generic/macro/name-resolution semantics in this row
do not silently drop unsupported Rust constructs
do not claim generated .hako is semantically equivalent Rust
do not use filesystem iteration order as manifest order
do not put absolute host paths in checked-in JSON
do not accept artifact_path containing ".."
do not flatten all modules into one Hako namespace
do not add creat-specific names or paths to schema/converter logic
```

## Next Task Candidates

```text
RUST-SUBSET-MODULE-SCHEMA-VALIDATION-PARITY-001
  align .hako converter validation with Python reference before crate handoff

RUST-SUBSET-CRATE-MANIFEST-V0-001
  decide B-prime manifest + per-module artifact contract

UNSUPPORTED-DIAGNOSTICS-V0
  add code/rust_kind/summary and optional span; source_path stays manifest context

MULTI-FILE-ADAPTER-PROBE
  synthetic mini crate emits manifest + multiple RustSubsetModule artifacts

CREAT-SUBSET-PILOT-SELECTION
  inventory creat after synthetic crate handoff is green
```

## Evidence

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Result

```text
implementation_allowed=0
crate_handoff_schema_accepted=0
current_adapter_single_file=1
converter_core_changed=0
input_route_changed=0
vm_product_route=retired
next_blocker=RUST-SUBSET-MODULE-SCHEMA-VALIDATION-PARITY-001
summary=ok
```
