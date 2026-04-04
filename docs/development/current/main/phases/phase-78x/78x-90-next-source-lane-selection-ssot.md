# Phase 78x SSOT

## Intent

`78x` selects the next source lane after `77x` proof/closeout.

## Facts to Keep Stable

- `77x` is proof-closed after `stage1_cli_env.hako` authority thinning.
- `stage1_mainline_smoke` is green.
- `stage1_cli_env` mainline MIR emission is green.
- `launcher.hako` focused probe remains a known residual blocker.
- The blocker is tracked, not treated as current lane owner.
- top-level runner wrappers are already thin:
  - `lang/src/runner/stage1_cli.hako`
  - `lang/src/runner/runner_facade.hako`
  - `lang/src/runner/launcher_native_entry.hako`
  - `lang/src/runner/stage1_cli_env_entry.hako`
- `CURRENT_TASK.md` and `10-Now.md` are still heavier than their stated thin-pointer role.

## Candidate Ranking

1. `phase-79x launcher emit_mir residual blocker follow-up`
   - target: `lang/src/runner/launcher.hako` focused `emit_mir_mainline` red
2. `phase-80x root/current pointer thinning`
   - target: `CURRENT_TASK.md`, `docs/development/current/main/10-Now.md`, `docs/development/current/main/05-Restart-Quick-Resume.md`
3. `phase-81x caller-zero archive rerun`
   - target: top-level aliases/wrappers after phases `67x-77x`

## Acceptance

1. The next lane is selected once.
2. The selected lane has a documented inventory and ranking.
3. The proof bundle remains green except for the known residual blocker.
4. Closeout hands off cleanly to the successor phase.
