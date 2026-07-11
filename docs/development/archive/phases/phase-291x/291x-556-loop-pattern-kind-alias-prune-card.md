---
Status: Landed
Date: 2026-04-28
Scope: Remove the unused LoopPatternKind legacy alias from live route detector code
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/tools/check-scripts-index.md
  - src/mir/loop_route_detection/kind.rs
  - tools/checks/route_detector_legacy_surface_guard.sh
---

# 291x-556: LoopPatternKind Alias Prune

## Goal

Remove the traceability-era `LoopPatternKind` alias from live route detector
code.

The current route detector vocabulary is `LoopRouteKind`. Historical/private
docs may still mention `LoopPatternKind`, but live Rust code should not keep a
dead alias just for old naming.

## Inventory

Removed live alias:

- `src/mir/loop_route_detection/kind.rs`
  - `pub type LoopPatternKind = LoopRouteKind`

Guarded regrowth:

- `tools/checks/route_detector_legacy_surface_guard.sh`

Historical docs intentionally left unchanged:

- `docs/private/**`
- `docs/archive/**`
- investigation archives under `docs/development/current/main/investigations/`

## Cleaner Boundary

```text
loop_route_detection
  current vocabulary: LoopRouteKind

historical docs
  may retain LoopPatternKind as archive wording

route_detector_legacy_surface_guard
  rejects LoopPatternKind in live code/tests
```

## Boundaries

- BoxShape-only.
- Do not change route classification behavior.
- Do not rename `LoopRouteKind`.
- Do not rewrite private/archive documentation.
- Do not reopen route detector physical owner migration.

## Acceptance

- No `LoopPatternKind` remains in `src` or `tests`.
- `bash tools/checks/route_detector_legacy_surface_guard.sh` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Removed the unused `LoopPatternKind` alias.
- Extended the route detector legacy surface guard to reject alias regrowth in
  live Rust code.
- Updated the check script index to describe the new guard coverage.

## Verification

```bash
rg -n "\\bLoopPatternKind\\b" src tests -g'*.rs'
bash tools/checks/route_detector_legacy_surface_guard.sh
bash tools/checks/current_state_pointer_guard.sh
cargo check -q
cargo fmt -- --check
git diff --check
```
