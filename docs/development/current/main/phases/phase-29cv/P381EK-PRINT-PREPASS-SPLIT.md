# P381EK Print Prepass Split

Date: 2026-05-06
Scope: isolate the module-generic string print fallback in the global-call prepass.

## Context

P381EH split selected same-module definition facts out of
`module_generic_string_prepass_global_call_view(...)`. The remaining inline
branch was the old module-generic string `print` fallback.

## Change

Extracted the fallback check into
`module_generic_string_prepass_print_global_call_surface(...)`.

No behavior changed.

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

The global-call prepass now has two visible responsibilities: owner-neutral
same-module definition facts and the local module-generic string print fallback.
