---
Status: Landed
Date: 2026-04-26
Scope: JoinIR Case-A update-summary name-only prune
Related:
  - src/mir/join_ir/lowering/loop_view_builder.rs
  - src/mir/join_ir/lowering/loop_update_summary.rs
  - src/mir/join_ir/lowering/loop_scope_shape/case_a_lowering_shape.rs
  - src/mir/loop_route_detection/features.rs
  - docs/development/current/main/phases/phase-291x/291x-319-joinir-casea-update-summary-name-only-inventory-card.md
---

# 291x-320: JoinIR Case-A Update-summary Name-only Prune

## Goal

Stop treating carrier names as observed loop-update metadata.

This is BoxShape cleanup: it removes a synthetic proof source without expanding
Case-A acceptance.

## Change

Updated `LoopViewBuilder::build(...)`:

```text
No AST/MIR update observation -> LoopFeatures.update_summary = None
```

Updated the deprecated `CaseALoweringShape::detect(scope)` wrapper the same
way.

Updated `loop_route_detection::extract_features(...)` to leave
`update_summary` absent when only `LoopScopeShape` carrier names are available.

Removed:

```text
analyze_loop_updates_by_name(...)
```

`analyze_loop_updates_from_ast(...)` remains the owner for observed update
summaries.

## Preserved Behavior

- simple-while route selection is unchanged.
- Case-A target descriptor fallback is unchanged.
- Observed AST update summaries still classify carrier updates.
- Carrier-count fallback remains separate debt and is not rewritten in this
  card.

## Non-Goals

- No Case-A target expansion.
- No Case-A target deletion.
- No broader carrier-count heuristic cleanup.
- No lowerer rewrite.
- No LoopScopeShape field change.

## Validation

```bash
cargo test -q case_a_update_summary_name_only
cargo test -q test_is_loop_lowered_function
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
