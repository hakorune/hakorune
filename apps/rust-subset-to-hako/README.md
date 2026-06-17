# rust-subset-to-hako

Status: design-only handoff

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

## Suggested First Implementation

Implement only the `.hako` converter over `examples/simple_subset.json`.

```text
input:  RustSubset JSON v0
output: Hako skeleton text
```

Do not implement the external Rust parser adapter in the first slice. If an
adapter is needed later, use Rust `syn` or `tree-sitter-rust` outside the
Hakorune app and keep it as a replaceable producer of the same JSON schema.

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
