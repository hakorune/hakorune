---
Status: Landed
Date: 2026-04-29
Scope: planner cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/planner/mod.rs
  - src/mir/builder/control_flow/plan/facts/loop_tests_parts/planner_ctx.rs
---

# 291x-693: Planner Dead Helper Prune

## Why

`cargo build --release` still reported a tiny planner shelf that had already
become historical: trivial accessor helpers and a standalone validator module
with no owner-path callers. The active planner path already reads canonical
facts directly from `PlanBuildOutcome`, so keeping those shelves only preserved
dead warning surface.

## Changes

- removed the dead planner helper module
  `plan/planner/helpers.rs`
  - `infer_skeleton_kind`
  - `infer_exit_usage`
- removed the dead planner validator module
  `plan/planner/validators.rs`
  - `debug_assert_cleanup_kinds_match_exit_kinds`
- trimmed `plan/planner/mod.rs` so it no longer exports those dead shelves

## Result

- `cargo build --release` warning count moved from **44** to **41**
- the planner module now contains only active owner-path surfaces

## Proof

```bash
cargo build --release
cargo test --release loopfacts_ctx_ -- --nocapture
```
