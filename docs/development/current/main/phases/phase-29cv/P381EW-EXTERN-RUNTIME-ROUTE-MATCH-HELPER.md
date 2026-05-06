# P381EW Extern Runtime Route Match Helper

Date: 2026-05-06
Scope: reduce duplicated Stage0 LoweringPlan extern-route validation.

## Context

Two MIR-call consumers were validating the same extern runtime route tuple:

- `hako_llvmc_ffi_mir_call_need_policy.inc`
- `hako_llvmc_ffi_mir_call_shell.inc`

Both checked route id, core op, route kind, emitted symbol, `runtime_call`,
`extern_registry` proof fields, and the cold runtime tier.

## Change

Added `lowering_plan_extern_call_view_matches_runtime_route(...)` to:

```text
hako_llvmc_ffi_lowering_plan_metadata.inc
```

Reused it from:

- MIR-call need classification
- MIR-call extern emission validation

Emission still validates return shape and value demand at the call surface,
because those checks are emission-specific.

No behavior changed.

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

The extern-route runtime predicate now has one owner in LoweringPlan metadata.
Need-policy and call-emission remain separate consumers, but no longer carry
their own copies of the same route/proof/tier validation.
