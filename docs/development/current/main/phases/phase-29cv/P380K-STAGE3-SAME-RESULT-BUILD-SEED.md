---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P380K, Stage3 same-result full seed build option
Related:
  - docs/development/current/main/phases/phase-29cv/P380I-STAGE3-SAME-RESULT-SEED-BOUNDARY.md
  - docs/development/current/main/phases/phase-29cv/P380J-STAGE1-MAINLINE-SMOKE-ROUTE-HONESTY.md
  - tools/selfhost/stage3_same_result_check.sh
  - tools/selfhost/mainline/build_stage1.sh
---

# P380K: Stage3 Same-Result Build Seed

## Problem

P380I clarified that `stage3_same_result_check.sh` needs a payload-emitting
full `stage1_cli_env.hako` artifact as its seed. The current default seed path
still points at:

```text
target/selfhost/hakorune.stage1_cli.stage2
```

That path is a compare label/output, not a reliable full env payload seed. When
it does not exist, the default helper fails before reaching the actual
same-result check.

## Decision

Add an explicit `--build-seed` option to `stage3_same_result_check.sh`.

When enabled, the helper builds the full seed with the existing artifact owner:

```text
tools/selfhost/mainline/build_stage1.sh
  --artifact-kind stage1-cli
  --entry lang/src/runner/stage1_cli_env.hako
  --out target/selfhost/hakorune.stage1_cli_env_seed
```

This keeps the route SSOT in `build_stage1.sh` and avoids adding another
bootstrap compiler path.

## Non-Goals

- no implicit reduced artifact promotion
- no new build helper
- no route/classifier widening
- no C shim change
- no `stageb-delegate` behavior change

## Acceptance

```bash
NYASH_LLVM_SKIP_BUILD=1 \
HAKO_BACKEND_COMPILE_RECIPE=pure-first \
HAKO_BACKEND_COMPAT_REPLAY=none \
tools/selfhost/stage3_same_result_check.sh \
  --artifact-kind stage1-cli \
  --build-seed \
  --stage2-bin /tmp/p380k_stage2 \
  --stage3-bin /tmp/p380k_stage3

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: seed build succeeds, Program(JSON), MIR(JSON), and metadata snapshots
match.

## Result

Implemented.

`stage3_same_result_check.sh` now accepts `--build-seed` and materializes the
full payload seed through the existing `build_stage1.sh` owner.

Verification:

```text
[Stage3] Building payload seed artifact: /home/tomoaki/git/hakorune-selfhost/target/selfhost/hakorune.stage1_cli_env_seed
[stage1] done: /home/tomoaki/git/hakorune-selfhost/target/selfhost/hakorune.stage1_cli_env_seed
[stage1] stage1-cli capability: OK (stage0 bootstrap proof + runnable reduced artifact)
[stage1] metadata: /home/tomoaki/git/hakorune-selfhost/target/selfhost/hakorune.stage1_cli_env_seed.artifact_kind
[Stage3] Using payload seed artifact (stage1-cli): /home/tomoaki/git/hakorune-selfhost/target/selfhost/hakorune.stage1_cli_env_seed
[Stage3] Re-emitting Stage2 payload snapshots...
[Stage3] Re-emitting Stage3 payload snapshots...
[Stage3] program-json: MATCH
[Stage3] mir-json: MATCH
[Stage3] metadata: MATCH
[Stage3] same-result: PASS (stage1-cli)
```

Guards:

```text
bash -n tools/selfhost/stage3_same_result_check.sh
```

Passed.
