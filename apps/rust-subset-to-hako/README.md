# rust-subset-to-hako

Status: v0 embedded-fixture `.hako` converter passes EXE/AOT parity

Purpose: provide a small real-app front that converts a conservative Rust subset
model into `.hako` skeleton code.

This is not a full Rust transpiler. The first useful app is:

```text
Rust source
  -> external parser adapter (syn / tree-sitter-rust / rust-analyzer)
  -> RustSubset JSON v0
  -> .hako converter app
  -> Hako skeleton
```

The Hakorune-owned part starts at `RustSubset JSON v0`. That keeps the first
front small enough to test string, array, map, record, box, enum, function, and
error handling shapes without building a Rust parser first.

## Why This App

This app is a good post-fastpath compiler construction front because it is:

- close to selfhost/compiler work
- larger than a microbench
- still scopeable
- useful for future Rust-to-Hakorune migration sketches
- strong at exercising structured data and text generation

## V0 Goal

Read a RustSubset JSON document and emit `.hako` skeleton code.

V0 accepts:

- `struct` with named fields
- simple `enum` with unit or tuple-like variants
- free `fn`
- `impl Type { fn ... }`
- simple `let`
- simple `return`
- method call / function call skeletons
- field access skeletons
- integer, string, bool, null-like placeholder literals

V0 emits:

- value-like Rust structs as `record`
- impl methods as top-level functions with `me` as the first argument
- unsupported bodies as `/* TODO */` stubs
- stable comments for unsupported Rust constructs

## Non Goals

V0 does not implement:

- borrow checking
- lifetime semantics
- macro expansion
- trait resolution
- generics beyond preserving names as comments
- full pattern matching
- async/await
- unsafe
- procedural macro support
- semantic equivalence with Rust

## Files

- `DESIGN.md`: handoff design for another AI/worker
- `schema/RustSubset-v0.md`: normalized input shape
- `examples/simple_input.rs`: sample source for external adapter
- `examples/simple_subset.json`: sample normalized input
- `examples/simple_expected.hako`: expected skeleton output

## Current Implementation

Two converters exist:

- `convert.py`: reference implementation and parity oracle.
- `convert.hako`: native `.hako` implementation using `apps/lib/json_native`.

The Python reference currently passes:

```bash
python3 apps/rust-subset-to-hako/selftest.py
```

The `.hako` converter now emits MIR JSON and compiles/runs through EXE/AOT for
the embedded `simple_subset.json` fixture.

Current accepted slice:

```text
python_reference_selftest=ok
json_native_probe_exe=ok
hako_converter_mir_json_emit=ok
hako_converter_exe=ok
hako_converter_parity=simple_expected.hako
vm_product_route=retired
primary_route=EXE/AOT
```

Reproduce the current state:

```bash
bash apps/rust-subset-to-hako/smoke.sh
```

## Current Scope Boundary

The first AOT slice intentionally embeds the sample RustSubset JSON in
`convert.hako`. That keeps the acceptance target focused on:

```text
JSON parse -> JsonNode traversal -> RustSubset skeleton emission -> EXE/AOT
```

File/stdin input is a separate follow-up row. `FileBox` is not part of the
current AOT acceptance slice.

`json_native` also contains a temporary RustSubset schema-key interning bridge
in `lexer/tokenizer.hako`. It exists because scanner-derived substrings can
behave as unstable `MapBox` keys on the current EXE/AOT route. Remove that
bridge after dynamic `StringBox` materialization / `MapBox` key canonicalization
accepts scanner-derived strings without literal interning.

## Suggested Next Implementation

Keep the first `.hako` converter slice over `examples/simple_subset.json`.

```text
input:  RustSubset JSON v0
output: Hako skeleton text
```

Do not implement the external Rust parser adapter in the first slice. If an
adapter is needed later, use Rust `syn` or `tree-sitter-rust` outside the
Hakorune app and keep it as a replaceable producer of the same JSON schema.

Do not bypass `json_native` with a native JSON DLL in this row. The app is useful
because it exercises real `.hako` JSON/tree traversal pressure.

## Acceptance

```text
rust_source_parser_owned_by_hako=0
rust_subset_json_schema_defined=1
hako_converter_scope=v0_skeleton
full_rust_transpiler_claim=0
borrow_checker_claim=0
macro_expansion_claim=0
semantic_equivalence_claim=0
summary=ok
```

## Current AOT Acceptance Target

```text
python_reference_selftest=ok
hako_converter_mir_json_emit=ok
json_native_probe_mir_json_emit=ok
json_native_probe_exe=ok
hako_converter_exe=ok
hako_converter_parity=ok
file_input_enabled=0
schema_key_interning_bridge=temporary
summary=ok
```
