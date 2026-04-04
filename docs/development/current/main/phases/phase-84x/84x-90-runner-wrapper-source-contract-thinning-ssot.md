# Phase 84x SSOT

## Intent

`84x` thins the remaining top-level `.hako` wrapper/source pressure after `83x` froze top-level selfhost shell wrappers as explicit front-door keeps.

## Facts to Keep Stable

- `83x` closed as an explicit keep proof; no top-level selfhost wrapper moved to archive.
- top-level `.hako` wrappers remain in scope:
  - `lang/src/runner/stage1_cli.hako`
  - `lang/src/runner/stage1_cli_env_entry.hako`
  - `lang/src/runner/runner_facade.hako`
  - `lang/src/runner/launcher_native_entry.hako`
- canonical homes already exist under:
  - `lang/src/runner/compat/`
  - `lang/src/runner/entry/`
  - `lang/src/runner/facade/`
- `launcher.hako emit_mir_mainline` is green.
- `stage1_mainline_smoke.sh` is green.

## Initial Focus

1. confirm which top-level `.hako` wrappers are still required as public interface stubs
2. confirm whether embedded snapshot / Stage1 build contracts still pin any top-level paths
3. thin comments/contracts or move caller-zero residue only when the interface stop-line stays explicit

## Acceptance

1. wrapper/source inventory is source-backed
2. target split is frozen before edits
3. green proof bundle remains intact after thinning
