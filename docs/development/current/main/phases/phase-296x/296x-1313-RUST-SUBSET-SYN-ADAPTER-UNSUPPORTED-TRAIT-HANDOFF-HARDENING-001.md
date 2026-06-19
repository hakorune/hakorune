# 296x-1313 RUST-SUBSET-SYN-ADAPTER-UNSUPPORTED-TRAIT-HANDOFF-HARDENING-001

Status: closed
Date: 2026-06-19

## Purpose

Harden the existing unsupported Rust trait item handoff with checked-in
RustSubset JSON and `.hako` EXE/AOT fixture parity.

This row does not add trait semantics. It only strengthens the existing
Unsupported handoff contract.

## Accepted Handoff

Input:

```rust
trait Drawable {
    fn draw(&self);
}
```

RustSubset:

```json
{
  "kind": "Unsupported",
  "rust_kind": "Trait",
  "summary": "Trait items are out of v0 scope"
}
```

Emitted `.hako` skeleton:

```hako
// TODO: Trait items are out of v0 scope
```

## Implementation

Added:

```text
apps/rust-subset-to-hako/examples/unsupported_trait_subset.json
apps/rust-subset-to-hako/convert_unsupported_trait_fixture.hako
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
trait_semantics_enabled=0
schema_node_added=0
converter_core_input_route_changed=0
vm_product_route=retired
```

## Next

Continue app-front source-shape selection. Trait semantics remain out of v0
scope until a design row explicitly accepts them.

```text
next_blocker=RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-001
```
