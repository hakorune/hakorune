# P381EI Module-Generic I64 Helper Name

Date: 2026-05-06
Scope: clarify Stage0 metadata helper naming for non-uniform direct-call definitions.

## Context

`lowering_plan_global_call_view_uses_module_generic_definition(...)` did not
only accept `definition_owner=module_generic`. It also accepted
`definition_owner=generic_i64_or_leaf`, and the broader
`lowering_plan_global_call_view_requires_same_module_function_definition(...)`
then combined that helper with `definition_owner=uniform_mir`.

That made the lower-level helper name narrower than its actual owner-family
contract.

## Change

Renamed the helper to
`lowering_plan_global_call_view_uses_module_generic_or_i64_definition(...)`.

No behavior changed.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/stage0_shape_inventory_guard.sh
git diff --check
```

## Result

The metadata helper name now matches the owner families it accepts:
`module_generic` and `generic_i64_or_leaf`.
