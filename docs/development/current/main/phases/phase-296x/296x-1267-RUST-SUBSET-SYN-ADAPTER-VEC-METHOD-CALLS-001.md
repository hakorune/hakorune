# RUST-SUBSET-SYN-ADAPTER-VEC-METHOD-CALLS-001

Date: 2026-06-19
Status: accepted
Scope: rust-subset-to-hako app-front source-shape support

## Decision

Accept Rust `Vec<T>` method calls through the existing `MethodCall` expression
shape.

No Vec-specific schema node is introduced. `vec![]` remains `ArrayLiteral`, and
receiver calls such as `xs.push(1)` remain ordinary `MethodCall` nodes. `.hako`
Array behavior is owned by the Hakorune compiler/runtime, not by the RustSubset
transport layer.

## Implementation

Added fixture:

```text
apps/rust-subset-to-hako/examples/vec_method_input.rs
apps/rust-subset-to-hako/examples/vec_method_subset.json
apps/rust-subset-to-hako/examples/vec_method_expected.hako
apps/rust-subset-to-hako/convert_vec_method_fixture.hako
```

No syn adapter code change is required beyond existing `Expr::MethodCall`
support. No converter core change is required.

## Evidence

```text
python3 apps/rust-subset-to-hako/convert.py \
  apps/rust-subset-to-hako/examples/vec_method_subset.json
```

Acceptance gate:

```bash
RUST_SUBSET_RUN_ADAPTER=1 RUST_SUBSET_RUN_REGRESSION=1 \
  bash apps/rust-subset-to-hako/smoke.sh
```

## Stop Lines

```text
do not add Vec-specific RustSubset schema nodes
do not special-case push by method name in converter_core.hako
do not claim compiler/runtime Array method semantics from this transport row
do not accept break/continue through this app-front row
```

## Report

```text
output_contract=rust-subset-syn-adapter-vec-method-calls-v0
selected_shape=vec_method_calls
schema_node_added=0
methodcall_schema_reused=1
syn_adapter_changed=0
python_converter_changed=0
hako_converter_changed=0
fixture_added=vec_method
compiler_recipe_acceptance_changed=0
summary=ok
```
