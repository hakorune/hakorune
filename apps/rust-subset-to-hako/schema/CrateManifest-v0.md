# RustSubset Crate Manifest v0

Status: accepted schema contract for `RUST-SUBSET-CRATE-MANIFEST-V0-001`.

## Purpose

Describe a crate-wide handoff without replacing `RustSubsetModule`.

The manifest is a transport index. It tells the Hakorune app-front which
per-module RustSubset JSON artifacts belong to one selected Cargo target.
It is not a semantic AST, name resolver, module linker, or generated-Hako
namespace model.

```text
Cargo target
  -> external adapter
  -> RustSubsetCrateManifest
  -> per-module RustSubsetModule artifacts
  -> Hakorune crate handoff wrapper
  -> RustSubsetConverter.convert_with_context(...)
```

## Root Shape

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

Each `artifact_path` points at an existing `RustSubsetModule` document:

```json
{
  "schema_version": 0,
  "kind": "RustSubsetModule",
  "module": "crate::model",
  "items": []
}
```

## Scope

The manifest unit is exactly one selected Cargo target:

```text
allowed:
  one lib target
  one bin target

not allowed:
  whole workspace
  whole Cargo package with multiple targets
  dependency crates
  examples/tests/benches auto-enumeration
```

The external adapter owns target selection. The Hakorune converter does not
inspect Cargo metadata.

## Fields

### `schema_version`

Must be integer `0`.

### `kind`

Must be string `"RustSubsetCrateManifest"`.

### `crate_name`

Diagnostic crate name. It must not be used as a resolver key.

### `target`

Selected Cargo target descriptor:

```json
{"kind": "lib", "name": "mini_crate"}
```

Allowed `target.kind` values in v0:

```text
lib
bin
```

### `root_module`

Canonical module id for the crate root. V0 uses `"crate"`.

The value must match one entry in `modules`.

### `modules`

Deterministic ordered list of module artifacts. The adapter owns ordering.
Filesystem enumeration order must not define the order.

Each entry:

```json
{
  "module": "crate::util",
  "source_path": "src/util.rs",
  "artifact_path": "modules/0002.json"
}
```

Rules:

- `module` is a canonical id, unique within the manifest.
- `source_path` is crate-root-relative diagnostic context.
- `artifact_path` is relative to the manifest directory.
- absolute `source_path` and `artifact_path` values are forbidden in
  checked-in artifacts.
- `artifact_path` must not contain `..`.
- every artifact must exist when the handoff wrapper validates the manifest.
- artifact root must be `RustSubsetModule`.
- artifact `module` must equal the manifest entry `module`.
- duplicate module ids and duplicate artifact paths are fail-fast.

## Adapter Ownership

The external adapter owns:

```text
Cargo target selection
crate root discovery
mod file graph discovery
#[path] module file mapping when supported
deterministic module ordering
source_path / artifact_path assignment
Rust parsing
per-module RustSubsetModule JSON production
Unsupported syntax classification
```

The adapter must fail before writing a manifest when graph completeness is not
proven.

```text
fail-fast:
  missing module file
  unresolved external module declaration
  include! module source
  macro-generated module source
  cfg-dependent graph without explicit selected cfg set

allowed Unsupported node:
  source item/expression/statement shape that is parsed but out of RustSubset v0
```

Missing source files are graph failures, not `Unsupported` semantic nodes.

## Hakorune App Ownership

A future crate handoff wrapper may own:

```text
manifest JSON parsing
manifest schema validation
artifact_path safety validation
FileBox reads for listed module artifacts
module id match validation
deterministic iteration in manifest order
per-module call into converter core
per-module skeleton output framing
```

`converter_core.hako` continues to own only:

```text
RustSubsetModule JSON text
schema validation
JsonNode traversal
.hako skeleton emission
```

`converter_core.hako` must not read files, invoke adapters, inspect Cargo
metadata, or discover crate/module graphs.

## Unsupported Diagnostics Context

Per-module artifacts may contain explicit `Unsupported` nodes. The manifest
provides the module/source context for reporting:

```text
module      = manifest.modules[i].module
source_path = manifest.modules[i].source_path
```

`Unsupported` nodes should carry source-local fields such as:

```json
{
  "kind": "Unsupported",
  "code": "RSV0-ITEM-TRAIT",
  "rust_kind": "Trait",
  "summary": "Trait items are out of v0 scope"
}
```

Spans and excerpts are optional in v0. Raw source excerpts are diagnostic only
and must not become semantic evidence.

## Stop Lines

```text
do not embed all modules into one giant RustSubset JSON
do not replace RustSubsetModule as the per-module semantic input
do not treat the manifest as a name resolver or module linker
do not flatten all modules into one Hako namespace in converter_core
do not include dependency crates
do not accept absolute artifact_path
do not accept artifact_path containing ".."
do not accept duplicate module ids or duplicate artifact paths
do not silently continue after incomplete graph discovery
do not implement Rust parser or Cargo metadata inspection in Hakorune
do not add creat-specific names or paths to the schema
do not claim generated skeleton EXE/AOT execution before an entrypoint contract
```

## First Implementation Row

The next implementation row should be a synthetic mini-crate probe:

```text
RUST-SUBSET-SYN-ADAPTER-MULTI-MODULE-PROBE-001
```

Expected first fixture:

```text
mini-crate/
  Cargo.toml
  src/lib.rs
  src/model.rs
  src/util.rs
```

The row should emit:

```text
crate-manifest.json
modules/0000.json
modules/0001.json
modules/0002.json
```

It must not yet make `converter_core.hako` read the manifest.
