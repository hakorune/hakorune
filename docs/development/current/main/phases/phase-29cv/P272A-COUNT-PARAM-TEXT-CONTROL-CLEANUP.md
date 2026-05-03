---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P272a, count-param text control cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P271A-BASIC-LOCAL-IF-TEXT-WRAPPERS.md
  - lang/src/mir/builder/internal/lower_loop_count_param_box.hako
---

# P272a: Count-Param Text Control Cleanup

## Problem

After P271a, the source-exe probe first stops at:

```text
target_shape_blocker_symbol=LowerLoopCountParamBox.try_lower_text/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```

The generated MIR for `LowerLoopCountParamBox.try_lower_text/1` does not show a
visible `void`/`null` const in the body instructions. The source body is still a
large text lowerer with boolean negations and a final large JSON string
construction inside the same function. That keeps value-class inference and
return-shape proof coupled to one oversized method.

## Decision

Keep the Stage0/classifier boundary unchanged and clean the owner-local source
shape instead:

```text
bool negation in try_lower_text -> i64 flags with explicit comparisons
final MIR JSON concat          -> _emit_count_param_json(...) helper
```

This is a BoxShape cleanup. It does not add collection semantics or a new body
shape to Stage0.

## Non-Goals

- no count-loop accepted-shape expansion
- no new `GlobalCallTargetShape`
- no generic void-sentinel classifier change
- no C shim/body-specific emitter change
- no ArrayBox/MapBox method acceptance

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p272a_count_param_text_control_cleanup.exe lang/src/runner/stage1_cli_env.hako
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
LowerLoopCountParamBox._emit_count_param_json/4  generic_pure_string_body  string_handle  DirectAbi
LowerLoopCountParamBox.try_lower_text/1          generic_pure_string_body  string_handle  DirectAbi
```

The next explicit blocker is now the higher-level function defs owner:

```text
target_shape_blocker_symbol=FuncLoweringBox.lower_func_defs/2
target_shape_blocker_reason=generic_string_unsupported_instruction
backend_reason=missing_multi_function_emitter
```
