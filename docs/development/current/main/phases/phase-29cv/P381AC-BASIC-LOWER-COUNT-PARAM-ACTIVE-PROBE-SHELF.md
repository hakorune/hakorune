---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381AC, BasicLower count-param active probe shelf
Related:
  - docs/development/current/main/phases/phase-29cv/P381V-COUNT-PARAM-PUBLIC-WRAPPER-CALL-CONTRACT.md
  - docs/development/current/main/phases/phase-29cv/P381AB-FOLD-VARINT-TEXT-SENTINEL-CARRIERS.md
  - lang/src/mir/builder/func_body/basic_lower_box.hako
  - lang/src/mir/builder/internal/lower_loop_count_param_box.hako
---

# P381AC: BasicLower Count Param Active Probe Shelf

## Problem

P381V routes `FuncBodyBasicLowerBox` through the public
`LowerLoopCountParamBox.try_lower/1` wrapper, but the direct Stage1 env EXE
route eventually returns to the nested implementation body:

```text
target_shape_blocker_symbol=LowerLoopCountParamBox.try_lower_text/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```

That means the current Stage0 route still has to understand the count-param
lowerer implementation body just because BasicLower probes it.

## Decision

Shelf the count-param probe from the active `FuncBodyBasicLowerBox` loop
dispatch for this direct route closeout. Keep `LowerLoopCountParamBox` source
intact for later restoration through the uniform multi-function emitter or a
separate source-owner cleanup card.

This follows the Stage0 rule: do not teach Stage0 the body semantics of a
selfhost compiler helper just to keep a speculative probe active.

## Boundary

Allowed:

- remove `LowerLoopCountParamBox` from the active BasicLower probe order
- keep sum/break-continue, multi-carrier, and simple loop probes unchanged
- keep the count-param lowerer source file intact

Not allowed:

- delete count-param lowerer source
- add backend acceptance for `try_lower_text/1`
- add a new `GlobalCallTargetShape`

## Acceptance

```bash
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
target/release/hakorune --emit-exe /tmp/p381ac_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

KEEP_OUT_DIR=1 OUT_DIR=/tmp/p381ac_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- direct Stage1 env EXE route no longer has an active dependency on
  `LowerLoopCountParamBox.try_lower_text/1`
- phase29cg bridge keeper remains green

## Result

Implemented. The count-param probe is shelved from the active
`FuncBodyBasicLowerBox` loop dispatch.
