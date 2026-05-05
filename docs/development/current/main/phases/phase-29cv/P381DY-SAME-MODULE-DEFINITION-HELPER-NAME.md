# P381DY Same-Module Definition Helper Name

Date: 2026-05-06
Scope: rename the Stage0 global-call definition requirement helper after P381DW.

## Context

After P381DW, Phase 1 direct-only retired capsules advertise
`definition_owner=uniform_mir`. The C helper
`lowering_plan_global_call_view_requires_module_generic_definition(...)` still
accepted both `module_generic` and `uniform_mir`, so its name no longer matched
the contract it enforced.

## Change

Renamed the helper to
`lowering_plan_global_call_view_requires_same_module_function_definition(...)`
and updated all call sites:

- direct MIR call shell emission guard
- selected-set planning
- module-generic prepass type/origin handling

No owner policy, proof validation, selected-set behavior, or emitted LLVM changed.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/stage0_shape_inventory_guard.sh
git diff --check
```

## Result

The C metadata helper now names the actual Stage0 contract: a direct global call
requires a selected same-module function definition, whether its MIR owner is
`module_generic` or `uniform_mir`.
