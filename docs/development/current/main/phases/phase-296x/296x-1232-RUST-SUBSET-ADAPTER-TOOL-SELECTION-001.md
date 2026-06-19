---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Select and scaffold the external Rust parser adapter route for rust-subset-to-hako.
Related:
  - apps/rust-subset-to-hako/schema/external-adapter-boundary-v0.md
  - apps/rust-subset-to-hako/tools/syn_adapter
  - apps/rust-subset-to-hako/examples/adapter_fixture_input.rs
  - apps/rust-subset-to-hako/examples/adapter_fixture_subset.json
  - apps/rust-subset-to-hako/STATUS.md
---

# RUST-SUBSET-ADAPTER-TOOL-SELECTION-001

## Decision

Use a small external `syn`-based adapter as the first Rust source producer.

```text
selected_tool=syn
adapter_location=apps/rust-subset-to-hako/tools/syn_adapter
adapter_owned_by_hakorune_app=0
handoff_artifact=RustSubset JSON v0 file
converter_core_changed=0
```

## Why `syn`

The v0 schema is AST-shaped and only needs a conservative Rust subset:

```text
struct
enum
fn
impl fn
let
return
field/binary/call/method-call/literal/name expressions
```

`syn` gives typed Rust syntax nodes for exactly this without introducing a
larger compiler-server dependency. `tree-sitter-rust` remains useful for loose
editor-style parsing, and `rust-analyzer` remains a future semantic option, but
both are heavier than needed for the first file-based producer.

## Boundary

The adapter is a host-side producer only.

```text
Rust source -> syn adapter -> RustSubset JSON v0 file
RustSubset JSON v0 file -> convert_adapter_fixture.hako / convert_file.hako
```

The `.hako` converter core still starts at RustSubset JSON text.

## Identity Policy

The adapter does not infer identity from field names or method names.

```text
#[hako_identity]
struct Counter { ... }
```

is the explicit v0 marker for:

```json
{"identity": true, "identity_reason": "resource_or_mutable_state"}
```

Unmarked structs are emitted as value aggregates:

```json
{"identity": false}
```

## Stop Lines

```text
do not implement Rust parsing inside .hako
do not make converter_core.hako depend on adapter details
do not infer box identity from names or mutability heuristics
do not silently drop unsupported Rust items
do not claim Rust semantic equivalence
do not re-enable VM product route
```

## Acceptance

```text
output_contract=rust-subset-adapter-tool-selection-v0

selected_tool=syn
syn_adapter_scaffold=1
adapter_fixture_input=apps/rust-subset-to-hako/examples/adapter_fixture_input.rs
adapter_fixture_json_matches_checked_fixture=1
adapter_fixture_handoff_parity=ok
converter_core_changed=0
vm_product_route=retired

summary=ok
```
