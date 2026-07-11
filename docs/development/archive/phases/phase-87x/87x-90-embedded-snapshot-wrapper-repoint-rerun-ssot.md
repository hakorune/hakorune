# Phase 87x SSOT

## Intent

`87x` reruns the embedded snapshot / wrapper repoint seam that `84x` deliberately deferred.

## Facts to Keep Stable

- `84x` repointed build/default contracts to canonical `lang/src/runner/entry/*`.
- `84x` explicitly deferred snapshot-coupled path pressure:
  - `src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json`
- top-level `.hako` wrappers remain explicit keep surfaces unless fresh caller evidence says otherwise.
- `86x` already reduced mirror/index pressure; this lane is source-facing again.

## Initial Focus

1. inventory snapshot-pinned top-level wrapper paths
2. separate repointable consumers from keep-now/public surfaces
3. keep stop-lines explicit before touching snapshot-coupled contracts

## Inventory Freeze

- `thin-now`
  - `src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json`
    - stale top-level runner wrapper paths were still embedded
  - `tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh`
    - still referenced top-level `lang/src/runner/launcher_native_entry.hako`
- `keep-now`
  - `lang/src/runner/stage1_cli.hako`
    - still used as a public compat facade path by tests/tools
  - top-level `runner_facade.hako`, `launcher_native_entry.hako`, `stage1_cli_env_entry.hako`
    - remain explicit keep surfaces even after the snapshot repoint

## First Cut Result

- refreshed `embedded_stage1_modules_snapshot.json`
  - `selfhost.runner.Runner` now points to `lang/src/runner/facade/runner_facade.hako`
  - `selfhost.runner.entry.launcher_native_entry` now points to `lang/src/runner/entry/launcher_native_entry.hako`
  - `selfhost.runner.entry.stage1_cli_env_entry` now points to `lang/src/runner/entry/stage1_cli_env_entry.hako`
- narrowed one repo-internal smoke caller to canonical `entry/launcher_native_entry.hako`

## Acceptance

1. snapshot/wrapper inventory is source-backed
2. any repoint is narrow and does not remove public keep surfaces by accident
3. proof keeps Stage1/mainline green
