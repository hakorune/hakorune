---
Status: Landed
Date: 2026-04-26
Scope: JoinIR Case-A update-summary name-only inventory
Related:
  - src/mir/join_ir/lowering/loop_view_builder.rs
  - src/mir/join_ir/lowering/loop_update_summary.rs
  - src/mir/join_ir/lowering/loop_scope_shape/case_a_lowering_shape.rs
  - docs/development/current/main/phases/phase-291x/291x-318-joinir-simple-while-main-gate-helper-card.md
---

# 291x-319: JoinIR Case-A Update-summary Name-only Inventory

## Goal

Inventory the remaining Case-A shape-dispatch seam where carrier names are
converted into a synthetic update summary.

This card is audit-only. It does not change accepted Case-A targets or
lowering behavior.

## Findings

`LoopViewBuilder::build(...)` constructs Case-A shape input from carrier names:

```text
scope.carriers -> analyze_loop_updates_by_name(...) -> LoopFeatures.update_summary
```

`analyze_loop_updates_by_name(...)` classifies every carrier as
`AccumulationLike`. That makes the update summary look observed even though no
loop body / MIR update expression was inspected.

The deprecated `CaseALoweringShape::detect(scope)` wrapper has the same seam:

```text
LoopScopeShape carrier names -> synthetic AccumulationLike summary
```

This is BoxShape debt:

```text
name-only metadata is not a proof of update kind
```

## Decision

The next implementation target is:

```text
JoinIR Case-A update-summary name-only prune
```

Implementation boundary:

```text
No AST / no MIR update observation -> no update_summary
```

Observed update summaries remain owned by:

```text
analyze_loop_updates_from_ast(...)
```

Name-only callers must use the conservative Case-A feature path instead of
synthesizing `AccumulationLike` rows.

## Non-Goals

- No Case-A target expansion.
- No Case-A target deletion.
- No simple-while route change.
- No lowerer behavior change beyond removing synthetic update-summary proof.
- No broader carrier-count heuristic rewrite in this card.

## Acceptance

```bash
cargo test -q case_a_update_summary_name_only
cargo test -q test_is_loop_lowered_function
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
