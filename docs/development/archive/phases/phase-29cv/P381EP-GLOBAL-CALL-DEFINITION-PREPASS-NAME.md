# P381EP Global-Call Definition Prepass Name

Date: 2026-05-06
Scope: clarify Stage0 same-module emitter prepass naming.

## Context

`same_module_function_prepass_global_call_definition_view(...)` no longer only
covered same-module function definitions. The helper now accepts the global-call
definition facts owned by MIR metadata:

- `module_generic`
- `uniform_mir`
- `generic_i64_or_leaf`
- `leaf_i64`

The old name made the prepass look narrower than its current contract.

## Change

Renamed the helper to `global_call_definition_prepass_view(...)`.

No behavior changed. The helper still:

- publishes `T_I64`
- propagates MIR-owned result origin for same-module definitions
- accepts direct leaf-i64 definitions

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

The Stage0 prepass helper name now matches the owner-neutral global-call
definition contract used after the capsule exits.
