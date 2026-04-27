---
Status: Landed
Date: 2026-04-28
Scope: move plan recipe callers to recipes owner and prune plan-side compatibility surface
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/recipes/mod.rs
  - src/mir/builder/control_flow/recipes/refs.rs
  - src/mir/builder/control_flow/plan/mod.rs
---

# 291x-573: Plan Recipes Surface Prune

## Goal

Remove the `plan/recipes` compatibility surface after moving remaining plan-side
callers to the recipes owner path.

This is a BoxShape cleanup. Recipe vocabulary remains owned by
`control_flow/recipes`; plan-side code consumes that owner directly instead of
mirroring the names through `plan/recipes`.

## Evidence

Before the prune, remaining users were local plan imports such as:

```text
crate::mir::builder::control_flow::plan::recipes::RecipeBody
crate::mir::builder::control_flow::plan::recipes::refs::StmtRef
```

After migrating those imports, this query has no matches:

```bash
rg -n "control_flow::plan::recipes|plan::recipes|super::recipes|crate::mir::builder::control_flow::plan::recipes|use .*plan::recipes" src/mir/builder/control_flow src tests -g'*.rs'
```

## Cleaner Boundary

```text
control_flow/recipes/
  owns RecipeBody, StmtIdx, StmtRange, and refs vocabulary

control_flow/plan/
  consumes recipe vocabulary by owner path
  does not publish a recipe compatibility shelf
```

## Boundaries

- Update import paths only.
- Delete `plan/recipes/mod.rs` and `plan/recipes/refs.rs`.
- Do not change recipe data structures or lowering behavior.
- Do not change route acceptance.

## Acceptance

- No `plan::recipes` users remain.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo check --release --bin hakorune` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Moved remaining plan-side imports to `control_flow::recipes`.
- Removed the `plan/recipes` module declaration and compatibility files.

## Verification

```bash
rg -n "control_flow::plan::recipes|plan::recipes|super::recipes|crate::mir::builder::control_flow::plan::recipes|use .*plan::recipes" src/mir/builder/control_flow src tests -g'*.rs'
bash tools/checks/current_state_pointer_guard.sh
cargo check --release --bin hakorune
cargo fmt -- --check
git diff --check
```
