# P381EU Same-Module Method View Helper Names

Date: 2026-05-06
Scope: rename Stage0 generic-method view helper prefix after the owner-neutral file move.

## Context

P381ES moved the generic-method view predicate table to:

```text
hako_llvmc_ffi_same_module_method_views.inc
```

The helper functions inside the file still used the historical
`module_generic_string_method_view_*` prefix.

## Change

Renamed the helper prefix to:

```text
same_module_method_view_*
```

Updated the call sites in `hako_llvmc_ffi_same_module_function_emit.inc`.

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

The method-view helper names now match the same-module method view shim instead
of the old string-specific file stem.
