# P381GW Same-Module Function Registry Helper

Date: 2026-05-06
Scope: deduplicate the planned/emitted same-module function symbol registries in
the Stage0 same-module function-plan seam.

## Context

Post-P381GI the `phase-29cv` lane is in optional polish only. The allowed next
work is targeted helper dedup when a local owner seam is clear.

`lang/c-abi/shims/hako_llvmc_ffi_same_module_function_plan.inc` still carried
two parallel registry families:

- planned same-module function symbols
- emitted same-module function symbols

Both repeated the same "contains / capacity check / remember" logic even though
the ownership and failure contract were identical.

## Change

- Added `same_module_function_symbol_registry_contains(...)`.
- Added `remember_same_module_function_symbol_in_registry(...)`.
- Rewired the planned/emitted registry wrappers to use those shared helpers.

No behavior changed:

- capacity failures still report `same_module_function_registry_full`
- planned/emitted registries stay separate
- Stage0 ownership and selected-set planning semantics are unchanged

## Result

The same-module function-plan seam now owns registry mechanics in one place
instead of duplicating them across planned/emitted wrappers.

This is optional-polish BoxShape cleanup only. It keeps the Stage0 LLVM line a
little smaller without adding new accept shapes or body-specific emit logic.

## Validation

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/stage0_shape_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
