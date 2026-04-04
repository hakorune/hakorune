# Phase 90x SSOT

## Intent

`90x` thins current/design doc surfaces after `89x` selected doc/design stale surface hygiene as the next structural lane.

## Facts to Keep Stable

- `87x` already repointed embedded snapshot paths to canonical `facade/*` and `entry/*`.
- `88x` already reran archive/delete-ready inventory and still found:
  - `archive-ready`: none
  - `delete-ready`: none
- `89x` ranked successors and selected current/design stale surface hygiene ahead of:
  - `phase-91x top-level .hako wrapper policy review`
  - `phase-92x selfhost proof/compat caller rerun`
- top-level selfhost shell wrappers remain explicit public/front-door keeps.
- top-level `.hako` wrappers remain explicit keep surfaces unless a later policy lane changes that stop-line.

## Target Split

1. current mirrors
   - `CURRENT_TASK.md`
   - `05-Restart-Quick-Resume.md`
   - `10-Now.md`
   - `15-Workstream-Map.md`
2. current/design docs that still overstate stale wrapper/current surfaces
   - `docs/development/runtime/cli-hakorune-stage1.md`
   - `docs/development/selfhosting/quickstart.md`
   - `docs/development/architecture/mir-logs-observability.md`
   - `docs/reference/environment-variables.md`
   - `docs/development/current/main/design/selfhost-compiler-structure-ssot.md`
3. phase-local references that should point at canonical `facade/*`, `entry/*`, `mainline/*`, `proof/*`, or `compat/*` surfaces

## Inventory Lock

- included in-lane:
  - current mirrors listed above
  - current/design docs that still name top-level wrapper surfaces as if they were primary owners
- explicitly out of lane:
  - `docs/archive/**`
  - historical `phase-*` records outside current pointer needs
  - source moves, archive sweeps, and caller-zero decisions

## Stop Lines

- do not reopen archive/delete decisions in this lane
- do not move source files in this lane
- keep root/current mirrors short; push detail into phase-local docs when needed

## Acceptance

1. stale current/design wording is reduced without widening root pointers
2. current mirrors stay thin and consistent with the active lane
3. closeout leaves `phase-91x` and `phase-92x` as the next ranked corridors
