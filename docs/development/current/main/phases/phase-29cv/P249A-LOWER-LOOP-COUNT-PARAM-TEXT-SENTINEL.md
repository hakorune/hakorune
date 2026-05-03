---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P249a, LowerLoopCountParam active text sentinel
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P248A-PARAM-LIST-STRING-VIEW.md
  - lang/src/mir/builder/internal/lower_loop_count_param_box.hako
---

# P249a: Lower Loop Count-Param Text Sentinel

## Problem

P248a advances the source-exe frontier to:

```text
target_shape_blocker_symbol=LowerLoopCountParamBox.try_lower/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```

`LowerLoopCountParamBox.try_lower/1` is a lowerer with many no-match `return
null` paths and a successful MIR JSON string return. On the active Stage0 route
the caller only needs a text lower result or a no-match marker.

## Decision

Do not widen generic string body and do not add a new body shape.

Add an active text API:

```text
try_lower_text(program_json) -> string
```

where `""` means no-match and non-empty string means lowered MIR JSON. Keep the
legacy `try_lower/1` wrapper for compatibility by converting `""` back to
`null`.

Update active callers to use `try_lower_text/1` and check for non-empty string.

## Non-Goals

- no new `GlobalCallTargetShape`
- no C body-specific emitter
- no change to accepted count-param loop pattern
- no fallback route
- no generic void-sentinel widening

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p249a_lower_loop_count_param_text_sentinel.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: the frontier should move past
`LowerLoopCountParamBox.try_lower/1`; a later blocker may remain.

## Result

Observed route metadata:

```text
target_symbol=LowerLoopCountParamBox.try_lower_text/1
target_shape=generic_pure_string_body
return_shape=string_handle
tier=DirectAbi
```

The source-exe frontier moved past `LowerLoopCountParamBox.try_lower/1`. The
next frontier is again the broader function-lowering owner:

```text
target_shape_blocker_symbol=FuncLoweringBox.lower_func_defs/2
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

Route metadata narrows the nested unsupported target to:

```text
target_symbol=FuncLoweringBox._lower_func_body/5
target_shape_blocker_symbol=FuncBodyBasicLowerBox._try_lower_loop/4
target_shape_blocker_reason=generic_string_unsupported_method_call
```
