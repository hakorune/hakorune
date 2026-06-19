---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Add Rust `vec![...]` literal transport through RustSubset JSON, converters, syn adapter, and EXE/AOT fixture parity.
Related:
  - apps/rust-subset-to-hako/STATUS.md
  - apps/rust-subset-to-hako/schema/RustSubset-v0.md
  - apps/rust-subset-to-hako/convert.py
  - apps/rust-subset-to-hako/converter_core.hako
  - apps/rust-subset-to-hako/tools/syn_adapter/src/exprs.rs
  - apps/rust-subset-to-hako/tools/syn_adapter/src/types.rs
  - apps/rust-subset-to-hako/examples/vec_input.rs
  - apps/rust-subset-to-hako/examples/vec_subset.json
  - apps/rust-subset-to-hako/examples/vec_expected.hako
  - apps/rust-subset-to-hako/convert_vec_fixture.hako
---

# RUST-SUBSET-SYN-ADAPTER-VEC-LITERAL-001

## Decision

Select `Vec literal` as the next RustSubset source shape after `while`.

Rust:

```rust
let xs: Vec<i64> = vec![1, 2, 3];
```

RustSubset JSON v0:

```text
ArrayLiteral { elements }
```

`.hako` skeleton:

```hako
local xs: Array = [1, 2, 3]
```

## Boundary

This row is transport-only.

```text
rust_vec_macro_enabled=1
rust_array_expr_enabled=0
typed_array_semantics_owned_by_hakorune_compiler=1
packed_array_enabled=0
array_method_lowering_changed=0
```

The host adapter accepts `vec![...]` expression payloads that parse as a
comma-separated Rust expression list. Unsupported macro payloads become
`Unsupported` elements instead of being guessed.

## Implementation

```text
schema:
  Expression kind ArrayLiteral with elements

syn adapter:
  Expr::Macro with path vec -> ArrayLiteral
  Type::Path with angle args preserves Vec<i64> spelling

Python converter:
  ArrayLiteral -> [a, b]
  Vec<T> type spelling -> Array

.hako converter:
  ArrayLiteral -> [a, b]
  Vec<T> type spelling -> Array

fixture wrapper:
  convert_vec_fixture.hako uses the accepted FileBox open/read/close shape
```

## Verification

```text
python_selftest=ok
cargo_check_syn_adapter=ok
syn_adapter_vec_fixture_diff=empty
python_vec_converter_diff=empty
vec_fixture_exe_aot_parity=ok
full_adapter_smoke=ok
```

Command:

```bash
RUST_SUBSET_RUN_ADAPTER=1 RUST_SUBSET_RUN_REGRESSION=1 bash apps/rust-subset-to-hako/smoke.sh
```

## Stop Lines

```text
do not claim full Rust macro support
do not add PackedArray semantics in this row
do not change Hakorune compiler typed Array lowering in this row
do not special-case the vec fixture in converter_core.hako
```

## Contract

```text
output_contract=rust-subset-syn-adapter-vec-literal-v0

vec_macro_literal_enabled=1
array_literal_schema_enabled=1
syn_adapter_array_literal_enabled=1
python_converter_array_literal_enabled=1
hako_converter_array_literal_enabled=1
vec_type_maps_to_array=1
full_adapter_smoke_green=1

summary=ok
```
