# Phase 89x SSOT

## Intent

`89x` selects the next source lane after `88x` confirmed archive/deletion is still a no-op.

## Facts to Keep Stable

- `87x` refreshed embedded snapshot paths to canonical `facade/*` and `entry/*`.
- `88x` reran archive/delete-ready inventory and still found:
  - `archive-ready`: none
  - `delete-ready`: none
- top-level selfhost shell wrappers remain explicit public/front-door keeps.
- top-level `.hako` wrappers remain explicit keep surfaces unless a later policy lane changes that stop-line.

## Candidate Ranking

1. `phase-90x current-doc/design stale surface hygiene`
   - target: current/design docs that still describe old wrapper/current surfaces too noisily
2. `phase-91x top-level .hako wrapper policy review`
   - target: explicit keep policy around `stage1_cli.hako`, `runner_facade.hako`, `launcher_native_entry.hako`, `stage1_cli_env_entry.hako`
3. `phase-92x selfhost proof/compat caller rerun`
   - target: proof/compat callers after latest canonical repoints

## Acceptance

1. the next lane is selected once
2. the selected lane is ranked against at least two alternatives
3. closeout hands off cleanly to the chosen successor
