# P381EZ Generic-Method Need Rule Names

Date: 2026-05-06
Scope: clarify Stage0 MIR-call need policy helper ownership.

## Context

The MIR-call need policy has separate LoweringPlan paths for:

- generic-method routes
- extern-call routes

After P381EY, the generic-method route validation moved into LoweringPlan
metadata. The remaining generic-method rule table still used broad
`LoweringPlanNeedRule` names.

## Change

Renamed the generic-method need-rule surface to:

```text
LoweringPlanGenericMethodNeedRule
lowering_plan_generic_method_need_rule_matches(...)
mir_call_generic_method_need_kind_from_lowering_plan_entry(...)
```

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

The generic-method and extern-call need-policy paths now name their rule tables
symmetrically and explicitly.
