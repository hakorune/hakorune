---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P380F, selfhost_exe_stageb direct-route env boundary
Related:
  - docs/development/current/main/phases/phase-29cv/P380E-STAGE1-CLI-ENV-EMIT-MIR-SMOKE.md
  - docs/development/current/main/phases/phase-29cv/P375A-PHASE29CG-MIR-FIRST-DIRECT-VERIFY-REMEASURE.md
  - tools/selfhost_exe_stageb.sh
---

# P380F: selfhost_exe_stageb Direct Env Boundary

## Problem

P380E proved that a clean direct MIR JSON capture of
`lang/src/runner/stage1_cli_env.hako` can be lowered through the pure-first EXE
route and can execute methodize-on `emit-mir` canaries. The formal helper still
failed:

```text
[llvm-pure/unsupported-shape] ... target_shape_blocker_symbol=StringScanBox.find_unescaped/3 target_shape_blocker_reason=generic_string_return_not_string
```

The env inventory isolated the difference:

```text
NYASH_MACRO_DISABLE=1 HAKO_MACRO_DISABLE=1
  -> pure-first compile OK

HAKO_JOINIR_PLANNER_REQUIRED=1 plus macro-disable
  -> pure-first compile OK

HAKO_JOINIR_STRICT=1 plus macro-disable
  -> direct MIR generation freezes

HAKO_JOINIR_STRICT=1 HAKO_JOINIR_PLANNER_REQUIRED=1 plus macro-disable
  -> MIR generation succeeds, but pure-first lowers fail at StringScanBox.find_unescaped/3
```

`tools/selfhost_exe_stageb.sh` forced both strict/dev knobs to `1` in its
default `direct` route. That makes the formal build helper generate a different
MIR dialect than the clean MIR-first proof.

## Decision

The `direct` route is the operational MIR-first EXE build route, not the
JoinIR strict/dev gate. It must not inject `HAKO_JOINIR_STRICT` or
`HAKO_JOINIR_PLANNER_REQUIRED` by default.

Caller-provided values remain opt-in. Strict/dev gate scripts can still pass
these env vars explicitly, but the default helper build should match the clean
source MIR path used by P380D/P380E.

## Non-Goals

- no Stage0 route/classifier widening
- no new `GlobalCallTargetShape`
- no C shim emitter change
- no source-owner workaround in `StringScanBox`
- no change to explicit `stageb-delegate` bridge capsule behavior

## Acceptance

```bash
NYASH_LLVM_SKIP_BUILD=1 \
HAKO_BACKEND_COMPILE_RECIPE=pure-first \
HAKO_BACKEND_COMPAT_REPLAY=none \
bash tools/selfhost_exe_stageb.sh \
  lang/src/runner/stage1_cli_env.hako \
  -o /tmp/p380f_stage1_cli_env.exe

NYASH_DISABLE_PLUGINS=1 \
HAKO_MIR_BUILDER_METHODIZE=1 \
NYASH_STAGE1_MODE=emit-mir \
STAGE1_SOURCE_TEXT='static box Main { main() { return 1 + 2 } }' \
/tmp/p380f_stage1_cli_env.exe

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: the formal helper emits `[emit-route] direct MIR-first`, links the
EXE, and the generated EXE returns `Result: 0` for the small `emit-mir` canary.

## Result

Implemented.

`tools/selfhost_exe_stageb.sh` now keeps the default `direct` route aligned with
the clean MIR-first proof:

- canonical schema/unified-call/using/macro-disable pins are preserved
- `HAKO_JOINIR_STRICT` is only forwarded when the caller already set it
- `HAKO_JOINIR_PLANNER_REQUIRED` is only forwarded when the caller already set it
- builder diagnostic toggles are only forwarded when the caller already set them

Verification:

```text
[emit-route] direct MIR-first (--emit-mir-json)
[emit] MIR JSON: /tmp/tmp.cJeGoWuvN8.json (85464802 bytes)
[link] EXE: /tmp/p380f_stage1_cli_env.exe
```

The generated EXE also passed the methodize-on `emit-mir` canary:

```text
STAGE1_SOURCE_TEXT='static box Main { main() { return 1 + 2 } }'
Result: 0
```

Post-change guards:

```text
bash -n tools/selfhost_exe_stageb.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

All passed.
