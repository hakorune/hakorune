# 293x-498 EXPRS-COLLECTION-LITERAL-002 Post-Collection-Literal Row Selection

Status: landed
Date: 2026-05-16

## Decision

`EXPRS-COLLECTION-LITERAL-001` closed the collection literal owner split.

Select exactly one next cleanup row:

```text
EXPRS-CHECK-001:
  move CheckExpr lowering out of exprs.rs into a dedicated builder check owner
```

## Why This Row

`EXPRS-CHECK-001` is small but cleanly completes the current expression-dispatcher
cleanup burst. It has a narrow owner and focused guards, and it does not change
accepted parser/check-block behavior.

## Selected Row

```text
row:
  EXPRS-CHECK-001
owner:
  new src/mir/builder/exprs_check.rs
scope:
  move CheckExpr lowering only
stop_line:
  no boolean coercion changes
  no parser/check-block surface changes
  no Select semantics changes
  no collection/indexing behavior
evidence:
  cargo test -q c198_check_block_parses_default_route
  cargo test -q c198_check_block_parses_token_cursor_route
  bash tools/checks/k2_wide_check_block_surface_guard.sh
  bash tools/checks/current_state_pointer_guard.sh
  tools/checks/dev_gate.sh quick
```

## Stop Lines

- Do not change parser/check-block syntax or acceptance.
- Do not change boolean coercion, Select semantics, or check-block result
  convention.
- Do not touch collection literals, indexing, static-data loads, allocator
  behavior, provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`.

## Closeout

This row closes when `EXPRS-CHECK-001` has a selected current card with owner,
scope, stop lines, and evidence.
