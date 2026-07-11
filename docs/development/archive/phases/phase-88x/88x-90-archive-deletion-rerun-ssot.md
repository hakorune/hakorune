# Phase 88x SSOT

## Intent

`88x` reruns archive/delete-ready inventory after `87x` refreshed the embedded snapshot and narrowed one repo-internal wrapper caller.

## Facts to Keep Stable

- `87x` refreshed:
  - `selfhost.runner.Runner -> lang/src/runner/facade/runner_facade.hako`
  - `selfhost.runner.entry.launcher_native_entry -> lang/src/runner/entry/launcher_native_entry.hako`
  - `selfhost.runner.entry.stage1_cli_env_entry -> lang/src/runner/entry/stage1_cli_env_entry.hako`
- top-level `.hako` wrappers still remain explicit keep surfaces unless caller-zero and public-surface evidence both say otherwise.
- top-level selfhost shell wrappers remain explicit public/front-door keeps.
- `launcher.hako emit_mir_mainline` and `stage1_mainline_smoke.sh` are green.

## Initial Focus

1. rerun caller inventory for wrapper aliases after snapshot refresh
2. classify true archive-ready/delete-ready surfaces vs explicit keep surfaces
3. only sweep focused residue if classification is source-backed

## Inventory Freeze

- `keep-now`
  - `lang/src/runner/stage1_cli.hako`
    - still has live source/tool/test callers
  - `lang/src/runner/runner_facade.hako`
  - `lang/src/runner/launcher_native_entry.hako`
  - `lang/src/runner/stage1_cli_env_entry.hako`
    - repo-internal source/tools callers are now effectively gone, but prior stop-line still treats these as explicit public keep surfaces
  - top-level selfhost shell wrappers
    - still treated as explicit public/front-door keeps
- `archive-ready`
  - none
- `delete-ready`
  - none

## Rerun Result

- `87x` did remove snapshot-pinned old runner paths and one repo-internal smoke caller
- that did **not** create a safe delete/archive candidate in current source
- result: `88x` is a no-op archive/deletion sweep

## Acceptance

1. archive/delete-ready inventory is source-backed
2. focused sweep is narrow or explicitly no-op
3. proof keeps mainline green
