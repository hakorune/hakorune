# P381EH Same-Module Prepass Split

Date: 2026-05-06
Scope: split same-module global-call definition prepass from the local print fallback.

## Context

After P381EG, the definition-entry boundary used same-module function wording,
but `module_generic_string_prepass_global_call_view(...)` still mixed two
responsibilities:

- selected same-module or leaf definition facts from LoweringPlan metadata
- the local `print` fallback used by the module-generic string prepass

The first branch is shared by `module_generic` and `uniform_mir` definition
owners. The second branch remains local to the old module-generic string
prepass.

## Change

Extracted the selected same-module/leaf definition branch into
`same_module_function_prepass_global_call_definition_view(...)`.

Kept the `print` fallback in `module_generic_string_prepass_global_call_view`.

No accepted shape changed.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
cargo test -q mir::global_call_route_plan::tests::void_logging
cargo test -q mir::global_call_route_plan::tests::static_string_array
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/stage0_shape_inventory_guard.sh
git diff --check
```

## Result

The global-call prepass has a clearer boundary: LoweringPlan-selected
same-module definitions are handled by an owner-neutral helper, and the legacy
print fallback remains local until the remaining module-generic string body
internals are split further.
