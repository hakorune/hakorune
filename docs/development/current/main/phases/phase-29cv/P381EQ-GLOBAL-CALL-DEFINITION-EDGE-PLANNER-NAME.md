# P381EQ Global-Call Definition Edge Planner Name

Date: 2026-05-06
Scope: clarify Stage0 definition-set planner naming.

## Context

`plan_same_module_function_definition_edges(...)` started as a same-module
function planner. It now scans a function's MIR LoweringPlan global-call routes
and records every direct definition edge that must be available before emit:

- `leaf_i64`
- `generic_i64_or_leaf`
- `module_generic`
- `uniform_mir`

The old name hid the leaf-definition branch and made the planner look narrower
than the route metadata contract.

## Change

Renamed the helper to `plan_function_global_call_definition_edges(...)`.

No behavior changed. The function still:

- records leaf-i64 definitions in the leaf registry
- resolves `generic_i64_or_leaf` targets to either leaf or same-module function
  definitions
- records module-generic and uniform-MIR targets in the same-module function
  registry
- repeats the scan transitively for planned same-module functions

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
cargo test -q mir::global_call_route_plan::tests::void_sentinel
cargo test -q runner::mir_json_emit::tests::global_call_routes::void_sentinel
cargo test -q mir::global_call_route_plan::tests::void_logging
bash tools/checks/stage0_shape_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

The Stage0 transitive definition-set planner now uses a name that matches the
global-call definition edge contract instead of implying same-module-only work.
