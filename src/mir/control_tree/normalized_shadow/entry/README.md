# Normalized Shadow Entry

This subtree owns narrow route-entry facades for normalized-shadow lowering.

## Boundaries

- Entry modules expose route intent, not storage history.
- Do not perform route priority selection here; `builder.rs` owns route order.
- Do not add new StepTree shape acceptance here.
- Implementation may delegate to quarantined compatibility modules while a
  physical move is in progress.

## Current Entries

- `if_only`: baseline if-only normalized-shadow entry for the Phase 123-128
  scope.
