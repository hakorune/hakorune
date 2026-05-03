---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P380D, clean rebuild confirmation for Stage1 CLI env pure-first EXE
Related:
  - docs/development/current/main/phases/phase-29cv/P380C-TO-I64-SIGN-GUARD-VOID-PHI-RETIRE.md
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - lang/src/shared/common/string_helpers.hako
---

# P380D: Stage1 CLI Env Pure-First EXE Confirm

## Problem

P380C initially recorded the next route blocker from a stale MIR probe:

```text
target_shape_blocker_symbol=LowerLoopCountParamBox._finish_count_param_text/5
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

A clean rebuild of `target/release/hakorune` and fresh MIR regeneration showed
that P380C had in fact moved the route farther: `_finish_count_param_text/5`,
`Main._run_emit_mir_mode/1`, and
`main._run_emit_mir_program_json_compat_mode/1` all route through existing
DirectAbi plans.

## Decision

Record the clean probe as the current truth. Do not amend P380C. Do not add new
code or route acceptance in this card.

## Non-Goals

- no source changes
- no Stage0/classifier widening
- no C shim body-specific emitter changes
- no generated grammar formatting change

## Acceptance

```bash
cargo build --release --bin hakorune
target/release/hakorune --emit-mir-json /tmp/p380c_clean_stage1_cli_env.mir.json \
  lang/src/runner/stage1_cli_env.hako

NYASH_LLVM_SKIP_BUILD=1 \
NYASH_LLVM_ROUTE_TRACE=1 \
NYASH_LLVM_BACKEND=crate \
NYASH_NY_LLVM_COMPILER=target/release/ny-llvmc \
NYASH_EMIT_EXE_NYRT=target/release \
HAKO_BACKEND_COMPILE_RECIPE=pure-first \
HAKO_BACKEND_COMPAT_REPLAY=none \
bash tools/ny_mir_builder.sh \
  --in /tmp/p380c_clean_stage1_cli_env.mir.json \
  --emit exe \
  -o /tmp/p380c_clean_stage1_cli_env.exe

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

The clean route probe succeeds:

```text
[llvm-route/trace] stage=lowering_plan result=hit reason=mir_lowering_plan_v0 extra=consumer=mir_call_global_generic_i64_emit site=b14165.i2 route=global.user_call core_op=UserGlobalCall tier=DirectAbi symbol=Main._run_emit_mir_mode/1
[llvm-route/trace] stage=lowering_plan result=hit reason=mir_lowering_plan_v0 extra=consumer=mir_call_global_generic_i64_emit site=b14166.i1 route=global.user_call core_op=UserGlobalCall tier=DirectAbi symbol=Main._run_emit_mir_program_json_compat_mode/1
[ny-llvmc] executable written: /tmp/p380c_clean_stage1_cli_env.exe
OK exe:/tmp/p380c_clean_stage1_cli_env.exe
```

This confirms P380C removed the active source-execution compile blocker for the
Stage1 CLI env pure-first EXE build. The next blocker must be measured at the
execution/smoke layer, not from the stale P380C MIR file.
