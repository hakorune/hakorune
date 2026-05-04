---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P380L, Stage3 build-seed SSOT sync
Related:
  - docs/development/current/main/phases/phase-29cv/P380K-STAGE3-SAME-RESULT-BUILD-SEED.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/stage2-selfhost-and-hako-alloc-ssot.md
  - docs/tools/script-index.md
---

# P380L: Stage3 Build-Seed SSOT Sync

## Problem

P380K added `tools/selfhost/stage3_same_result_check.sh --build-seed`, but the
durable bootstrap docs still described the Stage3 check as if a pre-existing
`target/selfhost/hakorune.stage1_cli.stage2` seed were the normal entry.

That wording can send future work back to the reduced-artifact/payload-seed
confusion fixed by P380I/P380K.

## Decision

Sync the durable docs:

- Stage3 build lane uses `--build-seed` for the full `stage1_cli_env.hako`
  payload seed.
- reduced `stage1-cli` remains a runnable bootstrap artifact, not a payload
  seed.
- script index names `stage1_mainline_smoke.sh` as compat direct-emit smoke.

## Non-Goals

- no code changes
- no helper behavior change
- no new route

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: docs point at `--build-seed` for Stage3 same-result build lane.

## Result

Implemented.

Synced:

- `selfhost-bootstrap-route-ssot.md` now points Stage3 build lane at
  `stage3_same_result_check.sh --build-seed`.
- `stage2-selfhost-and-hako-alloc-ssot.md` records the full
  `stage1_cli_env.hako` payload seed boundary.
- `docs/tools/script-index.md` labels `stage1_mainline_smoke.sh` as the compat
  direct-emit smoke and indexes `stage3_same_result_check.sh --build-seed`.
