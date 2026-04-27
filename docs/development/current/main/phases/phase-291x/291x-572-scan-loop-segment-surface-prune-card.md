---
Status: Landed
Date: 2026-04-28
Scope: prune unused scan-loop segment plan-side compatibility surface
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/recipes/scan_loop_segments.rs
  - src/mir/builder/control_flow/plan/mod.rs
---

# 291x-572: Scan-Loop Segment Surface Prune

## Goal

Remove the unused `plan/scan_loop_segments.rs` compatibility shelf now that
callers import the recipes-owned vocabulary directly.

This is a BoxShape cleanup only. The scan-loop segment vocabulary owner remains
`recipes/scan_loop_segments.rs`; no recipe, planner, or lowering behavior
changes.

## Evidence

Before the prune, all live scan-loop segment users referenced the owner path:

```text
crate::mir::builder::control_flow::recipes::scan_loop_segments
```

The only plan-side hits were the module declaration and the re-export shelf.

## Cleaner Boundary

```text
recipes/scan_loop_segments.rs
  owns scan-loop segment vocabulary

plan/
  consumes the recipes owner path directly where needed
  does not mirror the vocabulary through a compatibility shelf
```

## Boundaries

- Delete only the unused plan-side compatibility module and declaration.
- Do not move or edit the recipes-owned vocabulary.
- Do not change scan-loop route acceptance or lowering behavior.

## Acceptance

- No `plan::scan_loop_segments` users remain.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo check --release --bin hakorune` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Removed `src/mir/builder/control_flow/plan/scan_loop_segments.rs`.
- Removed the stale module declaration from `plan/mod.rs`.

## Verification

```bash
rg -n "plan::scan_loop_segments|control_flow::plan::scan_loop_segments|super::scan_loop_segments" src tests -g'*.rs'
bash tools/checks/current_state_pointer_guard.sh
cargo check --release --bin hakorune
cargo fmt -- --check
git diff --check
```
