# P381FA MIR-Call Table-Driven Shims

Date: 2026-05-06
Scope: finish the current BoxShape cleanup pass for Stage0 MIR-call shim policy/emission seams.

## Context

After P381EW/P381EY/P381EZ, the MIR-call shim path already had shared
LoweringPlan tuple predicates and explicit rule-table owner names, but two
high-traffic seams still kept by-name ladders for the actual Stage0 consumer
logic:

- `lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc`
- `lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc`

That left the Stage0 path with duplicated if-chain structure even though the
route/source truth had already moved to MIR metadata.

## Change

Converted the remaining shim-local ladders into table-driven consumers:

1. `mir_call_need_policy.inc`
   - constructor/global/extern need classification now reads shared name-rule
     tables
   - route-metadata and core-method need classification now read explicit rule
     structs instead of open-coded branch ladders
2. `mir_call_shell.inc`
   - constructor emit and method-birth receiver validation now read surface rule
     tables
   - LoweringPlan extern emission now reads one extern emit rule table, while
     compatibility wrappers keep the existing `lowering_plan_*_view_is_valid(...)`
     surface for same-module consumers

No behavior changed. This is still a native transitional shim surface, but the
remaining owner logic is now explicit data rather than hand-expanded branch
ladders.

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

`mir_call_need_policy` and `mir_call_shell` now share the same table-driven
cleanup level as the surrounding LoweringPlan metadata helpers. The next
remaining BoxShape move is not another local ladder cleanup; it is to route the
remaining `mir_call` ownership toward the uniform MIR emitter.
