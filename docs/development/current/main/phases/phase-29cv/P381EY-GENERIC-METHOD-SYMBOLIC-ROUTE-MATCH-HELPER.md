# P381EY Generic-Method Symbolic Route Match Helper

Date: 2026-05-06
Scope: reduce duplicated Stage0 LoweringPlan generic-method route validation.

## Context

`hako_llvmc_ffi_mir_call_need_policy.inc` classified runtime declaration needs
from generic-method LoweringPlan entries by checking:

- route id
- core op
- route kind
- tier
- emitted helper symbol
- route proof

The route tuple portion already had a LoweringPlan metadata helper, but the
symbol/proof portion was still local to the need policy.

## Change

Added `lowering_plan_generic_method_view_matches_symbolic_route(...)` to:

```text
hako_llvmc_ffi_lowering_plan_metadata.inc
```

Reused it from MIR-call need classification.

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

Generic-method route tuple plus symbol/proof validation is now owned by the
LoweringPlan metadata view instead of being reconstructed inside MIR-call need
classification.
