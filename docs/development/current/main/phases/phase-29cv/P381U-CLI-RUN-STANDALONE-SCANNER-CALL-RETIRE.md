---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381U, retire active standalone CLI run shape scanner call
Related:
  - docs/development/current/main/phases/phase-29cv/P381T-CLI-RUN-SHAPE-TEXT-TRACE-COUNT.md
  - docs/development/current/main/phases/phase-29cv/P244a-CLI-RUN-SHAPE-SCANNER-VOID.md
  - lang/src/mir/builder/func_lowering.hako
  - lang/src/mir/builder/func_body/cli_run_lower_box.hako
---

# P381U: CLI Run Standalone Scanner Call Retire

## Problem

P381T removes the `CliRunShapeScannerBox.scan/1` trace-counter Bool/i64
arithmetic issue, but the direct Stage1 env EXE route still stops at the
standalone scanner call:

```text
target_shape_blocker_symbol=CliRunShapeScannerBox.scan/1
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
backend_reason=missing_multi_function_emitter
```

The standalone scanner is observability-only. It returns `null`, does not lower
MIR, and duplicates trace tags that `CliRunLowerBox.lower_run/5` already emits
when it recognizes `HakoCli.run`.

## Decision

Remove the active standalone scanner call from
`FuncLoweringBox.lower_func_defs/2`.

Keep `CliRunLowerBox` as the owner for active `HakoCli.run` shape observation.
Leave `CliRunShapeScannerBox` as a shelved diagnostic helper until a later
inventory decides whether to delete it.

## Boundary

Allowed:

- remove the `CliRunShapeScannerBox.scan(s)` active call
- remove its `using` from `FuncLoweringBox`
- keep `CliRunLowerBox` trace tags unchanged

Not allowed:

- add a backend body shape for the standalone scanner
- widen void/null return ABI handling for this diagnostic path
- change actual function lowering output

## Acceptance

```bash
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
target/release/hakorune --emit-exe /tmp/p381u_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

KEEP_OUT_DIR=1 OUT_DIR=/tmp/p381u_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- pure-first no longer has an active dependency on
  `CliRunShapeScannerBox.scan/1`
- CLI run observation remains owned by `CliRunLowerBox`

## Result

Implemented. `FuncLoweringBox` no longer calls the standalone
`CliRunShapeScannerBox.scan/1` probe from the active pure-first route.
