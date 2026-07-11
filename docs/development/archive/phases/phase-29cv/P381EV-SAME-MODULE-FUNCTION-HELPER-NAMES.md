# P381EV Same-Module Function Helper Names

Date: 2026-05-06
Scope: rename Stage0 same-module function emitter helper identifiers after the owner-neutral file move.

## Context

P381ET moved the selected same-module body emitter to:

```text
hako_llvmc_ffi_same_module_function_emit.inc
```

The internal helper identifiers still used the historical
`module_generic_string_*` prefix.

## Change

Renamed internal helper identifiers and local context structs to:

```text
same_module_function_*
SameModuleFunction*
```

The existing `module_generic_string_const_missing` diagnostic reason string is
kept stable because it can appear in unsupported-shape diagnostics.

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

The active same-module function emitter now presents owner-neutral helper names
inside the owner-neutral shim file. Remaining `module_generic_string` references
in this file are diagnostic text only.
