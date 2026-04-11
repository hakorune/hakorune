# Phase 164x: repo-wide fmt drift cleanup

- Status: Parked
- Purpose: keep repo-wide formatting drift separate from the `phase-163x` optimization lane.
- Scope:
  - formatting-only cleanup for the repo-wide `cargo fmt --check` drift
  - no behavior changes
  - no refactors mixed into this corridor
- Current confirmed inventory:
  - the 11 files reported by the worker investigation in `164x-90`
- Decision Now:
  - do not mix this corridor with `phase-163x`
  - treat `src/mir/passes/escape.rs` as already excluded from the fmt drift set

## Restart Handoff

- root pointer:
  - `CURRENT_TASK.md`
- workstream map:
  - `docs/development/current/main/15-Workstream-Map.md`
- phase index:
  - `docs/development/current/main/phases/README.md`
- SSOT:
  - `docs/development/current/main/phases/phase-164x/164x-90-fmt-drift-cleanup-ssot.md`

## Next Step

- resume only when the repo-wide formatting cleanup lane is explicitly selected
- keep any formatting-only edits out of the optimization commits unless the user asks to merge them
