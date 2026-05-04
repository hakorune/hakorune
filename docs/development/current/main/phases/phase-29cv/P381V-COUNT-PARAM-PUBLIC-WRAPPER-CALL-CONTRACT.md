---
Status: Superseded
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381V, BasicLower count-param public wrapper call contract
Related:
  - docs/development/current/main/phases/phase-29cv/P381P-COUNT-PARAM-FINISH-INLINE-OWNER.md
  - lang/src/mir/builder/func_body/basic_lower_box.hako
  - lang/src/mir/builder/internal/lower_loop_count_param_box.hako
---

# P381V: Count Param Public Wrapper Call Contract

## Problem

After P381U, the direct Stage1 env EXE route reaches:

```text
target_shape_blocker_symbol=LowerLoopCountParamBox.try_lower_text/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```

`try_lower_text/1` is the owner-internal text-sentinel implementation. The
public `try_lower/1` wrapper converts the internal `""` miss sentinel to
`null`, which is the caller-facing lowerer contract used by most BasicLower
delegates.

Calling the internal text function directly from `FuncBodyBasicLowerBox`
exposes an implementation detail to the active multi-function route.

## Decision

Change `FuncBodyBasicLowerBox._try_lower_loop/4` to call
`LowerLoopCountParamBox.try_lower/1` and check for `null`, matching the public
lowerer contract.

This preserves count-param lowering as an active capability without adding a
backend body shape or C shim emitter.

## Boundary

Allowed:

- change the BasicLower caller from `try_lower_text` to `try_lower`
- keep `try_lower_text` as the owner-internal text-sentinel body
- keep emitted count-param MIR JSON unchanged

Not allowed:

- remove count-param lowerer capability
- add backend acceptance for the internal text-sentinel body
- add a new `GlobalCallTargetShape`

## Acceptance

```bash
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
target/release/hakorune --emit-exe /tmp/p381v_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

KEEP_OUT_DIR=1 OUT_DIR=/tmp/p381v_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- BasicLower no longer exposes the internal `try_lower_text/1` body as its
  active call target
- count-param loop lowering remains available through the public wrapper

## Result

Superseded by P381AC. The count-param probe was removed from the active
`FuncBodyBasicLowerBox` route instead of switching the caller to a public
wrapper, which keeps the speculative helper off the Stage1 env pure-first path.
