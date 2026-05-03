---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P260a, count-param text result guard
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P259A-RETURN-BINARY-TEXT-SENTINELS.md
  - lang/src/mir/builder/func_body/basic_lower_box.hako
  - lang/src/mir/builder/internal/lower_loop_count_param_box.hako
---

# P260a: Count Param Text Result Guard

## Problem

After P259a, the source-exe probe advances to:

```text
target_shape_blocker_symbol=LowerLoopCountParamBox.try_lower_text/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

`LowerLoopCountParamBox.try_lower_text/1` has a text-sentinel contract:

```text
unsupported -> ""
supported   -> MIR JSON string
```

However, the caller in `FuncBodyBasicLowerBox._try_lower_loop/4` still checks
the result as nullable:

```text
if out_cp != null && out_cp != "" { ... }
```

That stale nullable guard reintroduces a void-sentinel check around a text-only
helper.

## Decision

Do not widen generic void-sentinel handling.

Align the caller with the `try_lower_text` contract and check only:

```text
if out_cp != "" { ... }
```

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic void-sentinel acceptance change
- no change to count-param loop lowering priority
- no C body-specific emitter

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p260a_count_param_text_result_guard.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: the route should move past the
`LowerLoopCountParamBox.try_lower_text/1` void-sentinel blocker; a later blocker
may remain.

## Result

`LowerLoopCountParamBox.try_lower_text/1` now routes as:

```text
LowerLoopCountParamBox.try_lower_text/1  generic_pure_string_body  DirectAbi
```

`FuncBodyBasicLowerBox._try_lower_loop/4` remains direct:

```text
FuncBodyBasicLowerBox._try_lower_loop/4  generic_string_or_void_sentinel_body  DirectAbi
```

The source-exe probe now advances to:

```text
target_shape_blocker_symbol=FuncLoweringBox._lower_return_call/6
target_shape_blocker_reason=generic_string_unsupported_instruction
backend_reason=missing_multi_function_emitter
```
