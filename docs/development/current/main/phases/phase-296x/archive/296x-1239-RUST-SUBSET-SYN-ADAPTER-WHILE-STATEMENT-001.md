---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Add RustSubset `While` statement support through schema, converters, syn adapter, and EXE/AOT fixture parity.
Related:
  - apps/rust-subset-to-hako/STATUS.md
  - apps/rust-subset-to-hako/schema/RustSubset-v0.md
  - apps/rust-subset-to-hako/convert.py
  - apps/rust-subset-to-hako/converter_core.hako
  - apps/rust-subset-to-hako/tools/syn_adapter/src/stmts.rs
  - apps/rust-subset-to-hako/examples/while_input.rs
  - apps/rust-subset-to-hako/examples/while_subset.json
  - apps/rust-subset-to-hako/examples/while_expected.hako
  - apps/rust-subset-to-hako/convert_while_fixture.hako
---

# RUST-SUBSET-SYN-ADAPTER-WHILE-STATEMENT-001

## Decision

Select `while` as the next RustSubset source shape after Assign.

Rust:

```rust
while i < limit {
    sum = sum + i;
    i = i + 1;
}
```

RustSubset JSON v0:

```text
While { cond, body }
```

`.hako` skeleton:

```hako
loop(i < limit) {
    sum = sum + i
    i = i + 1
}
```

## Boundary

This row accepts only plain Rust `while cond { body }`.

```text
break_enabled=0
continue_enabled=0
else_if_enabled=0
loop_expr_enabled=0
```

`break` / `continue` remain in the compiler backlog:

```text
COREPLAN-LOOP-BREAK-MULTI-STAGE-RECIPE-ACCEPTANCE-001
```

## Implementation

```text
schema:
  Statement kind While with cond/body

syn adapter:
  Expr::While -> While JSON

Python converter:
  While -> loop(cond) { body }

.hako converter:
  While -> loop(cond) { body }

fixture wrapper:
  convert_while_fixture.hako uses the same FileBox open/read/close shape as the
  existing if/assign fixture wrappers.
```

## Verification

```text
python_selftest=ok
cargo_check_syn_adapter=ok
syn_adapter_while_fixture_diff=empty
python_while_converter_diff=empty
while_fixture_exe_aot_parity=ok
full_adapter_smoke=ok
```

Command:

```bash
RUST_SUBSET_RUN_ADAPTER=1 RUST_SUBSET_RUN_REGRESSION=1 bash apps/rust-subset-to-hako/smoke.sh
```

## Stop Lines

```text
do not use Rust `while` to claim compiler break/continue acceptance
do not add Vec literal in this row
do not special-case the while fixture in converter_core.hako
do not use static FileBox.read in the fixture wrapper; keep the accepted FileBox open/read/close route
```

## Contract

```text
output_contract=rust-subset-syn-adapter-while-statement-v0

while_statement_schema_enabled=1
syn_adapter_while_enabled=1
python_converter_while_enabled=1
hako_converter_while_enabled=1
while_maps_to_hako_loop=1
break_continue_enabled=0
full_adapter_smoke_green=1

summary=ok
```
