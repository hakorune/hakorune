---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Split the syn-based RustSubset host adapter into responsibility-focused modules without changing emitted JSON.
Related:
  - apps/rust-subset-to-hako/STATUS.md
  - apps/rust-subset-to-hako/tools/syn_adapter/src/main.rs
  - apps/rust-subset-to-hako/tools/syn_adapter/src/cli.rs
  - apps/rust-subset-to-hako/tools/syn_adapter/src/items.rs
  - apps/rust-subset-to-hako/tools/syn_adapter/src/functions.rs
  - apps/rust-subset-to-hako/tools/syn_adapter/src/stmts.rs
  - apps/rust-subset-to-hako/tools/syn_adapter/src/exprs.rs
  - apps/rust-subset-to-hako/tools/syn_adapter/src/types.rs
---

# RUST-SUBSET-SYN-ADAPTER-MODULE-SPLIT-001

## Decision

Split `tools/syn_adapter/src/main.rs` before adding larger source-shape support.

The adapter now uses these modules:

```text
cli:
  command-line parsing, diagnostics, output writing

items:
  Rust file/item lowering to RustSubset JSON

functions:
  function and impl-method lowering

stmts:
  statement/block lowering

exprs:
  expression lowering

types:
  shared Rust type/pattern/member/item vocabulary helpers
```

`main.rs` is a thin orchestration entry:

```text
parse args -> read source -> syn parse -> file_to_json -> write output
```

## Why

The previous single file mixed CLI, type helpers, expression lowering,
statement lowering, item lowering, and JSON output. That was still small enough
to work, but it would have made the next shape (`while`, `loop`, Vec literal, or
other control/data forms) harder to add without coupling.

This row is BoxShape-only:

```text
behavior_changed=0
new_source_shape_enabled=0
schema_changed=0
converter_core_changed=0
```

## Verification

```text
cargo_check_syn_adapter=ok
assign_fixture_json_sha_before=9c0b0cca2307f19e3c54d212cfa7f085e3076016ce94a7db8e1be48e33af9e22
assign_fixture_json_sha_after=9c0b0cca2307f19e3c54d212cfa7f085e3076016ce94a7db8e1be48e33af9e22
assign_fixture_json_diff=empty
rust_subset_full_adapter_smoke=ok
```

Commands:

```bash
cargo check --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml
cargo run --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml --quiet -- apps/rust-subset-to-hako/examples/assign_input.rs --module assign_fixture -o /tmp/after_assign.json
diff -u /tmp/before_assign.json /tmp/after_assign.json
RUST_SUBSET_RUN_ADAPTER=1 RUST_SUBSET_RUN_REGRESSION=1 bash apps/rust-subset-to-hako/smoke.sh
```

## Stop Lines

```text
do not add while/loop/Vec support in this row
do not change RustSubset JSON v0 schema in this row
do not move converter_core.hako logic in this row
do not mix compiler Recipe/CorePlan loop/break acceptance into this app-front split
```

## Contract

```text
output_contract=rust-subset-syn-adapter-module-split-v0

module_split_enabled=1
module_count=6
main_rs_facade_only=1
behavior_changed=0
new_source_shape_enabled=0
assign_fixture_json_diff=empty
full_adapter_smoke_green=1
current_app_front_blocker_unchanged=RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-003

summary=ok
```
