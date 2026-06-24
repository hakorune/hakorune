# RUST-SUBSET-SYN-ADAPTER-EXPLICIT-UNIT-RETURN-001

Date: 2026-06-19
Status: accepted
Scope: rust-subset-to-hako app-front source-shape hardening

## Decision

Fixture-guard Rust functions with explicit unit return type `-> ()` and no
terminal `return`.

No new RustSubset node is introduced. The existing type mapping already lowers
Rust `()` to RustSubset `"void"`, so this row fixes the contract with a fixture
instead of adding more adapter code.

```text
Rust:       fn f(x: i64) -> () { body }
RustSubset: {"kind":"Function","return_type":"void","body":[...]}
.hako:      function f(x: i64): void { body }
```

## Implementation

Added fixture:

```text
apps/rust-subset-to-hako/examples/unit_return_input.rs
apps/rust-subset-to-hako/examples/unit_return_subset.json
apps/rust-subset-to-hako/examples/unit_return_expected.hako
apps/rust-subset-to-hako/convert_unit_return_fixture.hako
```

Adapter code change:

```text
none
```

The existing implementation remains the owner:

```text
Type::Tuple(empty) -> "void"
ReturnType::Type(_, ty) -> type_name(ty)
```

## Evidence

```bash
cargo run --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml --quiet -- \
  apps/rust-subset-to-hako/examples/unit_return_input.rs \
  --module unit_return_fixture \
  -o apps/rust-subset-to-hako/examples/unit_return_subset.json

python3 apps/rust-subset-to-hako/convert.py \
  apps/rust-subset-to-hako/examples/unit_return_subset.json \
  | diff -u apps/rust-subset-to-hako/examples/unit_return_expected.hako -
```

Acceptance gate:

```bash
RUST_SUBSET_RUN_ADAPTER=1 RUST_SUBSET_RUN_REGRESSION=1 \
  bash apps/rust-subset-to-hako/smoke.sh
```

## Stop Lines

```text
do not add a UnitReturn schema node
do not synthesize return statements
do not change converter core logic for ordinary void body emission
do not mix break/continue or compiler Recipe/CorePlan acceptance into this row
```

## Report

```text
output_contract=rust-subset-syn-adapter-explicit-unit-return-v0
selected_shape=returnless_typed_unit_function
schema_node_added=0
adapter_code_changed=0
unit_return_maps_to_void=1
synthetic_return_added=0
fixture_added=unit_return
compiler_recipe_acceptance_changed=0
summary=ok
```
