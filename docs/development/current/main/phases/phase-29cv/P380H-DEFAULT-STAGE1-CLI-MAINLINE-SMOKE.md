---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P380H, default target/selfhost stage1-cli smoke confirmation
Related:
  - docs/development/current/main/phases/phase-29cv/P380G-STAGE1-BUILD-LANE-MAINLINE-SMOKE.md
  - tools/selfhost/mainline/build_stage1.sh
  - tools/selfhost/mainline/stage1_mainline_smoke.sh
---

# P380H: Default Stage1 CLI Mainline Smoke

## Problem

P380G confirmed the formal build lane with a `/tmp` output path. The daily
mainline entry uses the default artifact:

```text
target/selfhost/hakorune.stage1_cli
```

That default path should also be green after P380F, without requiring manual
sidecar creation or a custom `--bin` argument.

## Decision

Use the existing mainline build script and smoke. Do not add new helpers.

## Non-Goals

- no code changes
- no new route/classifier shape
- no bridge capsule promotion
- no `stageb-delegate` behavior change

## Acceptance

```bash
NYASH_LLVM_SKIP_BUILD=1 \
HAKO_BACKEND_COMPILE_RECIPE=pure-first \
HAKO_BACKEND_COMPAT_REPLAY=none \
tools/selfhost/mainline/build_stage1.sh \
  --artifact-kind stage1-cli \
  --timeout-secs 240 \
  --force-rebuild

bash tools/selfhost/mainline/stage1_mainline_smoke.sh
```

Expected:

- default artifact is written to `target/selfhost/hakorune.stage1_cli`
- sidecar is written to `target/selfhost/hakorune.stage1_cli.artifact_kind`
- smoke reports `PASS (hakorune.stage1_cli)`

## Result

Confirmed.

```text
[stage1] done: /home/tomoaki/git/hakorune-selfhost/target/selfhost/hakorune.stage1_cli
[stage1] stage1-cli capability: OK (stage0 bootstrap proof + runnable reduced artifact)
[stage1] metadata: /home/tomoaki/git/hakorune-selfhost/target/selfhost/hakorune.stage1_cli.artifact_kind
[stage1-mainline-smoke] PASS (hakorune.stage1_cli)
```

P380J clarifies this smoke as the compat direct-emit smoke. It should not be
read as proof that the reduced default artifact emits MIR payloads itself.
