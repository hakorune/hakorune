# P381EM Global-Call Readiness Helper

Date: 2026-05-06
Scope: centralize direct global-call definition readiness checks in the Stage0 emission shell.

## Context

`emit_global_call_lowering_plan_mir_call(...)` validated the call route, result
register, arity, definition readiness, argument materialization, and final call
emission in one block.

The readiness ladder is an owner-family decision:

- `leaf_i64` must already have emitted the leaf symbol
- `generic_i64_or_leaf` can target either a planned same-module function or an
  emitted leaf symbol
- `module_generic` / `uniform_mir` routes must target a planned same-module
  function

## Change

Extracted the readiness ladder into
`global_call_definition_target_is_ready(...)`.

No behavior changed.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
cargo test -q mir::global_call_route_plan
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/stage0_shape_inventory_guard.sh
git diff --check
```

## Result

Direct global-call emission now has a single local readiness helper before the
argument materialization and call emission steps.
