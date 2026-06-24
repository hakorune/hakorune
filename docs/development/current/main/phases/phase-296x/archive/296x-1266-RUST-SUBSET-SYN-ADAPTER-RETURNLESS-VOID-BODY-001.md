# RUST-SUBSET-SYN-ADAPTER-RETURNLESS-VOID-BODY-001

Date: 2026-06-19
Status: accepted
Scope: rust-subset-to-hako app-front source-shape hardening

## Decision

Harden Rust functions with no explicit return type and no terminal `return`.

This is not a new RustSubset node. The existing representation is sufficient:

```text
return_type="void"
body=[ordinary statements]
```

The row adds fixture coverage so future converter or adapter changes do not
regress returnless void function bodies into empty-body output or implicit
return nodes.

## Implementation

Added fixture:

```text
apps/rust-subset-to-hako/examples/void_body_input.rs
apps/rust-subset-to-hako/examples/void_body_subset.json
apps/rust-subset-to-hako/examples/void_body_expected.hako
apps/rust-subset-to-hako/convert_void_body_fixture.hako
```

The syn adapter already emits statement bodies for default-return functions via
`return_type(&func.output) == "void"`. The Python and `.hako` converters already
emit `function ...: void` with ordinary statements, so no converter core change
is required.

## Evidence

```text
python3 apps/rust-subset-to-hako/convert.py \
  apps/rust-subset-to-hako/examples/void_body_subset.json
```

Acceptance gate:

```bash
RUST_SUBSET_RUN_ADAPTER=1 RUST_SUBSET_RUN_REGRESSION=1 \
  bash apps/rust-subset-to-hako/smoke.sh
```

## Stop Lines

```text
do not add a new statement kind for implicit void returns
do not inject synthetic return statements
do not change converter_core.hako for this hardening row
do not mix compiler Recipe/CorePlan acceptance with source-shape transport
```

## Report

```text
output_contract=rust-subset-syn-adapter-returnless-void-body-v0
selected_shape=returnless_void_body
schema_node_added=0
return_type_void_contract=1
implicit_return_node_added=0
syn_adapter_changed=0
python_converter_changed=0
hako_converter_changed=0
fixture_added=void_body
compiler_recipe_acceptance_changed=0
summary=ok
```
