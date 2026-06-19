# External Adapter Boundary v0

Status: accepted boundary for `RUST-SUBSET-EXTERNAL-ADAPTER-BOUNDARY-001`.

## Purpose

The rust-subset-to-hako app owns conversion from RustSubset JSON v0 to `.hako`
skeleton code. It does not own Rust parsing.

```text
Rust source
  -> external adapter
  -> RustSubset JSON v0
  -> rust-subset-to-hako
  -> .hako skeleton
```

## Ownership

External adapter owns:

```text
Rust source reading
Rust parser choice: syn / tree-sitter-rust / rust-analyzer
Rust AST traversal
lossy Rust-to-RustSubset normalization
diagnostics for unsupported Rust source shapes
```

Hakorune app owns:

```text
RustSubset JSON v0 parsing through json_native
RustSubset schema validation
RustSubset node traversal
.hako skeleton emission
EXE/AOT acceptance
```

## Input Contract

The adapter must produce UTF-8 JSON matching `RustSubset-v0.md`.

Required root fields:

```json
{
  "schema_version": 0,
  "kind": "RustSubsetModule",
  "items": []
}
```

Unknown Rust constructs must not be silently dropped. The adapter must encode
known-but-unsupported shapes as an explicit unsupported node or fail before
handoff.

```json
{
  "kind": "Unsupported",
  "rust_kind": "Trait",
  "summary": "trait items are out of v0 scope"
}
```

Unknown JSON item/expression/statement kinds remain fail-fast in the converter.

## Output Route

The handoff artifact is a JSON file consumed by `convert_file.hako` through
FileBox.

```text
adapter output file -> convert_file.hako -> converter_core.hako
```

`convert.hako` remains the embedded-fixture wrapper and must not gain adapter
or FileBox ownership.

## Stop Lines

```text
do not implement the Rust parser inside Hakorune in this row
do not call a host JSON parser from converter_core.hako
do not bypass json_native in the Hakorune-owned app route
do not make converter_core.hako depend on FileBox/stdin/argv
do not silently drop unsupported Rust constructs
do not treat generated .hako as semantically equivalent Rust
```

## Acceptance

```text
external_adapter_owned_by_hako=0
adapter_output_contract=RustSubset JSON v0
adapter_output_route=file
hako_input_wrapper=convert_file.hako
converter_core_changed_for_adapter=0
vm_product_route=retired
primary_route=EXE/AOT
summary=ok
```

## Selected v0 Adapter

`RUST-SUBSET-ADAPTER-TOOL-SELECTION-001` selects a small `syn`-based host
producer:

```text
apps/rust-subset-to-hako/tools/syn_adapter
```

Reason:

```text
syn directly exposes the item/expr/stmt shapes used by RustSubset JSON v0
tree-sitter-rust is deferred for loose editor-style parsing
rust-analyzer is deferred for semantic analysis
```

The selected adapter is still replaceable because the handoff contract remains
the RustSubset JSON v0 file.

## Crate / Multi-file Handoff Status

The current accepted handoff is one Rust source file to one `RustSubsetModule`
JSON document. Crate-wide handoff is not accepted yet.

```text
current_adapter_input=single_rust_file
current_adapter_output=RustSubsetModule
crate_handoff_schema_accepted=0
multi_file_adapter_enabled=0
```

Future crate handoff must keep this ownership split:

```text
external adapter:
  crate/file graph discovery
  module path/source path collection
  per-file RustSubset module production

Hakorune converter:
  RustSubset JSON text parsing
  schema validation
  .hako skeleton emission
```

The preferred next formal contract is a crate manifest JSON that points at
per-module `RustSubsetModule` JSON artifacts. The manifest is a transport
index, not a semantic AST replacement for `RustSubsetModule`.

Candidate crate-level artifacts and stop lines are documented in:

```text
docs/development/current/main/phases/phase-296x/296x-1315-RUST-SUBSET-CRATE-HANDOFF-INVENTORY-001.md
```
