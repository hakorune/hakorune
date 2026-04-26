# Normalized Shadow Entry

This subtree owns narrow route-entry facades for normalized-shadow lowering.

## Boundaries

- Entry modules expose route intent, not storage history.
- Do not perform route priority selection here; `builder.rs` owns route order.
- Do not add new StepTree shape acceptance here.
- Keep implementation local to the entry owner unless a separate support module
  has a clearer semantic owner.

## Current Entries

- `if_only`: baseline if-only normalized-shadow entry for the Phase 123-128
  scope.
