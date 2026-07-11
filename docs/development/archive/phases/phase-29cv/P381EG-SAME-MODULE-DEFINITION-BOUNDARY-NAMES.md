# P381EG Same-Module Definition Boundary Names

Date: 2026-05-06
Scope: rename Stage0 same-module definition boundary helpers.

## Context

P381EA/P381EB moved selected-set and definition driver vocabulary from
module-generic string wording to same-module function wording. Two local helper
names at the definition-entry boundary still used the older owner-specific
prefix even though their logic is shared by `module_generic` and `uniform_mir`
definition owners.

## Change

Renamed the local C helper boundaries:

- `module_generic_string_function_definition_is_eligible` ->
  `same_module_function_definition_is_eligible`
- `module_generic_string_read_function_view` ->
  `same_module_function_read_view`

No behavior changed. Body internals keep `module_generic_string` names until a
separate cleanup can move those responsibilities behind narrower helpers.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/stage0_shape_inventory_guard.sh
git diff --check
```

## Result

The selected same-module definition entry now exposes owner-neutral names at the
boundary shared by `module_generic` and `uniform_mir` owners.
