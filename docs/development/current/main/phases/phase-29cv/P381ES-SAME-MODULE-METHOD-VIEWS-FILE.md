# P381ES Same-Module Method Views File

Date: 2026-05-06
Scope: move Stage0 generic-method view predicates to an owner-neutral file name.

## Context

The generic-method view predicates are a table-like LoweringPlan contract used
by the same-module function body emitter. They are not a string-only surface,
even though the old include file lived under the historical
`module_generic_string` stem.

The old file name was:

```text
hako_llvmc_ffi_module_generic_string_method_views.inc
```

## Change

Moved the predicates to:

```text
hako_llvmc_ffi_same_module_method_views.inc
```

Updated the include site in `hako_llvmc_ffi_module_generic_string_function_emit.inc`.

No behavior changed. The inner `module_generic_string_method_view_*` helper names
remain for a later mechanical cleanup so this commit stays file-boundary only.

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

The generic-method route predicate table no longer sits under a string-specific
file name.
