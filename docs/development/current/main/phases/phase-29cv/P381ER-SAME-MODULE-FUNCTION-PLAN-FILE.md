# P381ER Same-Module Function Plan File

Date: 2026-05-06
Scope: move the Stage0 transitive definition planner to an owner-neutral file name.

## Context

The transitive definition-set planner records direct global-call definition
edges for leaf-i64, generic-i64-or-leaf, module-generic, and uniform-MIR routes.
It no longer belongs under a string-specific file stem.

The old file name was:

```text
hako_llvmc_ffi_module_generic_string_plan.inc
```

## Change

Moved the planner to:

```text
hako_llvmc_ffi_same_module_function_plan.inc
```

Updated the Stage0 include site in `hako_llvmc_ffi_pure_compile.inc`.

No behavior changed. The public planner entry remains
`plan_same_module_function_definitions()` because the emitted definition set is
still the same-module function registry.

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

The owner-neutral planner is no longer physically grouped under the historical
module-generic string file stem.
