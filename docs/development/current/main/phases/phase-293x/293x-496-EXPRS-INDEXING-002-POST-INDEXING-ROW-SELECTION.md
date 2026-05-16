# 293x-496 EXPRS-INDEXING-002 Post-Indexing Row Selection

Status: landed
Date: 2026-05-16

## Decision

`EXPRS-INDEXING-001` closed the indexing owner split.

Select exactly one next cleanup row:

```text
EXPRS-COLLECTION-LITERAL-001:
  move ArrayLiteral / MapLiteral lowering out of exprs.rs into a dedicated
  builder collection-literals owner
```

## Why This Row

This is the next-lowest-risk `exprs.rs` cleanup from the worker inventory.
It removes another coherent expression-lowering family while preserving the
existing ArrayBox/MapBox route behavior.

## Selected Row

```text
row:
  EXPRS-COLLECTION-LITERAL-001
owner:
  new src/mir/builder/collection_literals.rs
scope:
  move ArrayLiteral / MapLiteral lowering out of exprs.rs
stop_line:
  no ArrayBox / MapBox route changes
  preserve array element inference and type/origin registry writes
  no static-data/indexing behavior
  no backend/provider behavior
evidence:
  cargo test -q array_value_get_uses_unified_receiver_arg_shape_and_element_return
  cargo test -q map_value_set_uses_unified_receiver_arg_shape_and_receipt_string_return
  bash tools/checks/current_state_pointer_guard.sh
  tools/checks/dev_gate.sh quick
```

## Stop Lines

- Do not change ArrayBox/MapBox methods, route certainty, or effect masks.
- Do not change array element inference or type/origin registry writes.
- Do not touch indexing, static-data loads, CheckExpr, parser syntax, allocator
  behavior, provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`.

## Closeout

This row closes when `EXPRS-COLLECTION-LITERAL-001` has a selected current card
with owner, scope, stop lines, and evidence.
