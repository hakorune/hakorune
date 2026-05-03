---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P243a, LowerLoopCountParam owner-local final MIR JSON emit
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P242A-GENERIC-I64-UNARY-NOT.md
  - lang/src/mir/builder/internal/lower_loop_count_param_box.hako
---

# P243a: Lower Loop Count-Param Inline Emit

## Problem

P242a advances the source-exe probe to:

```text
target_shape_blocker_symbol=LowerLoopCountParamBox.try_lower/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```

`LowerLoopCountParamBox.try_lower/1` is a string-or-null lowerer. Its no-match
paths return null, while its match path returns MIR JSON through the sibling
helper `_emit_count_param_json/4`.

That sibling helper is already a direct pure string body, but keeping the only
string return behind another local global call makes the route fact harder than
the owner path needs to be.

## Decision

Do not add a new body shape and do not widen generic string.

Inline the final MIR JSON assembly into `try_lower/1` and remove the sibling
emit helper. The owner method already owns all accepted facts
(`start_text`, `limit`, `step`, `cmp`), so the final string return should be
visible at the same owner boundary.

## Non-Goals

- no new `GlobalCallTargetShape`
- no C body-specific emitter
- no change to the accepted loop count-param pattern
- no fallback route
- no broad string classifier expansion

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p243a_lower_loop_count_param_inline_emit.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: the frontier should move past
`LowerLoopCountParamBox.try_lower/1`; a later blocker may remain.

## Result

Observed probe:

```text
target_shape=generic_string_or_void_sentinel_body
target_symbol=LowerLoopCountParamBox.try_lower/1
return_shape=string_handle_or_null
tier=DirectAbi
```

The next source-exe frontier is:

```text
target_shape_blocker_symbol=CliRunShapeScannerBox.scan/1
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
backend_reason=missing_multi_function_emitter
```
