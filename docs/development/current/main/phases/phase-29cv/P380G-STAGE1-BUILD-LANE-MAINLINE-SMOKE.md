---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P380G, Stage1 build lane mainline smoke confirmation
Related:
  - docs/development/current/main/phases/phase-29cv/P380F-SELFHOST-EXE-STAGEB-DIRECT-ENV-BOUNDARY.md
  - tools/selfhost/mainline/build_stage1.sh
  - tools/selfhost/mainline/stage1_mainline_smoke.sh
  - tools/selfhost/lib/stage1_contract.sh
---

# P380G: Stage1 Build Lane Mainline Smoke

## Problem

P380E showed that a manually built full `stage1_cli_env.hako` EXE could satisfy
small `emit-mir` canaries, but the low-level `stage1_contract_exec_mode` path
required a sidecar:

```text
<exe>.artifact_kind
```

Without that metadata, `stage1_contract_exec_mode` treats the binary as
`unknown` and falls back to CLI-style `--emit-mir-json` execution, which is not
the Stage1 env contract for generated Stage1 CLI artifacts.

## Decision

Do not add sidecar inference to `tools/selfhost_exe_stageb.sh`. The existing
formal artifact lane already owns metadata:

```text
tools/selfhost/mainline/build_stage1.sh
```

This script writes `<out>.artifact_kind` after the build. After P380F, it can
reuse the corrected `selfhost_exe_stageb.sh` direct route and preserve the
existing artifact contract.

## Non-Goals

- no new metadata inference in low-level helpers
- no path-name special case for `stage1_cli_env.hako`
- no Stage0 route/classifier widening
- no C shim change

## Acceptance

```bash
NYASH_LLVM_SKIP_BUILD=1 \
HAKO_BACKEND_COMPILE_RECIPE=pure-first \
HAKO_BACKEND_COMPAT_REPLAY=none \
tools/selfhost/mainline/build_stage1.sh \
  --artifact-kind stage1-cli \
  --entry lang/src/runner/stage1_cli_env.hako \
  --out /tmp/p380g_build_stage1_cli_env.exe \
  --timeout-secs 240 \
  --force-rebuild

source tools/selfhost/lib/stage1_contract.sh
stage1_contract_exec_mode \
  /tmp/p380g_build_stage1_cli_env.exe \
  emit-mir \
  lang/src/runner/stage1_cli_env.hako \
  'static box Main { main() { return 1 + 2 } }'

bash tools/selfhost/mainline/stage1_mainline_smoke.sh \
  --bin /tmp/p380g_build_stage1_cli_env.exe \
  apps/tests/hello_simple_llvm.hako
```

Expected:

- build writes `/tmp/p380g_build_stage1_cli_env.exe.artifact_kind`
- `stage1_contract_exec_mode ... emit-mir` emits MIR JSON
- `stage1_mainline_smoke.sh` reports PASS

## Result

Confirmed.

```text
[stage1] done: /tmp/p380g_build_stage1_cli_env.exe
[stage1] stage1-cli capability: OK (stage0 bootstrap proof + runnable reduced artifact)
[stage1] metadata: /tmp/p380g_build_stage1_cli_env.exe.artifact_kind
```

The contract helper emitted MIR JSON for `return 1 + 2`, and the current
mainline smoke passed. P380J clarifies that `stage1_mainline_smoke.sh` is the
compat direct-emit smoke; the selected `--bin` is not itself treated as the
payload emitter by that wrapper.

```text
[stage1-mainline-smoke] PASS (p380g_build_stage1_cli_env.exe)
```
