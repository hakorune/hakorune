---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P308a, LowerLoopCountParam finish helper split
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P302A-LOWER-LOOP-COUNT-PARAM-TEXT-BOUNDARY-SPLIT.md
  - docs/development/current/main/phases/phase-29cv/P307A-MIR-JSON-FUNCTION-BLOCK-ARRAY-GET-CONSUME.md
  - lang/src/mir/builder/internal/lower_loop_count_param_box.hako
---

# P308a: LowerLoopCountParam Finish Helper Split

## Problem

P307a advances the source-exe probe past the MIR JSON emitter and stops at:

```text
reason=missing_multi_function_emitter
target_shape_blocker_symbol=LowerLoopCountParamBox._finish_count_param_text/5
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

`_finish_count_param_text/5` is still an owner-local text lowerer, but it keeps
normalized compare unpacking, loop body binary lookup, operator acceptance, step
normalization, and final JSON emission in one function.

That shape reintroduces the broad branch/PHI pressure that P302a removed from
`try_lower_text/1`.

## Decision

Split `_finish_count_param_text/5` into smaller owner-local helpers:

```text
_norm_cmp_op_text(norm)
_norm_limit_text(norm)
_loop_increment_binary_pos(s, k_loop, varname)
_body_increment_op_text(s, k_bop, k_bop_end)
_loop_increment_step_text(s, k_loop, varname)
```

Each text helper keeps `""` as the failed-probe sentinel. Numeric position
helpers use `-1`.

## Non-Goals

- no C shim acceptance widening
- no new `GlobalCallTargetShape`
- no generic string/body classifier widening
- no ArrayBox/MapBox semantics
- no loop pattern policy change

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p308a_finish_split.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected probe result:

```text
LowerLoopCountParamBox._finish_count_param_text/5 no longer reports
generic_string_unsupported_void_sentinel_const.
```

## Result

Accepted.

`_finish_count_param_text/5` now delegates compare unpacking and loop increment
step discovery to smaller owner-local helpers. The callsites from
`LowerLoopCountParamBox.try_lower_text/1` now classify as:

```text
LowerLoopCountParamBox._finish_count_param_text/5
tier=DirectAbi
proof=typed_global_call_generic_pure_string
target_shape=generic_pure_string_body
```

The probe advances to the next module generic prepass blocker:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox._emit_inst/1
target_shape_blocker_reason=-
```
