---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Select and close the first syn-adapter coverage hardening slice.
Related:
  - apps/rust-subset-to-hako/examples/simple_input.rs
  - apps/rust-subset-to-hako/examples/simple_subset.json
  - apps/rust-subset-to-hako/tools/syn_adapter
  - apps/rust-subset-to-hako/smoke.sh
---

# RUST-SUBSET-SYN-ADAPTER-COVERAGE-SELECTION-001

## Decision

The first coverage hardening target is Rust tail-expression return lowering.

```text
selected_shape=tail_expression_return
front=apps/rust-subset-to-hako/examples/simple_input.rs
expected_schema=apps/rust-subset-to-hako/examples/simple_subset.json
```

Rust functions may return their final expression without an explicit `return`.
The adapter must encode that final expression as a RustSubset `Return`, not as
an `Expr` statement.

## Result

```text
tail_expr_return_normalized=1
free_function_receiver_none_omitted=1
simple_input_semantic_parity=ok
adapter_fixture_strict_json_diff=ok
```

The optional adapter smoke now covers:

```text
RUST_SUBSET_RUN_ADAPTER=1 bash apps/rust-subset-to-hako/smoke.sh
```

## Stop Lines

```text
do not infer Rust semantics beyond the selected v0 shapes
do not change converter_core.hako for adapter output quirks
do not make simple_input formatting the schema truth
do not silently drop unsupported Rust constructs
```

## Contract

```text
output_contract=rust-subset-syn-adapter-coverage-selection-v0

tail_expression_return=ok
simple_input_semantic_parity=ok
adapter_fixture_json_diff=ok
converter_core_changed=0

summary=ok
```
