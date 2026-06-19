# RUST-SUBSET-SYN-ADAPTER-ELSE-IF-STATEMENT-001

Date: 2026-06-19
Status: accepted
Scope: rust-subset-to-hako app-front source-shape support

## Decision

Accept Rust `else if` as recursive RustSubset `If`.

No new schema node is introduced. A Rust `else if` lowers to a parent `If`
whose `else` array contains exactly one nested `If` statement. The Python and
`.hako` converters already emit nested `If` statements, so this row only changes
the syn adapter selection and fixture coverage.

## Implementation

```text
syn Expr::If else_branch:
  Expr::Block -> existing else statement list
  Expr::If    -> vec![nested If JSON]
  other       -> Unsupported
```

Added fixture:

```text
apps/rust-subset-to-hako/examples/else_if_input.rs
apps/rust-subset-to-hako/examples/else_if_subset.json
apps/rust-subset-to-hako/examples/else_if_expected.hako
apps/rust-subset-to-hako/convert_else_if_fixture.hako
```

## Evidence

```text
python3 apps/rust-subset-to-hako/convert.py \
  apps/rust-subset-to-hako/examples/else_if_subset.json
```

Acceptance gate:

```bash
RUST_SUBSET_RUN_ADAPTER=1 RUST_SUBSET_RUN_REGRESSION=1 \
  bash apps/rust-subset-to-hako/smoke.sh
```

## Stop Lines

```text
do not add an ElseIf schema node
do not desugar into source text before RustSubset JSON
do not accept break/continue through this app-front row
do not mix compiler Recipe/CorePlan acceptance with source-shape transport
```

## Report

```text
output_contract=rust-subset-syn-adapter-else-if-statement-v0
selected_shape=else_if_statement
schema_node_added=0
recursive_if_schema_used=1
syn_adapter_else_if_supported=1
python_converter_changed=0
hako_converter_changed=0
fixture_added=else_if
compiler_recipe_acceptance_changed=0
summary=ok
```
