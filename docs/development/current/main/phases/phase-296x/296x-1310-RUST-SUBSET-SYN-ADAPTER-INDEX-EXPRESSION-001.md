# 296x-1310 RUST-SUBSET-SYN-ADAPTER-INDEX-EXPRESSION-001

Status: closed
Date: 2026-06-19

## Purpose

Add Rust indexing expression transport to the rust-subset-to-hako app front.

The accepted source shape is:

```rust
xs[i]
```

The RustSubset schema node is:

```json
{
  "kind": "Index",
  "target": {"kind": "Name", "name": "xs"},
  "index": {"kind": "Name", "name": "i"}
}
```

The emitted `.hako` skeleton is:

```hako
xs[i]
```

## Implementation

Updated:

```text
apps/rust-subset-to-hako/schema/RustSubset-v0.md
apps/rust-subset-to-hako/convert.py
apps/rust-subset-to-hako/converter_core.hako
apps/rust-subset-to-hako/tools/syn_adapter/src/exprs.rs
apps/rust-subset-to-hako/smoke.sh
```

Added fixture files:

```text
apps/rust-subset-to-hako/examples/index_input.rs
apps/rust-subset-to-hako/examples/index_subset.json
apps/rust-subset-to-hako/examples/index_expected.hako
apps/rust-subset-to-hako/convert_index_fixture.hako
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

This row only transports the expression shape through the external adapter,
RustSubset JSON, and `.hako` skeleton converter.

```text
array_storage_semantics_changed=0
bounds_semantics_changed=0
compiler_fastpath_changed=0
converter_core_input_route_changed=0
vm_product_route=retired
```

## Next

Continue app-front source-shape selection. Keep compiler Recipe/CorePlan work
separate unless a fixture exposes a real compiler acceptance blocker.

```text
next_blocker=RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-001
```
