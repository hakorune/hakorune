---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P253a, LowerLoopCountParam text sentinel wrappers
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P249A-LOWER-LOOP-COUNT-PARAM-TEXT-SENTINEL.md
  - docs/development/current/main/phases/phase-29cv/P252A-FUNC-LOWERING-DIRECT-DEFS-ACCUMULATOR.md
  - lang/src/mir/builder/internal/lower_loop_count_param_box.hako
---

# P253a: Count-Param Text Sentinel Wrappers

## Problem

After P252a, the source-exe probe reaches:

```text
target_shape_blocker_symbol=LowerLoopCountParamBox.try_lower_text/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

`try_lower_text/1` already exposes an empty-string no-match contract, but its
active body still guards several helper calls with `null` comparisons. That
leaves void sentinel constants inside a function that should be a pure text
emitter.

## Decision

Do not widen generic string void handling and do not add a new shape.

Add small owner-local wrappers that convert nullable helper results into text
sentinels before the active body consumes them:

```text
_loop_var_name_text(...)
_local_int_text_before(...)
_read_string_after_text(...)
_read_int_after_text(...)
_normalize_cmp_limit_text(...)
```

Then make `try_lower_text/1` branch on `""` instead of `null` for those
nullable lookups.

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic null/void widening
- no change to count-param loop semantics
- no C body-specific emitter
- no rewrite of PatternUtilBox or LoopScanBox ownership

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p253a_count_param_text_sentinel_wrappers.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: the route should move past the void sentinel
blocker in `LowerLoopCountParamBox.try_lower_text/1`; a later blocker may remain.

## Result

`LowerLoopCountParamBox.try_lower_text/1` now routes as
`generic_pure_string_body` with `DirectAbi`.

The owner-local nullable-to-text wrappers also route as `generic_pure_string_body`:

```text
LowerLoopCountParamBox._loop_var_name_text/2        DirectAbi
LowerLoopCountParamBox._local_int_text_before/3     DirectAbi
LowerLoopCountParamBox._read_string_after_text/2    DirectAbi
LowerLoopCountParamBox._read_int_after_text/2       DirectAbi
LowerLoopCountParamBox._normalize_cmp_limit_text/3  DirectAbi
```

The source-exe probe now advances to:

```text
target_shape_blocker_symbol=ExternCallLowerBox.lower_hostbridge/4
target_shape_blocker_reason=generic_string_unsupported_method_call
```
