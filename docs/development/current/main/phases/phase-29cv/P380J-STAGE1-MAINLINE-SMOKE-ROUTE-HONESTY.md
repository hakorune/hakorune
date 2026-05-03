---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P380J, Stage1 mainline smoke route-honesty cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P380G-STAGE1-BUILD-LANE-MAINLINE-SMOKE.md
  - docs/development/current/main/phases/phase-29cv/P380H-DEFAULT-STAGE1-CLI-MAINLINE-SMOKE.md
  - docs/development/current/main/phases/phase-29cv/P380I-STAGE3-SAME-RESULT-SEED-BOUNDARY.md
  - tools/selfhost/mainline/stage1_mainline_smoke.sh
  - tools/selfhost/compat/run_stage1_cli.sh
---

# P380J: Stage1 Mainline Smoke Route Honesty

## Problem

P380G/P380H confirmed the current `stage1_mainline_smoke.sh` is green, but the
smoke wording could be misread as proof that the selected `--bin` Stage1
artifact itself emits MIR payloads.

The actual current shell contract is narrower:

- `stage1_mainline_smoke.sh` checks that a Stage1 artifact exists and then calls
  `tools/selfhost/compat/run_stage1_cli.sh --bin <bin> emit mir-json <source>`
- `run_stage1_cli.sh` keeps this compatibility emit path on
  `target/release/hakorune` direct MIR emission
- reduced `stage1-cli` artifacts remain runnable bootstrap outputs, not payload
  emit seeds

P380I already locked the payload seed boundary. This card removes the ambiguous
wording from the smoke and docs without changing behavior.

## Decision

Keep `stage1_mainline_smoke.sh` as the historical daily smoke, but label it as a
compat direct-emit smoke. Full payload emission stays on:

```text
stage1_contract_exec_mode <full-stage1-cli-env-artifact> emit-mir ...
tools/selfhost/stage3_same_result_check.sh --seed-bin <full-stage1-cli-env-artifact>
```

## Non-Goals

- no behavior change to `run_stage1_cli.sh`
- no reduced artifact payload capability promotion
- no new helper
- no Stage0 route/classifier change

## Acceptance

```bash
bash tools/selfhost/mainline/stage1_mainline_smoke.sh
```

Expected: PASS message includes `route=compat-direct-emit`, making the route
truth visible in logs.

## Result

Implemented.

The smoke now prints the route explicitly:

```text
[stage1-mainline-smoke] emit mir-json route=compat-direct-emit entry=/home/tomoaki/git/hakorune-selfhost/apps/tests/hello_simple_llvm.hako
[stage1-mainline-smoke] PASS (hakorune.stage1_cli route=compat-direct-emit)
```

Guards:

```text
bash -n tools/selfhost/mainline/stage1_mainline_smoke.sh
bash tools/selfhost/mainline/stage1_mainline_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

All passed.
