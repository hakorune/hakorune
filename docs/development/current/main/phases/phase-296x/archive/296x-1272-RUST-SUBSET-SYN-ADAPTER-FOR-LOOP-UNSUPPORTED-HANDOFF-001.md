# RUST-SUBSET-SYN-ADAPTER-FOR-LOOP-UNSUPPORTED-HANDOFF-001

Date: 2026-06-19
Status: accepted
Scope: rust-subset-to-hako app-front source-shape handoff

## Decision

Accept Rust `for` loops only as an explicit unsupported handoff.

No RustSubset `ForLoop` node is introduced, and no iterator semantics are
implemented. The syn adapter emits:

```json
{"kind":"Unsupported","reason":"Rust for loop expression is out of v0 scope"}
```

`for` statements are emitted as ordinary expression statements containing that
Unsupported expression, not as tail-expression returns. This avoids turning a
source-shape handoff into `return /* TODO */`.

## Implementation

Added:

```text
Expr::ForLoop -> Unsupported("Rust for loop expression is out of v0 scope")
Stmt::Expr(Expr::ForLoop, _) -> Expr(Unsupported(...))
```

Added fixture:

```text
apps/rust-subset-to-hako/examples/for_loop_unsupported_input.rs
apps/rust-subset-to-hako/examples/for_loop_unsupported_subset.json
apps/rust-subset-to-hako/examples/for_loop_unsupported_expected.hako
apps/rust-subset-to-hako/convert_for_loop_unsupported_fixture.hako
```

## Evidence

```bash
cargo run --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml --quiet -- \
  apps/rust-subset-to-hako/examples/for_loop_unsupported_input.rs \
  --module for_loop_unsupported_fixture \
  -o apps/rust-subset-to-hako/examples/for_loop_unsupported_subset.json

python3 apps/rust-subset-to-hako/convert.py \
  apps/rust-subset-to-hako/examples/for_loop_unsupported_subset.json \
  | diff -u apps/rust-subset-to-hako/examples/for_loop_unsupported_expected.hako -
```

Acceptance gate:

```bash
RUST_SUBSET_RUN_ADAPTER=1 RUST_SUBSET_RUN_REGRESSION=1 \
  bash apps/rust-subset-to-hako/smoke.sh
```

## Stop Lines

```text
do not implement iterator semantics
do not add a RustSubset ForLoop node
do not desugar for-loops into while loops
do not accept break/continue through this app-front row
do not mix compiler Recipe/CorePlan acceptance with source-shape handoff
```

## Report

```text
output_contract=rust-subset-syn-adapter-for-loop-unsupported-handoff-v0
selected_shape=for_loop_unsupported_handoff
schema_node_added=0
iterator_semantics_enabled=0
while_desugar_enabled=0
tail_return_for_unsupported_for_loop=0
unsupported_reason_stable=1
fixture_added=for_loop_unsupported
compiler_recipe_acceptance_changed=0
summary=ok
```
