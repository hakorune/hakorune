# P381EL Same-Module Lookup Scope

Date: 2026-05-06
Scope: narrow target-function lookup in Stage0 same-module definition planning.

## Context

`plan_same_module_function_definition_edges(...)` looked up the target function
for every direct global-call route before deciding whether the route needed leaf
planning, same-module planning, or no planning.

Only the `generic_i64_or_leaf` owner branch needs the target function to decide
whether it is a numeric leaf. `leaf_i64` is already explicit in metadata, and
`module_generic` / `uniform_mir` routes only need selected-symbol planning.

## Change

Moved `same_module_function_find(view.target_symbol)` into the
`generic_i64_or_leaf` leaf-detection branch.

No accepted shape changed.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
cargo test -q mir::global_call_route_plan::tests::generic_i64
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/stage0_shape_inventory_guard.sh
git diff --check
```

## Result

The planner no longer performs target-function lookup for owner families that do
not need body-shape inspection.
