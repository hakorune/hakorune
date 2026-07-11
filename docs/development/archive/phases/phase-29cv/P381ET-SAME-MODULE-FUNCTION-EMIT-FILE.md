# P381ET Same-Module Function Emit File

Date: 2026-05-06
Scope: move the Stage0 same-module function body emitter to an owner-neutral file name.

## Context

The body prepass/emitter now serves selected same-module functions for direct
global-call routes across module-generic and uniform-MIR definition owners. The
old file name still used the historical `module_generic_string` stem.

The old file name was:

```text
hako_llvmc_ffi_module_generic_string_function_emit.inc
```

## Change

Moved the body prepass/emitter to:

```text
hako_llvmc_ffi_same_module_function_emit.inc
```

Updated the include site in `hako_llvmc_ffi_pure_compile_generic_lowering.inc`.

No behavior changed. The inner `module_generic_string_*` helper names remain for
later mechanical cleanup so this commit stays file-boundary only.

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

The selected same-module function emitter no longer sits under a string-specific
file name.
