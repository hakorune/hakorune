# 293x-495 EXPRS-INDEXING-001

Status: landed
Date: 2026-05-16

## Decision

`EXPRS-INDEXING-001` is a BoxShape cleanup for MIR builder expression lowering.
It moves indexing-specific lowering out of `src/mir/builder/exprs.rs` into a
dedicated builder owner.

## Scope

- Add `src/mir/builder/indexing.rs`.
- Move index target class inference out of `exprs.rs`.
- Move static-data index load lowering out of `exprs.rs`.
- Move ArrayBox/MapBox get/set index lowering out of `exprs.rs`.
- Keep `exprs.rs` as the dispatcher/facade for the AST index expression arm.

## Stop Lines

- Do not accept new indexable classes.
- Do not change static table support beyond the existing `u16` element load.
- Do not change ArrayBox/MapBox route semantics, receiver/result value shape, or
  error text/tags.
- Do not touch parser syntax, collection literal lowering, CheckExpr lowering,
  allocator behavior, provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `IDX.1` | Add the `indexing` owner module and wire it from `builder/mod.rs`. | build sees the new module. | no behavior change |
| `IDX.2` | Move index target inference and formatting helpers. | helper tests remain green. | no accepted class changes |
| `IDX.3` | Move static-data and ArrayBox/MapBox index lowering. | focused index guards are green. | no route/error changes |
| `IDX.4` | Verify and close out. | required evidence is green. | no adjacent refactor |

## Required Evidence

```text
bash tools/checks/k2_wide_static_const_table_load_guard.sh
cargo test -q static_const_table_load
cargo test -q array_value_get_uses_unified_receiver_arg_shape_and_element_return
cargo test -q map_value_get_existing_key_uses_unified_receiver_arg_shape_and_stored_value_return
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```

## Closeout

This row closes when indexing-specific logic has a dedicated builder owner and
the accepted expression/indexing behavior is unchanged.

## Result

Landed:

- Added `src/mir/builder/indexing.rs`.
- Moved index target class inference and index target formatting helpers out of
  `exprs.rs`.
- Moved static-data index load lowering out of `exprs.rs`.
- Moved ArrayBox/MapBox index get/set lowering out of `exprs.rs`.
- Kept `exprs.rs` as the AST dispatcher/facade.

No accepted indexable classes, static table support, ArrayBox/MapBox route
semantics, receiver/result value shape, parser syntax, allocator behavior,
provider activation, hooks, host allocator replacement, or `#[global_allocator]`
behavior changed.

## Evidence

```text
cargo check -q
bash tools/checks/k2_wide_static_const_table_load_guard.sh
cargo test -q static_const_table_load
cargo test -q array_value_get_uses_unified_receiver_arg_shape_and_element_return
cargo test -q map_value_get_existing_key_uses_unified_receiver_arg_shape_and_stored_value_return
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```
