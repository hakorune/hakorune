---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P269a, count-param loop lowerer void-scope removal
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P249A-LOWER-LOOP-COUNT-PARAM-TEXT-SENTINEL.md
  - docs/development/current/main/phases/phase-29cv/P253A-COUNT-PARAM-TEXT-SENTINEL-WRAPPERS.md
  - docs/development/current/main/phases/phase-29cv/P268A-RETURN-CALL-LOWERER-SELECT-FREE-TEXT.md
  - lang/src/mir/builder/internal/lower_loop_count_param_box.hako
---

# P269a: Count-Param Remove Void Scope

## Problem

After P268a, the source-exe probe first stops at:

```text
target_shape_blocker_symbol=LowerLoopCountParamBox.try_lower_text/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```

`try_lower_text/1` already uses the empty-string no-match contract, but it still
contains a standalone local block used only for variable scoping around step
extraction. That block can materialize a void sentinel in MIR even though the
helper itself is meant to be string-only.

## Decision

Do not widen generic string/void handling.

Remove the standalone scope and keep the same step extraction as a plain local
sequence. This is behavior-preserving and keeps the owner-local lowerer on the
text-sentinel contract:

```text
success -> MIR JSON text
no-match -> ""
```

## Non-Goals

- no accepted count-param loop shape change
- no new `GlobalCallTargetShape`
- no generic classifier acceptance change
- no C shim/body-specific emitter change

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p269a_count_param_remove_void_scope.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: route metadata no longer classifies
`LowerLoopCountParamBox.try_lower_text/1` as
`generic_string_unsupported_void_sentinel_const`. A later explicit blocker may
remain.

## Result

Accepted.

The source-exe probe no longer stops at
`LowerLoopCountParamBox.try_lower_text/1`:

```text
LowerLoopCountParamBox.try_lower_text/1  generic_pure_string_body  string_handle  DirectAbi
```

The next explicit blocker is now the defs orchestrator:

```text
target_shape_blocker_symbol=FuncLoweringBox.lower_func_defs/2
target_shape_blocker_reason=generic_string_unsupported_extern_call
backend_reason=missing_multi_function_emitter
```
