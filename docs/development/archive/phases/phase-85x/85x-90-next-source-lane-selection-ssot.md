# Phase 85x SSOT

## Intent

`85x` selects the next source lane after `84x` landed with Stage1 build/default contracts repointed to canonical `entry/*` stubs.

## Facts to Keep Stable

- `84x` landed without removing top-level `.hako` wrappers.
- top-level `.hako` wrappers remain explicit keep surfaces:
  - `lang/src/runner/stage1_cli.hako`
  - `lang/src/runner/stage1_cli_env_entry.hako`
  - `lang/src/runner/runner_facade.hako`
  - `lang/src/runner/launcher_native_entry.hako`
- `tools/selfhost/mainline/build_stage1.sh` and `tools/selfhost/README.md` now point at canonical `lang/src/runner/entry/*` paths.
- `launcher.hako emit_mir_mainline` is green.
- `stage1_mainline_smoke.sh` is green.
- root/current mirrors were intentionally thinned in `80x` and should stay thin.

## Candidate Ranking

1. `phase-86x phase index / current mirror hygiene`
   - target: `docs/development/current/main/phases/README.md` and remaining heavy mirror surfaces
   - question: thin registry/current mirrors again now that `84x` landed
2. `phase-87x embedded snapshot / wrapper repoint rerun`
   - target: `src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json` and wrapper/source pressure deferred by `84x`
   - question: can snapshot-coupled path pressure be narrowed without widening public stop-lines
3. `phase-88x archive/deletion rerun`
   - target: archive-ready surfaces after the latest wrapper/source thinning
   - question: did `84x` create any true archive-ready residue

## Acceptance

1. the next lane is selected once
2. the selected lane is ranked against at least two alternatives
3. closeout hands off cleanly to the chosen successor
