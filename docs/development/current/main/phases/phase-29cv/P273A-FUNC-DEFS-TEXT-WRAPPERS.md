---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P273a, func-defs nullable text wrappers
Related:
  - docs/development/current/main/phases/phase-29cv/P272A-COUNT-PARAM-TEXT-CONTROL-CLEANUP.md
  - lang/src/mir/builder/func_lowering.hako
---

# P273a: Func-Defs Text Wrappers

## Problem

After a clean rebuild and P272a, the source-exe probe first stops at:

```text
target_shape_blocker_symbol=FuncLoweringBox.lower_func_defs/2
target_shape_blocker_reason=generic_string_unsupported_instruction
backend_reason=missing_multi_function_emitter
```

`lower_func_defs/2` still handles nullable owner-local helper results directly:

```text
defs_json guard
DefsScannerBox.extract_body(...)
FuncLoweringBox._lower_func_body(...)
```

That keeps void/null sentinel checks inside the large function-defs loop and
couples nullable flow to the loop-carried text accumulator.

## Decision

Keep `lower_func_defs/2` on a text-only contract by converting nullable helper
results at small owner-local boundaries:

```text
nullable input/result -> text wrapper -> "" failure sentinel
append candidate      -> _append_func_defs_text(acc, item)
return accumulator    -> text wrapper
lower_func_defs loop  -> only checks ""
```

This mirrors the local-if wrapper pattern and avoids widening Stage0 generic
string handling.

## Non-Goals

- no function-def accepted-shape expansion
- no new `GlobalCallTargetShape`
- no generic void-sentinel classifier change
- no C shim/body-specific emitter change
- no ArrayBox/MapBox method acceptance

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p273a_func_defs_text_wrappers.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: route metadata no longer classifies
`FuncLoweringBox.lower_func_defs/2` as `generic_string_unsupported_instruction`.
A later explicit blocker may remain.

## Result

Accepted.

The source-exe probe no longer stops at
`FuncLoweringBox.lower_func_defs/2` for
`generic_string_unsupported_instruction` or `generic_string_return_not_string`.
The new dependency blocker is:

```text
target_shape_blocker_symbol=FuncBodyBasicLowerBox._try_lower_local_if_return/4
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```

Current route metadata confirms the owner-level movement:

```text
FuncLoweringBox.lower_func_defs/2  generic_string_global_target_shape_unknown  FuncBodyBasicLowerBox._try_lower_local_if_return/4  generic_string_unsupported_void_sentinel_const  Unsupported
```
