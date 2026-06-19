# rust-subset-syn-adapter

Host-side Rust parser adapter for the rust-subset-to-hako app front.

This tool is outside the Hakorune-owned converter core. It converts Rust source
into RustSubset JSON v0:

```text
Rust source -> syn adapter -> RustSubset JSON v0 -> .hako converter app
```

## Scope

Supported v0 shapes:

```text
struct
enum
fn
impl fn
let
return
literal/name/field/binary/call/method-call expressions
```

Unsupported Rust syntax is represented as explicit `Unsupported` nodes where
possible. The converter remains fail-fast for unknown JSON kinds.

## Identity

The adapter does not infer `box` identity from names, methods, or mutability.
Use an explicit marker:

```rust
#[hako_identity]
struct Counter {
    value: i64,
}
```

This emits:

```json
{
  "kind": "Struct",
  "identity": true,
  "identity_reason": "resource_or_mutable_state"
}
```

Unmarked structs remain `identity=false` and convert to `record`.

## Usage

```bash
cargo run --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml -- \
  apps/rust-subset-to-hako/examples/adapter_fixture_input.rs \
  --module adapter_fixture \
  -o /tmp/adapter_fixture_subset.json
```

Optional smoke gate:

```bash
RUST_SUBSET_RUN_ADAPTER=1 bash apps/rust-subset-to-hako/smoke.sh
```

## Crate Handoff Boundary

The current adapter is intentionally single-file:

```text
one input .rs file -> one RustSubsetModule JSON file
```

Crate-wide or multi-file handoff is a separate design row. It must not move
crate graph discovery, Rust name resolution, or adapter invocation into
`converter_core.hako`.

The preferred next contract is:

```text
RustSubsetCrateManifest JSON -> per-module RustSubsetModule JSON artifacts
```

The manifest is only a transport index. Per-module `RustSubsetModule` remains
the semantic input consumed by the converter core.

## Stop Lines

```text
do not move Rust parsing into .hako
do not make converter_core.hako depend on syn output quirks
do not silently drop unsupported Rust constructs
do not infer identity from source names
do not claim crate-wide handoff from the single-file adapter
```
