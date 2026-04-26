# Normalized Shadow Support

This subtree owns small semantic facades shared by normalized-shadow route
lowerers.

## Boundaries

- No route selection.
- No StepTree shape acceptance.
- No fixture-name or by-name heuristics.
- Keep fail-fast tags owned by the underlying lowering contract.

## Current Facades

- `expr_lowering`: shared assignment and minimal-compare lowering used by route
  lowerers.

`expr_lowering` owns the shared assignment/minimal-compare implementation.
Route lowerers and the legacy entry path should depend on this support path
instead of defining helper behavior inside `legacy`.
