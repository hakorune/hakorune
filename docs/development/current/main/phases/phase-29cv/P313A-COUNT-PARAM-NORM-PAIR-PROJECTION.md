---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P313a, LowerLoopCountParam normalized cmp/limit projection
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P308A-LOWER-LOOP-COUNT-PARAM-FINISH-SPLIT.md
  - docs/development/current/main/phases/phase-29cv/P312A-MIR-JSON-PHI-INCOMING-ARRAY-CONSUME.md
  - lang/src/mir/builder/internal/lower_loop_count_param_box.hako
---

# P313a: Count Param Norm Pair Projection

## Problem

P312a advances the source-exe probe to:

```text
reason=missing_multi_function_emitter
target_shape_blocker_symbol=LowerLoopCountParamBox._norm_cmp_op_text/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

`_norm_cmp_op_text/1` and `_norm_limit_text/1` are owner-local helpers that
project the normalized `cmp:limit` pair produced by
`PatternUtilBox.normalize_cmp_limit(...)`.

The helpers still perform their own failed-probe checks and return `""` when
the separator is missing. That duplicates the sentinel contract from
`_finish_count_param_text/5` and makes the projection helpers look like
string-or-void bodies.

## Decision

Move the separator lookup and failed-probe guard into `_finish_count_param_text/5`.
Then make the projection helpers total over a validated separator position:

```text
_norm_cmp_op_text(norm, cpos)
_norm_limit_text(norm, cpos)
```

This keeps the text sentinel at the owner boundary and leaves the helpers as
pure string projections.

## Non-Goals

- no generic string/body classifier widening
- no C shim acceptance widening
- no new `GlobalCallTargetShape`
- no count-param loop policy change
- no change to `PatternUtilBox.normalize_cmp_limit(...)`

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p313a_norm_pair.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected probe result:

```text
LowerLoopCountParamBox._norm_cmp_op_text/1 no longer appears as the
target_shape_blocker_symbol.
```

## Result

Accepted. `_finish_count_param_text/5` now owns the separator guard, and
`_norm_cmp_op_text/2` / `_norm_limit_text/2` are pure projections over a
validated separator position.

Validation:

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p313.exe lang/src/runner/stage1_cli_env.hako
```

The probe advanced past the count-param normalized pair helpers to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox._emit_vid_array_rec/3
target_shape_blocker_reason=-
```
