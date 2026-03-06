# Loop Routes - JoinIR Frontend

## Responsibility

`loop_routes/` lowers loop-shaped Program JSON into JoinIR after
`LoopFrontendBinding` has already chosen a `LoopRoute`.

This layer does three things only:
- accept `LoopRoute`
- inspect JSON v0 loop/body structure
- emit JoinIR entry / loop_step / k_exit functions

## Non-goals

- function-name dispatch
  - handled by `loop_frontend_binding.rs`
- Box or method-name semantics
  - handled downstream in Bridge / VM
- legacy fixture-key compatibility
  - handled by `route.rs`

## Module Layout

- `mod.rs`
  - `LoopRoute`, `LoweringError`, thin dispatch
- `common.rs`
  - shared parse/context/module builders
- `simple.rs`
  - generic loop route
- `filter.rs`
  - filter route shim; currently delegates to `simple`
- `print_tokens.rs`
  - print-tokens route shim; currently delegates to `simple`
- `break_route.rs`
  - break route
- `continue_route.rs`
  - continue route
- `continue_return_route.rs`
  - continue+early-return route
- `param_guess.rs`
  - legacy break-route parameter heuristics
- `step_calculator.rs`
  - linear step-delta helper for continue-family routes

## Contract

- `LoopFrontendBinding` decides the route.
- `loop_routes/` does not re-dispatch by function name.
- Unsupported routes return `Err(LoweringError::UnimplementedRoute { .. })`.
- Shared helpers stay in `common.rs`; route-specific logic stays in each route module.

## Extension Rule

When adding a new loop route:
1. add a `LoopRoute` variant
2. add one lowering module
3. wire one match arm in `lower_loop_with_route`
4. keep route-specific semantics out of `common.rs`

## Cleanup Note

This directory replaced the old numbered loop-lowering surface.
Any remaining legacy `pattern*` tokens should be treated as historical notes or fixture labels,
not as architecture vocabulary for the active frontend path.
