# 293x-499 EXPRS-CHECK-001

Status: landed
Date: 2026-05-16

## Decision

`EXPRS-CHECK-001` is a small BoxShape cleanup for MIR builder expression
lowering. It moves CheckExpr lowering out of `src/mir/builder/exprs.rs` into a
dedicated builder owner.

## Scope

- Add `src/mir/builder/exprs_check.rs`.
- Move CheckExpr lowering only.
- Keep `exprs.rs` as the dispatcher/facade for the CheckExpr AST arm.

## Stop Lines

- Do not change parser/check-block syntax or acceptance.
- Do not change boolean coercion, Select semantics, or the current i64
  1/0 result convention.
- Do not touch collection literals, indexing, static-data lowering, allocator
  behavior, provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `CHK.1` | Add `exprs_check` owner module and wire it from `builder.rs`. | build sees the new module. | no behavior change |
| `CHK.2` | Move CheckExpr lowering. | focused check tests are green. | no Select/boolean changes |
| `CHK.3` | Verify and close out. | required evidence is green. | no adjacent refactor |

## Required Evidence

```text
cargo test -q c198_check_block_parses_default_route
cargo test -q c198_check_block_parses_token_cursor_route
bash tools/checks/k2_wide_check_block_surface_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```

## Closeout

This row closes when CheckExpr lowering has a dedicated builder owner and the
accepted check-block behavior is unchanged.

## Result

Landed:

- Added `src/mir/builder/exprs_check.rs`.
- Moved CheckExpr lowering out of `exprs.rs`.
- Updated the C198 check-block surface guard to observe the new CheckExpr owner.
- Kept `exprs.rs` as the AST dispatcher/facade.

No parser/check-block syntax, boolean coercion, Select semantics, i64 1/0 result
convention, collection literal lowering, indexing, static-data lowering,
allocator behavior, provider activation, hooks, host allocator replacement, or
`#[global_allocator]` behavior changed.

## Evidence

```text
cargo check -q
cargo test -q c198_check_block_parses_default_route
cargo test -q c198_check_block_parses_token_cursor_route
bash tools/checks/k2_wide_check_block_surface_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
