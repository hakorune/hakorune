# 296x-1312 RUST-SUBSET-SYN-ADAPTER-GENERIC-FUNCTION-SKELETON-001

Status: closed
Date: 2026-06-19

## Purpose

Fixture-guard generic Rust function skeleton transport.

This row preserves type parameter spellings in RustSubset JSON and emitted
`.hako` skeleton output. It does not add a type-parameter model or generic
semantics.

## Accepted Shape

Input:

```rust
fn identity<T>(value: T) -> T {
    value
}
```

RustSubset:

```json
{
  "kind": "Function",
  "name": "identity",
  "params": [{"name": "value", "type": "T"}],
  "return_type": "T"
}
```

Emitted `.hako` skeleton:

```hako
function identity(value: T): T {
    return value
}
```

## Implementation

Added fixture files:

```text
apps/rust-subset-to-hako/examples/generic_function_input.rs
apps/rust-subset-to-hako/examples/generic_function_subset.json
apps/rust-subset-to-hako/examples/generic_function_expected.hako
apps/rust-subset-to-hako/convert_generic_function_fixture.hako
```

Updated:

```text
apps/rust-subset-to-hako/selftest.py
apps/rust-subset-to-hako/smoke.sh
apps/rust-subset-to-hako/README.md
apps/rust-subset-to-hako/STATUS.md
```

## Evidence

```bash
python3 apps/rust-subset-to-hako/selftest.py
cargo check -q --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml
cargo check -q --lib
bash apps/rust-subset-to-hako/smoke.sh
bash apps/rust-subset-to-hako/smoke_adapter.sh
```

Observed result:

```text
summary=ok
```

## Boundary

```text
type_parameter_model_enabled=0
generic_semantics_claim=0
converter_core_input_route_changed=0
vm_product_route=retired
```

## Next

Continue app-front source-shape selection. Treat richer generic semantics as a
future language/design task, not as part of this skeleton transport row.

```text
next_blocker=RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-001
```
