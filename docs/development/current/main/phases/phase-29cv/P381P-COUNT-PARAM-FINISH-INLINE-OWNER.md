---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381P, LowerLoopCountParam finish owner cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P308A-LOWER-LOOP-COUNT-PARAM-FINISH-SPLIT.md
  - docs/development/current/main/phases/phase-29cv/P313A-COUNT-PARAM-NORM-PAIR-PROJECTION.md
  - docs/development/current/main/phases/phase-29cv/P381O-PURE-MEM2REG-DIAGNOSTIC-SURFACE.md
  - lang/src/mir/builder/internal/lower_loop_count_param_box.hako
---

# P381P: Count Param Finish Inline Owner

## Problem

P381O exposed the exact full Stage1 env EXE blocker:

```text
reason=missing_multi_function_emitter
target_shape_blocker_symbol=LowerLoopCountParamBox._finish_count_param_text/5
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

This is the same source-owner seam recorded around P308/P313/P380C. The helper
is not a new compiler feature. It is an owner-local finish step for
`LowerLoopCountParamBox.try_lower_text/1`.

Adding a generic string/body acceptance for the helper would widen Stage0.

## Decision

Do not add a new backend body shape.

Keep `try_lower_text/1` as the owner boundary and inline the finish sequence
there after the normalized compare/limit pair is selected:

```text
norm -> cpos -> cmp -> limit -> step -> emit_count_param_json
```

The old helper remains as source text for now but is no longer on the active
lowering path. A later prune card can delete it after inventory confirms no
active caller.

## Boundary

Allowed:

- remove the active call to `_finish_count_param_text/5`
- keep `""` failed-probe sentinel in `try_lower_text/1`
- reuse existing projection and emit helpers

Not allowed:

- add a new `GlobalCallTargetShape`
- widen `generic_string_body`
- change emitted MIR JSON shape
- add fallback/delegate routing

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh

NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
target/release/hakorune --emit-exe /tmp/p381p_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

KEEP_OUT_DIR=1 OUT_DIR=/tmp/p381p_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- the direct Stage1 env EXE route no longer reports
  `_finish_count_param_text/5`
- phase29cg bridge keeper remains green
- no backend body-shape/C-shim acceptance widening

## Result

Done:

- removed the active `_finish_count_param_text/5` callsite from
  `try_lower_text/1`
- kept the count-param finish flow inside the existing owner boundary
