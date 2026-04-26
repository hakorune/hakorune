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

`expr_lowering` currently delegates to the legacy entry lowerer while the
implementation is being split. Route lowerers should depend on this support
path instead of importing `legacy`.
