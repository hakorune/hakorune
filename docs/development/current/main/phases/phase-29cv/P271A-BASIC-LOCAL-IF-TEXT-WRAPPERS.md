---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P271a, basic local-if text wrappers
Related:
  - docs/development/current/main/phases/phase-29cv/P266A-BASIC-LOCAL-IF-TEXT-FAIL-SENTINEL.md
  - docs/development/current/main/phases/phase-29cv/P270A-FUNC-DEFS-REMOVE-TRACE-EXTERN.md
  - lang/src/mir/builder/func_body/basic_lower_box.hako
---

# P271a: Basic Local-If Text Wrappers

## Problem

After P270a, the source-exe probe first stops at:

```text
target_shape_blocker_symbol=FuncBodyBasicLowerBox._try_lower_local_if_return/4
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```

P266a changed the try-lower failure return to `""`, but the body still compares
several extraction helper results against `null`. Those comparisons materialize
void sentinel constants inside the candidate body.

## Decision

Keep the local-if lowerer on a text-only contract by converting nullable helper
results at the boundary:

```text
nullable helper result -> _text_or_empty(value)
try-lower body checks only ""
```

This keeps void/null handling in a tiny wrapper and avoids widening the generic
string classifier.

## Non-Goals

- no local-if accepted-shape change
- no new `GlobalCallTargetShape`
- no generic void-sentinel classifier change
- no C shim/body-specific emitter change

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p271a_basic_local_if_text_wrappers.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: route metadata no longer classifies
`FuncBodyBasicLowerBox._try_lower_local_if_return/4` as
`generic_string_unsupported_void_sentinel_const`. A later explicit blocker may
remain.

## Result

Accepted.

The source-exe probe no longer stops at
`FuncBodyBasicLowerBox._try_lower_local_if_return/4`:

```text
FuncBodyBasicLowerBox._try_lower_local_if_return/4  generic_pure_string_body  string_handle  DirectAbi
FuncBodyBasicLowerBox._text_or_empty/1              generic_pure_string_body  string_handle  DirectAbi
```

The next explicit blocker is now back in the loop lowerer:

```text
target_shape_blocker_symbol=LowerLoopCountParamBox.try_lower_text/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```
