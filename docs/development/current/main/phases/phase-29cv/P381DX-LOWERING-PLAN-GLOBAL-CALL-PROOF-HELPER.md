# P381DX LoweringPlan Global-Call Proof Helper

Date: 2026-05-06
Scope: centralize Stage0 typed global-call proof validation in LoweringPlan metadata.

## Context

P381DW moved the Phase 1 direct-only retired capsules to
`definition_owner=uniform_mir`. After that owner cleanup, the C metadata reader
still carried two copies of the typed global-call proof allow-list:

- one for raw lowering-plan entries
- one for parsed global-call views

That duplicated the same contract and made future proof retirement/error
diagnostics harder to audit.

## Change

Added `lowering_plan_proof_is_typed_global_call_contract(...)` and made both
entry-level and view-level validation consume that helper.

Removed the individual proof-reader functions that were only used to rebuild
the same allow-list.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/stage0_shape_inventory_guard.sh
git diff --check
```

## Result

The Stage0 C metadata reader now has one typed global-call proof allow-list.
Proof strings remain MIR-owned, and behavior is unchanged.
