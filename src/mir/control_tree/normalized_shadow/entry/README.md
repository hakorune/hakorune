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

## Fossil Boundary

`if_only` is a historical Phase 123-128 baseline route. It runs only after the
newer normalized-shadow routes in `builder.rs` decline:

1. `PostIfPostKBuilderBox`
2. `IfAsLastJoinKLowererBox`
3. `if_only`

Do not silently "fix" the baseline shortcuts in this entry:

- compare LHS placeholder emission
- simplified then-branch emission
- Phase 123-128 decline tags

If this baseline is replaced, do it as a dedicated route replacement card with
updated archived-smoke expectations. New StepTree shapes should get a new route
before this baseline instead of widening `if_only`.
