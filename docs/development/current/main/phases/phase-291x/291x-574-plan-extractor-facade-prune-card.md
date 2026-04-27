---
Status: Landed
Date: 2026-04-28
Scope: prune unused plan-side extractor compatibility facades
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/facts/ast_feature_extractor.rs
  - src/mir/builder/control_flow/plan/extractors/mod.rs
  - src/mir/builder/control_flow/plan/mod.rs
---

# 291x-574: Plan Extractor Facade Prune

## Goal

Remove unused plan-side compatibility facades for facts-owned extractors.

This cleanup keeps `facts/` as the owner for descriptive extractor logic and
keeps `plan/` from publishing route-specific mirror modules.

## Evidence

Before the prune:

- `plan/ast_feature_extractor.rs` had one remaining caller.
- `plan/escape_shape_recognizer.rs` had no route callers.
- `plan/extractors/if_phi_join.rs` had no route callers.
- `plan/extractors/loop_simple_while.rs` had no route callers.

The one remaining AST feature caller now imports the facts owner directly:

```text
crate::mir::builder::control_flow::facts::ast_feature_extractor
```

## Cleaner Boundary

```text
facts/ast_feature_extractor.rs
  owns AST route feature extraction

facts/extractors/*
  owns route-specific descriptive extractors

plan/extractors/common_helpers
  remains as the only plan compatibility wrapper for shared helper migration
```

## Boundaries

- Migrate only the remaining AST feature import to the facts owner.
- Delete unused plan-side route extractor facades.
- Do not change extractor implementation or route acceptance.
- Do not remove `plan/extractors/common_helpers`; it still has live callers.

## Acceptance

- No deleted facade users remain.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo check --release --bin hakorune` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Moved `loop_cond/break_continue_entry.rs` to the facts AST feature owner path.
- Removed unused `plan/ast_feature_extractor.rs`.
- Removed unused `plan/escape_shape_recognizer.rs`.
- Removed unused `plan/extractors/if_phi_join.rs`.
- Removed unused `plan/extractors/loop_simple_while.rs`.
- Updated `plan/extractors/mod.rs` to document that route-specific facades
  should not be regrown.

## Verification

```bash
rg -n "plan::ast_feature_extractor|control_flow::plan::ast_feature_extractor|plan::escape_shape_recognizer|control_flow::plan::escape_shape_recognizer|plan::extractors::if_phi_join|plan::extractors::loop_simple_while" src tests -g'*.rs'
bash tools/checks/current_state_pointer_guard.sh
cargo check --release --bin hakorune
cargo fmt -- --check
git diff --check
```
