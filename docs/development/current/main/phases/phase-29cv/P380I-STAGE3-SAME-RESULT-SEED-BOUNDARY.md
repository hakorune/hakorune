---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P380I, Stage3 same-result seed boundary
Related:
  - docs/development/current/main/phases/phase-29cv/P380H-DEFAULT-STAGE1-CLI-MAINLINE-SMOKE.md
  - tools/selfhost/stage3_same_result_check.sh
  - tools/selfhost/lib/stage1_contract.sh
---

# P380I: Stage3 Same-Result Seed Boundary

## Problem

After P380H, the default `target/selfhost/hakorune.stage1_cli` artifact builds
and the current mainline smoke passes. However, that default artifact is built
from the reduced run-only entry:

```text
lang/src/runner/entry/stage1_cli_env_entry.hako
```

The Stage3 same-result helper needs a payload-emitting seed because it
materializes Program(JSON) and MIR(JSON) snapshots twice. Using the reduced
artifact as the seed fails at MIR materialization:

```text
[Stage3:FAIL] failed to materialize MIR(JSON) via stage1 env route: target/selfhost/hakorune.stage1_cli
```

This is a seed-boundary issue, not a Stage0 route/classifier blocker.

## Decision

Keep the existing contract split:

- default `stage1-cli` artifact: runnable reduced bootstrap output
- full `stage1_cli_env.hako` artifact: payload-emitting seed for
  `stage3_same_result_check.sh`

Improve the same-result helper's fail-fast message when the caller accidentally
uses a reduced `stage1-cli` seed.

## Non-Goals

- no route/classifier widening
- no `stageb-delegate` promotion
- no change to reduced artifact liveness semantics
- no inferred payload capability from artifact kind alone

## Acceptance

Reduced seed should fail with a seed-boundary hint:

```bash
tools/selfhost/stage3_same_result_check.sh \
  --artifact-kind stage1-cli \
  --seed-bin target/selfhost/hakorune.stage1_cli
```

Full env seed should pass:

```bash
tools/selfhost/stage3_same_result_check.sh \
  --artifact-kind stage1-cli \
  --seed-bin /tmp/p380g_build_stage1_cli_env.exe \
  --stage2-bin /tmp/p380i_stage2 \
  --stage3-bin /tmp/p380i_stage3
```

## Result

Implemented.

The reduced default seed now fails with the intended seed-boundary message:

```text
[Stage3:FAIL] failed to materialize MIR(JSON) via stage1 env route: target/selfhost/hakorune.stage1_cli
              seed-boundary: reduced stage1-cli artifacts are runnable bootstrap outputs, not payload emit seeds
              hint: build a full stage1_cli_env.hako artifact and pass it with --seed-bin
```

The full env seed passes:

```text
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
